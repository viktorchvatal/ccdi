// ============================================ PUBLIC =============================================

use ccdi_imager_interface::{ImagerDriver, ImagerDevice};

pub struct CameraState {
    driver: Box<dyn ImagerDriver>
}

impl CameraState {
    pub fn new(driver: Box<dyn ImagerDriver>) -> Self {
        Self {
            driver
        }
    }
}