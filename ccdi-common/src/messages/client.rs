use serde_derive::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    ClientTestResponse(i32),
    View(ViewState),
    JpegImage(Vec<u8>),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ViewState {
    pub header: String,
}