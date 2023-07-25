use std::sync::Arc;

use serde_derive::{Serialize, Deserialize};

use crate::RawImage;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StorageMessage {
    EnableStore,
    DisableStore,
    ProcessImage(Arc<RawImage>),
    SetDirectory(String),
}