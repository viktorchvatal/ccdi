use ccdi_common::{ClientMessage, StateMessage, ViewState};

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
            ClientConnected => vec![ClientMessage::View(self.get_view())]
        })
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