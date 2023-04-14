use std::{thread::{self, JoinHandle}};
use std::sync::mpsc::{Sender, Receiver};

use crate::{WebServerMessage, WebClientMessage};

pub fn start_server_thread(
    server_rx: Receiver<WebServerMessage>,
    clients_tx: Sender<WebClientMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("algorithm".to_string())
        .spawn(move || {
            loop {
                match server_rx.recv() {
                    Ok(message) => {
                        ::log::info!("Server thread received: {:?}", message);
                        let _ = clients_tx.send(format!("Returned back: {}", message));
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