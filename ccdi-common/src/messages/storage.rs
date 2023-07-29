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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StorageDetail {
    pub storage_name: String,
    pub counter: usize,
    pub storage_log: Vec<StorageLogRecord>,
    pub storage_enabled: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StorageLogRecord {
    pub name: String,
    pub status: StorageLogStatus,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StorageLogStatus {
    Success,
    Error(String),
}

impl Default for StorageDetail {
    fn default() -> Self {
        Self {
            storage_name: String::from("?"),
            counter: 0,
            storage_log: Vec::new(),
            storage_enabled: false,
        }
    }
}