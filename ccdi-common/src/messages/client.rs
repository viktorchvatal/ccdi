use std::sync::Arc;

use ccdi_imager_interface::{ImagerProperties, ExposureParams};
use nanocv::ImgSize;
use serde_derive::{Serialize, Deserialize};

use crate::{RgbImage, RenderingType, StorageState};

use super::gui_config::GuiConfig;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Reconnect,
    View(ViewState),
    RgbImage(Arc<RgbImage<u16>>),
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
    pub camera_properties: Option<Arc<ImagerProperties>>,
    pub camera_params: CameraParams,
    pub config: GuiConfig,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            detail: String::new(),
            status: Default::default(),
            camera_properties: None,
            camera_params: Default::default(),
            config: GuiConfig::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CameraParams {
    pub loop_enabled: bool,
    pub gain: u16,
    pub time: f64,
    pub rendering: RenderingType,
    pub render_size: ImgSize,
    pub temperature: f64,
}

impl CameraParams {
    pub fn new(render_size: ImgSize) -> Self {
        Self {
            loop_enabled: false,
            gain: 0,
            time: 1.0,
            rendering: RenderingType::FullImage,
            render_size,
            temperature: 20.0,
        }
    }
}

impl Default for CameraParams {
    fn default() -> Self {
        CameraParams::new(ImgSize::new(900, 600))
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct LogicStatus {
    pub camera: ConnectionState,
    pub exposure: ConnectionState,
    pub storage: StorageState,
}

impl Default for LogicStatus {
    fn default() -> Self {
        Self {
            camera: ConnectionState::Disconnected,
            exposure: ConnectionState::Disconnected,
            storage: StorageState::Unknown,
        }
    }
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