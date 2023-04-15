mod logger;
mod server;
mod websocket;
mod bridge;

use std::fmt::Debug;
use tokio::sync::mpsc;

use logger::init_logger;
use server::start_server_thread;
use warp::Filter;
use websocket::create_clients;
use websocket::create_websocket_service;
use websocket::start_single_async_to_multiple_clients_sender;
use bridge::start_tokio_to_std_channel_bridge;
use bridge::start_std_to_tokio_channel_bridge;

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

    let clients = create_clients(ws_from_client_tx);

    start_tokio_to_std_channel_bridge(ws_from_client_rx, sync_server_tx);
    start_single_async_to_multiple_clients_sender(clients.clone(), async_clients_rx);
    let _thread = start_std_to_tokio_channel_bridge(sync_clients_rx, async_clients_tx);

    let websocket_service = create_websocket_service("ccdi", clients);

    let index = warp::path::end().map(|| warp::reply::html(INDEX));

    let routes = warp::get().and(websocket_service.or(index));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080)).await;
}

fn to_string<T: Debug>(item: T) -> String {
    format!("{:?}", item)
}

fn log_err(label: &str, result: Result<(), String>) {
    if let Err(error) = result {
        eprintln!("Error in '{}': {}", label, error)
    }
}

