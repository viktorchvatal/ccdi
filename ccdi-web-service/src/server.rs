use std::{thread::{self, JoinHandle}};
use std::sync::mpsc::{Sender, Receiver};

use ccdi_common::{StateMessage, ClientMessage};

pub fn start_server_thread(
    server_rx: Receiver<StateMessage>,
    clients_tx: Sender<ClientMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            loop {
                match server_rx.recv() {
                    Ok(message) => {
                        ::log::info!("Server thread received: {:?}", message);
                        let _ = clients_tx.send(ClientMessage::ClientTestResponse(666));
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