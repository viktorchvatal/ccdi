use std::{thread::{self, JoinHandle}};
use std::sync::mpsc::Receiver;

use crate::WebServerMessage;

pub fn start_server_thread(
    server_rx: Receiver<WebServerMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("algorithm".to_string())
        .spawn(move || {
            loop {
                match server_rx.recv() {
                    Ok(message) => ::log::info!("Server thread received: {:?}", message),
                    Err(_) => {
                        // Channel closed, exit
                        return;
                    },
                }
            }
        })
        .map_err(|err| format!("{:?}", err))
}