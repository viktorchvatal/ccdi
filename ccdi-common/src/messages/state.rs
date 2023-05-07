use std::sync::Arc;

use serde_derive::{Serialize, Deserialize};

use crate::RgbImage;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StateMessage {
    ExposureMessage(ExposureCommand),
    ClientConnected,
    ImageDisplayed(Arc<RgbImage<u16>>),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ExposureCommand {
    Start,
    SetGain(u16),
    SetTime(f64),
}