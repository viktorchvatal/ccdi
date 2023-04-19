use std::sync::Arc;

use ccdi_imager_interface::{ImagerProperties, ExposureParams};
use serde_derive::{Serialize, Deserialize};

use crate::RgbImage;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    View(ViewState),
    JpegImage(Vec<u8>),
    RgbImage(RgbImage<u16>),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RawImage {
    pub params: ExposureParams,
    pub data: Vec<u16>
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ViewState {
    pub detail: String,
    pub status: LogicStatus,
    pub camera_properties: Option<Arc<ImagerProperties>>
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct LogicStatus {
    pub camera: ConnectionState,
    pub exposure: ConnectionState,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Established
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}