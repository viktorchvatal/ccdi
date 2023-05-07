use nanocv::ImgSize;
use serde_derive::{Serialize, Deserialize};

use crate::RawImage;

// ============================================ PUBLIC =============================================

/// Message for image processing thread
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ProcessMessage {
    ConvertRawImage(ConvertRawImage),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ConvertRawImage {
    pub image: RawImage,
    pub size: ImgSize,
}