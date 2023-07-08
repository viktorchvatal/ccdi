use std::{thread::{self, JoinHandle}, time::Duration, sync::{mpsc::RecvTimeoutError, Arc}};
use std::sync::mpsc::{Sender, Receiver};

use ccdi_common::{StateMessage, ClientMessage, log_err, ProcessMessage, StorageMessage};
use log::{error, debug};

use crate::{state::BackendState, convert::handle_process_message, ServiceConfig, storage::Storage};

// ============================================ PUBLIC =============================================

pub struct LogicParams {
    pub demo_mode: bool,
}

pub fn start_logic_thread(
    params: LogicParams,
    config: Arc<ServiceConfig>,
    server_rx: Receiver<StateMessage>,
    clients_tx: Sender<ClientMessage>,
    process_tx: Sender<ProcessMessage>,
    storage_tx: Sender<StorageMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("logic".to_string())
        .spawn(move || {
            let mut state = BackendState::new(params.demo_mode, process_tx, storage_tx, config);

            loop {
                match server_rx.recv_timeout(Duration::from_millis(50)) {
                    // Process the received message
                    Ok(message) => receive_message(&mut state, message, &clients_tx),
                    // Last sender disconnected - exit thread
                    Err(RecvTimeoutError::Disconnected) => return,
                    // No messages received within timeout - perform periodic tasks
                    Err(RecvTimeoutError::Timeout) => periodic_tasks(&mut state, &clients_tx),
                }
            }
        })
        .map_err(|err| format!("{:?}", err))
}

pub fn start_process_thread(
    process_rx: Receiver<ProcessMessage>,
    clients_tx: Sender<ClientMessage>,
    server_tx: Sender<StateMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("logic".to_string())
        .spawn(move || {
            loop {
                match process_rx.recv() {
                    // Process the received message
                    Ok(message) => {
                        debug!("Handling image process request");

                        let reply = handle_process_message(message);

                        debug!("Image process finished");

                        for message in reply.into_iter() {
                            if let ClientMessage::RgbImage(ref image) = message {
                                log_err("Send process message to server", server_tx.send(
                                    StateMessage::ImageDisplayed(image.clone())
                                ));
                            }

                            log_err("Send process message to client", clients_tx.send(message));
                        }
                    },
                    // Last sender disconnected - exit thread
                    Err(_) => return,
                }
            }
        })
        .map_err(|err| format!("{:?}", err))
}

pub fn start_storage_thread(
    config: Arc<ServiceConfig>,
    storage_rx: Receiver<StorageMessage>,
    server_tx: Sender<StateMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("logic".to_string())
        .spawn(move || {
            let mut storage = Storage::new(config);

            let send_results = |result: Result<Vec<StateMessage>, String>| match result {
                Ok(messages) => {
                    for message in messages {
                        log_err("Send message from storage to server", server_tx.send(message));
                    }
                },
                Err(error) => error!(
                    "Processing storage messages or periadic task failed: {}", error
                )
            };

            loop {
                match storage_rx.recv_timeout(Duration::from_millis(1000)) {
                    // Process the received message
                    Ok(message) => send_results(storage.process(message)),
                    // Last sender disconnected - exit thread
                    Err(RecvTimeoutError::Disconnected) => return,
                    // No messages received within timeout - perform periodic tasks
                    Err(RecvTimeoutError::Timeout) => send_results(storage.periodic_tasks()),
                }
            }
        })
        .map_err(|err| format!("{:?}", err))
}

// =========================================== PRIVATE =============================================

fn receive_message(
    state: &mut BackendState,
    message: StateMessage,
    clients_tx: &Sender<ClientMessage>,
) {
    if let Some(responses) = log_err("Process state message", state.process(message)) {
        send_client_messages(responses, clients_tx);
    }
}

fn periodic_tasks(
    state: &mut BackendState,
    clients_tx: &Sender<ClientMessage>,
) {
    if let Some(responses) = log_err("Perform periodic tasks", state.periodic()) {
        send_client_messages(responses, clients_tx);
    }
}

fn send_client_messages(
    messages: Vec<ClientMessage>,
    clients_tx: &Sender<ClientMessage>,
) {
    for message in messages {
        log_err("Send client response", clients_tx.send(message));
    }
}