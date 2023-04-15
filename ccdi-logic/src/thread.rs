use std::{thread::{self, JoinHandle}};
use std::sync::mpsc::{Sender, Receiver};

use ccdi_common::{StateMessage, ClientMessage, log_err};
use log::{debug};

use crate::state::State;

// ============================================ PUBLIC =============================================

pub fn start_logic_thread(
    server_rx: Receiver<StateMessage>,
    clients_tx: Sender<ClientMessage>,
) -> Result<JoinHandle<()>, String> {
    thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            loop {
                let mut state = State::new();

                match server_rx.recv() {
                    Ok(message) => receive_message(&mut state, message, &clients_tx),
                    Err(_) => {
                        // Channel closed, exit
                        return;
                    },
                }
            }
        })
        .map_err(|err| format!("{:?}", err))
}

// =========================================== PRIVATE =============================================

fn receive_message(
    state: &mut State,
    message: StateMessage,
    clients_tx: &Sender<ClientMessage>,
) {
    debug!("State received: {:?}", &message);

    if let Some(responses) = log_err("Process state message", state.process(message)) {
        for response in responses {
            log_err("Send client response", clients_tx.send(response));
        }
    }

}