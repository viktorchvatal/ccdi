mod logger;
mod server;

use std::convert::Infallible;
use std::sync::Arc;
use std::fmt::Debug;
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

    let (server_tx, mut server_rx) = mpsc::channel::<WebServerMessage>(10);
    // let server_tx = Arc::new(server_tx);

    tokio::spawn(async move {
        while let Some(message) = server_rx.recv().await {
            log_err(
                "Sending message to server worker thread",
                sync_server_tx.send(message).map_err(to_string)
            );
        }
    });

    let websocket_service = warp::path("ccdi")
        .and(warp::ws())
        .and(with_server_tx(server_tx.clone()))
        .and_then(ws_handler);

    let index = warp::path::end().map(|| warp::reply::html(INDEX));

    let routes = warp::get().and(websocket_service.or(index));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080)).await;
}

fn with_server_tx(
    server_tx: Sender<WebServerMessage>
) -> impl Filter<Extract = (Sender<WebServerMessage>,), Error = Infallible> + Clone {
    warp::any().map(move || server_tx.clone())
}

pub async fn ws_handler(
    ws: warp::ws::Ws,
    server_tx: Sender<WebServerMessage>
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_client_connection(socket, server_tx)))
}

pub async fn handle_client_connection(
    websocket: WebSocket,
    server_tx: Sender<WebServerMessage>
) {
    ::log::info!("Websocket client connected");
    let (tx, mut rx) = websocket.split();

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
}

async fn process_message(
    message: Message,
    server_tx: &Sender<WebServerMessage>,
) -> Result<(), String> {
    server_tx.send(convert_server_message(message)?).await.map_err(to_string)
}

fn convert_server_message(message: Message) -> Result<WebServerMessage, String> {
    if message.is_text() {
        String::from_utf8(message.into_bytes()).map_err(|err| format!("{:?}", err))
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