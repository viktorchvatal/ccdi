use ccdi_common::{ClientMessage, StateMessage};

// ============================================ PUBLIC =============================================

pub struct State {

}

impl State {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn process(&mut self, message: StateMessage) -> Result<Vec<ClientMessage>, String> {
        use StateMessage::*;

        Ok(match message {
            ClientTest(number) => vec![ClientMessage::ClientTestResponse(number*2)],
        })
    }
}