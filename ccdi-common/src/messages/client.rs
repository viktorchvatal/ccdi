use serde_derive::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    ClientTestResponse(i32),
    View(ViewState),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ViewState {
    pub header: String,
}