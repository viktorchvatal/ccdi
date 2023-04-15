use serde_derive::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StateMessage {
    ClientTest(i32),
    ClientConnected,
}