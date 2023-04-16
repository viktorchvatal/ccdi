// ============================================ PUBLIC =============================================

use ccdi_imager_interface::BasicProperties;

use super::properties;

pub struct ExposureController {
    properties: BasicProperties
}

impl ExposureController {
    pub fn new(properties: BasicProperties) -> Self {
        Self { properties }
    }
}