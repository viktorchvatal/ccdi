use std::collections::HashMap;
use std::sync::Arc;
use futures::stream::SplitSink;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use futures::StreamExt;
use warp::ws::Message;
use warp::ws::WebSocket;

use crate::WebServerMessage;
use crate::log_err;
use crate::to_string;

pub type Clients = Arc<RwLock<ClientSharedState>>;

pub struct ClientSharedState {
    counter: usize,
    server_tx: Sender<WebServerMessage>,
    transmitters: HashMap<usize, SplitSink<WebSocket, Message>>
}

impl ClientSharedState {
    pub fn new(server_tx: Sender<WebServerMessage>) -> Self {
        Self {
            counter: 0,
            server_tx,
            transmitters: HashMap::new()
        }
    }

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

pub fn start_async_to_sync_channels_thread(
    async_clients_rx: Receiver<WebServerMessage>,
    sync_server_tx: std::sync::mpsc::Sender<String>
) {
    tokio::spawn(async move {
        bridge_async_client_channels_to_mspc_channels(
            async_clients_rx, sync_server_tx
        ).await;
    });
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

async fn bridge_async_client_channels_to_mspc_channels(
    mut async_clients_rx: Receiver<WebServerMessage>,
    sync_server_tx: std::sync::mpsc::Sender<String>
) {
    while let Some(message) = async_clients_rx.recv().await {
        log_err(
            "Sending message to server worker thread",
            sync_server_tx.send(message).map_err(to_string)
        )
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
        String::from_utf8(message.into_bytes())
            .map_err(|err| format!("{:?}", err))
    } else {
        Err(String::from("Msg is not text"))
    }
}
