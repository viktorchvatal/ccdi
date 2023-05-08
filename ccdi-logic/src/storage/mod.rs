use std::sync::Arc;

use ccdi_common::{StorageMessage, StateMessage};

use crate::ServiceConfig;

// ============================================ PUBLIC =============================================

pub struct Storage {
    config: Arc<ServiceConfig>,
}

impl Storage {
    pub fn new(config: Arc<ServiceConfig>) -> Self {
        Self { config }
    }

    pub fn process(&mut self, message: StorageMessage) -> Result<Vec<StateMessage>, String> {
        Ok(vec![])
    }

    pub fn periodic_tasks(&mut self) -> Result<Vec<StateMessage>, String> {
        Ok(vec![])
    }
}