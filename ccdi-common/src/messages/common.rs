use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StorageState {
    Unknown,
    Error(String),
    Available(StorageDetails)
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StorageDetails {
    pub total_gigabytes: f64,
    pub free_gigabytes: f64,
}