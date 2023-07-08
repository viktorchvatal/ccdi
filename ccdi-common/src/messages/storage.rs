use std::sync::Arc;

use serde_derive::{Serialize, Deserialize};

use crate::RawImage;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StorageMessage {
    EnableStore(String),
    DisableStore,
    ProcessImage(Arc<RawImage>),
}