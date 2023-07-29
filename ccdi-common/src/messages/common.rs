use serde_derive::{Serialize, Deserialize};

use crate::ConnectionState;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StorageState {
    Unknown,
    Error(String),
    Available(StorageCapacity)
}

impl StorageState {
    pub fn as_connection_state(&self) -> ConnectionState {
        match self {
            StorageState::Unknown => ConnectionState::Disconnected,
            StorageState::Error(_) => ConnectionState::Disconnected,
            StorageState::Available(_) => ConnectionState::Established,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StorageCapacity {
    pub total_gigabytes: f64,
    pub free_gigabytes: f64,
}