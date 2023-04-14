mod logger;
mod server;
mod websocket;

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::fmt::Debug;
use futures::stream::SplitSink;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use logger::init_logger;
use server::start_server_thread;
use warp::Filter;
use futures::StreamExt;
use warp::Rejection;
use warp::Reply;
use warp::ws::Message;
use warp::ws::WebSocket;
use warp::ws::Ws;

const INDEX: &str = include_str!("static/index.html");

type WebClientMessage = String;
type WebServerMessage = String;

fn main() {

    let (server_tx, server_rx) = std::sync::mpsc::channel::<WebServerMessage>();
    let server_thread = start_server_thread(server_rx);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main(server_tx))
}

async fn tokio_main(sync_server_tx: std::sync::mpsc::Sender<WebServerMessage>) {
    init_logger();

    let (ws_from_client_tx, mut ws_from_client_rx) = mpsc::channel::<WsMessageFromClient>(10);
    // let server_tx = Arc::new(server_tx);

    let clients = Arc::new(RwLock::new(ClientSharedState{
        counter: 0,
        server_tx: ws_from_client_tx,
        transmitters: HashMap::new()
    }));

    tokio::spawn(async move {
        bridge_async_client_channels_to_mspc_channels(
            ws_from_client_rx, sync_server_tx
        ).await;
    });

    let websocket_service = warp::path("ccdi")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    let index = warp::path::end().map(|| warp::reply::html(INDEX));

    let routes = warp::get().and(websocket_service.or(index));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080)).await;
}

async fn bridge_async_client_channels_to_mspc_channels(
    mut async_server_rx: mpsc::Receiver<WsMessageFromClient>,
    sync_server_tx: std::sync::mpsc::Sender<String>
) {
    while let Some(message) = async_server_rx.recv().await {
        match message {
            WsMessageFromClient::TextMessage(text) => log_err(
                "Sending message to server worker thread",
                sync_server_tx.send(text).map_err(to_string)
            )
        }
    }
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

pub async fn ws_handler(ws: Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_client_connection(socket, clients)))
}

pub async fn handle_client_connection(
    websocket: WebSocket,
    clients: Clients
) {
    let server_tx = clients.read().await.server_tx.clone();
    let (tx, mut rx) = websocket.split();
    let id = clients.write().await.register_client(tx);

    while let Some(result) = rx.next().await {
        match result {
            Ok(message) => log_err(
                "Processing server message from client",
                process_message(message, &server_tx).await
            ),
            Err(e) => {
                eprintln!("error receiving ws message: {}", e);
                break;
            }
        };
    }
    log_err("Unregister client", clients.write().await.unregister_client(id));
}

async fn process_message(
    message: Message,
    server_tx: &Sender<WsMessageFromClient>,
) -> Result<(), String> {
    server_tx.send(convert_server_message(message)?).await.map_err(to_string)
}

fn convert_server_message(message: Message) -> Result<WsMessageFromClient, String> {
    if message.is_text() {
        String::from_utf8(message.into_bytes())
            .map(|text| WsMessageFromClient::TextMessage(text))
            .map_err(|err| format!("{:?}", err))
    } else {
        Err(String::from("Msg is not text"))
    }
}

fn to_string<T: Debug>(item: T) -> String {
    format!("{:?}", item)
}

fn log_err(label: &str, result: Result<(), String>) {
    if let Err(error) = result {
        eprintln!("Error in '{}': {}", label, error)
    }
}

pub type Clients = Arc<RwLock<ClientSharedState>>;

pub struct ClientSharedState {
    counter: usize,
    server_tx: Sender<WsMessageFromClient>,
    transmitters: HashMap<usize, SplitSink<WebSocket, Message>>
}

impl ClientSharedState {
    fn register_client(&mut self, transmitter: SplitSink<WebSocket, Message>) -> usize {
        let id = self.counter;
        self.counter += 1;
        self.transmitters.insert(id, transmitter);
        ::log::info!("Client {} registered ({} clients total)", id, self.transmitters.len());
        println!("{:?}", self.transmitters.keys().map(|&i| i).collect::<Vec<usize>>());
        id
    }

    fn unregister_client(&mut self, id: usize) -> Result<(), String> {
        self.transmitters.remove(&id);
        ::log::info!("Client {} unregistered ({} clients total)", id, self.transmitters.len());
        Ok(())
    }
}

#[derive(Debug)]
pub enum WsMessageFromClient {
    TextMessage(String)
}