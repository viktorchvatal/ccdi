use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio_stream::wrappers::UnboundedReceiverStream;
use std::{thread::{self, JoinHandle}};

use futures::{StreamExt, FutureExt};
use warp::Error;
use warp::ws::Message;
use warp::ws::WebSocket;

use crate::log_err;
use crate::to_string;

pub type Clients = Arc<RwLock<ClientSharedState>>;

pub struct ClientSharedState {
    counter: usize,
    server_tx: UnboundedSender<String>,
    transmitters: HashMap<usize, UnboundedSender<Result<Message, Error>>>
}

impl ClientSharedState {
    pub fn new(server_tx: UnboundedSender<String>) -> Self {
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
        ::log::info!("Client {} registered ({} clients total)", id, self.transmitters.len());
        id
    }

    fn unregister_client(&mut self, id: usize) -> Result<(), String> {
        self.transmitters.remove(&id);
        ::log::info!("Client {} unregistered ({} clients total)", id, self.transmitters.len());
        Ok(())
    }
}

pub fn start_async_to_sync_channels_thread(
    async_clients_rx: UnboundedReceiver<String>,
    sync_server_tx: std::sync::mpsc::Sender<String>
) {
    tokio::spawn(async move {
        bridge_async_client_channels_to_mspc_channels(
            async_clients_rx, sync_server_tx
        ).await;
    });
}

pub fn start_sync_to_async_clients_sender(
    clients_rx: std::sync::mpsc::Receiver<String>,
    async_clients_tx: tokio::sync::mpsc::UnboundedSender<String>
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("server_to_tokio_transmitter".to_string())
        .spawn(move || {
            loop {
                match clients_rx.recv() {
                    Ok(message) => {
                        log_err(
                            "Clients sync to async transmitter",
                            async_clients_tx.send(message).map_err(to_string)
                        )
                    },
                    Err(_) => {
                        // Channel closed, exit
                        return;
                    },
                }
            }
        })
        .map_err(|err| format!("{:?}", err))
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

pub async fn handle_client_connection(
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

async fn bridge_async_client_channels_to_mspc_channels(
    mut async_clients_rx: UnboundedReceiver<String>,
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
