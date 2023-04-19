mod websocket;
mod bridge;
mod config;

use ccdi_common::ClientMessage;
use ccdi_common::StateMessage;
use ccdi_common::init_logger;
use ccdi_logic::LogicConfig;
use ccdi_logic::start_logic_thread;
use config::ServiceConfig;
use tokio::sync::mpsc;

use warp::Filter;
use websocket::create_clients;
use websocket::create_websocket_service;
use websocket::start_single_async_to_multiple_clients_sender;
use bridge::start_tokio_to_std_channel_bridge;
use bridge::start_std_to_tokio_channel_bridge;

// ============================================ PUBLIC =============================================

const INDEX: &[u8] = include_bytes!("static/index.html");
const WASM: &[u8] = include_bytes!("static/ccdi-web-client.wasm");
const JS: &[u8] = include_bytes!("static/ccdi-web-client.js");
const CSS: &[u8] = include_bytes!("static/ccdi-web-client.css");

fn main() {
    let config: ServiceConfig = argh::from_env();

    let logic_config = LogicConfig {
        demo_mode: config.demo,
    };

    let (server_tx, server_rx) = std::sync::mpsc::channel::<StateMessage>();
    let (clients_tx, clients_rx) = std::sync::mpsc::channel::<ClientMessage>();
    let _server_thread = start_logic_thread(logic_config, server_rx, clients_tx);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main(server_tx, clients_rx))
}

async fn tokio_main(
    sync_server_tx: std::sync::mpsc::Sender<StateMessage>,
    sync_clients_rx: std::sync::mpsc::Receiver<ClientMessage>,
) {
    init_logger(true);

    let (ws_from_client_tx, ws_from_client_rx) = mpsc::unbounded_channel::<StateMessage>();
    let (async_clients_tx, async_clients_rx) = mpsc::unbounded_channel::<ClientMessage>();
    // let server_tx = Arc::new(server_tx);

    let clients = create_clients(ws_from_client_tx);

    start_tokio_to_std_channel_bridge(ws_from_client_rx, sync_server_tx);
    start_single_async_to_multiple_clients_sender(clients.clone(), async_clients_rx);
    let _thread = start_std_to_tokio_channel_bridge(sync_clients_rx, async_clients_tx);

    let websocket_service = create_websocket_service("ccdi", clients);

    let index = warp::path::end().map(|| warp::reply::html(INDEX));

    let wasm = warp::path("ccdi-web-client.wasm")
        .map(|| warp::reply::with_header(WASM, "Content-Type", "application/wasm"));

    let js = warp::path("ccdi-web-client.js")
        .map(|| warp::reply::with_header(JS, "Content-Type", "text/javascript"));

    let css = warp::path("ccdi-web-client.css")
        .map(|| warp::reply::with_header(CSS, "Content-Type", "text/css"));

    let static_files = wasm.or(js).or(css).or(index);

    let routes = warp::get().and(
        websocket_service.or(static_files)
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8081)).await;
}
