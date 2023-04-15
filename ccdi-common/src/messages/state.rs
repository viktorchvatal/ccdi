use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StateMessage {
    ClientTest(i32),
    ClientConnected,
}