use ccdi_imager_interface::ImagerProperties;
use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    ClientTestResponse(i32),
    View(ViewState),
    JpegImage(Vec<u8>),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ViewState {
    pub detail: String,
    pub status: LogicStatus,
    pub camera_properties: Option<ImagerProperties>
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct LogicStatus {
    pub camera: ConnectionState,
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