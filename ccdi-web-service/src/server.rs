use std::{thread::{self, JoinHandle}};
use std::sync::mpsc::{Sender, Receiver};

pub fn start_server_thread(
    server_rx: Receiver<String>,
    clients_tx: Sender<String>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            loop {
                match server_rx.recv() {
                    Ok(message) => {
                        ::log::info!("Server thread received: {:?}", message);
                        let _ = clients_tx.send(String::from(r#"{"value":666}"#));
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