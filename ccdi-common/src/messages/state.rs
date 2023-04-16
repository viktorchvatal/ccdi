use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StateMessage {
    ExposureMessage(ExposureCommand),
    ClientConnected,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ExposureCommand {
    Start,
    SetGain(i32),
    SetTime(f64),
}