use ccdi_common::{ClientMessage, StateMessage, ViewState};

// ============================================ PUBLIC =============================================

pub struct State {

}

impl State {
    pub fn new() -> Self {
        Self {

        }
    }

    /// Process incoming message and return messages to be sent to clients
    pub fn process(&mut self, message: StateMessage) -> Result<Vec<ClientMessage>, String> {
        use StateMessage::*;

        Ok(match message {
            ClientTest(number) => vec![ClientMessage::ClientTestResponse(number*2)],
            ClientConnected => vec![
                ClientMessage::View(self.get_view()),
                ClientMessage::JpegImage(TEST_IMAGE.to_vec()),
            ]
        })
    }

    /// Called periodically to perform any tasks needed and return messages for clients
    pub fn periodic(&mut self) -> Result<Vec<ClientMessage>, String> {
        Ok(vec![])
    }
}

// =========================================== PRIVATE =============================================

impl State {
    fn get_view(&self) -> ViewState {
        ViewState {
            header: format!("Initial view")
        }
    }
}

const TEST_IMAGE: &[u8] = include_bytes!("test-image.jpg");