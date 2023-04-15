use log::*;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

use futures::{StreamExt, FutureExt};
use warp::{Error, Reply, Rejection, Filter};
use warp::ws::{Message, WebSocket, Ws};

use crate::log_err;
use crate::to_string;

pub type Clients = Arc<RwLock<ClientSharedState>>;

pub struct ClientSharedState {
    counter: usize,
    server_tx: UnboundedSender<String>,
    transmitters: HashMap<usize, UnboundedSender<Result<Message, Error>>>
}

pub fn start_single_async_to_multiple_clients_sender(
    clients: Clients,
    mut async_clients_rx: tokio::sync::mpsc::UnboundedReceiver<String>
) {
    tokio::spawn(async move {
        loop {
            if let Some(message) = async_clients_rx.recv().await {
                for transmitter in clients.read().await.transmitters.values() {
                    log_err(
                        "Send message to client channel",
                        transmitter.send(Ok(Message::text(message.clone()))).map_err(to_string)
                    );
                }
            }
        }
    });
}

pub fn create_clients(ws_from_client_tx: UnboundedSender<String>) -> Clients {
    Arc::new(RwLock::new(ClientSharedState::new(ws_from_client_tx)))
}

pub fn create_websocket_service(
    path: &str, clients: Clients
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone + '_ {
    warp::path(path)
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler)
}

async fn ws_handler(ws: Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_client_connection(socket, clients)))
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn handle_client_connection(
    websocket: WebSocket,
    clients: Clients
) {
    let server_tx = clients.read().await.server_tx.clone();
    let (ws_tx, mut ws_rx) = websocket.split();

    let (client_sender, client_rcv) = mpsc::unbounded_channel::<Result<Message, Error>>();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    let id = clients.write().await.register_client(client_sender);

    while let Some(result) = ws_rx.next().await {
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
    server_tx: &UnboundedSender<String>,
) -> Result<(), String> {
    server_tx.send(convert_server_message(message)?).map_err(to_string)
}

fn convert_server_message(message: Message) -> Result<String, String> {
    if message.is_text() {
        String::from_utf8(message.into_bytes())
            .map_err(|err| format!("{:?}", err))
    } else {
        Err(String::from("Msg is not text"))
    }
}

impl ClientSharedState {
    fn new(server_tx: UnboundedSender<String>) -> Self {
        Self {
            counter: 0,
            server_tx,
            transmitters: HashMap::new()
        }
    }

    fn register_client(&mut self, transmitter: UnboundedSender<Result<Message, Error>>) -> usize {
        let id = self.counter;
        self.counter += 1;
        self.transmitters.insert(id, transmitter);
        info!("Client {} registered ({} clients total)", id, self.transmitters.len());
        id
    }

    fn unregister_client(&mut self, id: usize) -> Result<(), String> {
        self.transmitters.remove(&id);
        info!("Client {} unregistered ({} clients total)", id, self.transmitters.len());
        Ok(())
    }
}