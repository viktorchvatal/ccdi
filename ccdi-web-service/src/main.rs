mod logger;
mod server;
mod websocket;

use std::convert::Infallible;
use std::sync::Arc;
use std::fmt::Debug;
use tokio::sync::RwLock;
use tokio::sync::mpsc;

use logger::init_logger;
use server::start_server_thread;
use warp::Filter;
use warp::Rejection;
use warp::Reply;
use warp::ws::Ws;
use websocket::ClientSharedState;
use websocket::Clients;
use websocket::handle_client_connection;
use websocket::start_async_to_sync_channels_thread;
use websocket::start_single_async_to_multiple_clients_sender;
use websocket::start_sync_to_async_clients_sender;

const INDEX: &str = include_str!("static/index.html");

fn main() {

    let (server_tx, server_rx) = std::sync::mpsc::channel::<String>();
    let (clients_tx, clients_rx) = std::sync::mpsc::channel::<String>();
    let server_thread = start_server_thread(server_rx, clients_tx);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main(server_tx, clients_rx))
}

async fn tokio_main(
    sync_server_tx: std::sync::mpsc::Sender<String>,
    sync_clients_rx: std::sync::mpsc::Receiver<String>,
) {
    init_logger();

    let (ws_from_client_tx, ws_from_client_rx) = mpsc::unbounded_channel::<String>();
    let (async_clients_tx, async_clients_rx) = mpsc::unbounded_channel::<String>();
    // let server_tx = Arc::new(server_tx);

    let clients = Arc::new(RwLock::new(ClientSharedState::new(ws_from_client_tx)));

    start_async_to_sync_channels_thread(ws_from_client_rx, sync_server_tx);
    start_single_async_to_multiple_clients_sender(clients.clone(), async_clients_rx);
    let _thread = start_sync_to_async_clients_sender(sync_clients_rx, async_clients_tx);

    let websocket_service = warp::path("ccdi")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    let index = warp::path::end().map(|| warp::reply::html(INDEX));

    let routes = warp::get().and(websocket_service.or(index));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

pub async fn ws_handler(ws: Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_client_connection(socket, clients)))
}

fn to_string<T: Debug>(item: T) -> String {
    format!("{:?}", item)
}

fn log_err(label: &str, result: Result<(), String>) {
    if let Err(error) = result {
        eprintln!("Error in '{}': {}", label, error)
    }
}

