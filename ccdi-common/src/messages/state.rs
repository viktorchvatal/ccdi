use std::sync::Arc;

use serde_derive::{Serialize, Deserialize};

use crate::RgbImage;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StateMessage {
    ExposureMessage(ExposureCommand),
    CameraParam(CameraParamMessage),
    ClientConnected,
    ImageDisplayed(Arc<RgbImage<u16>>),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ExposureCommand {
    Start,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CameraParamMessage {
    EnableLoop(bool),
    SetGain(u16),
    SetTime(f64),
    SetRenderingType(RenderingType),
}


#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum RenderingType {
    FullImage,
    Center1x,
    Corners1x,
}