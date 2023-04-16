use std::{sync::Arc, time::Instant};

use ccdi_imager_interface::{ImagerDevice, ImagerProperties};

// ============================================ PUBLIC =============================================

pub struct PropertiesController {
    properties: Arc<ImagerProperties>,
    last_properties_read: Instant
}

impl PropertiesController {
    pub fn new(device: &mut dyn ImagerDevice) -> Result<Self, String> {
        match device.read_properties() {
            Err(_) => Err(String::from("Reading camera properties failed")),
            Ok(properties) => Ok(
                Self {
                    properties: Arc::new(properties),
                    last_properties_read: Instant::now(),
                }
            )
        }
    }

    pub fn read_properties(
        &mut self,
        device: &mut dyn ImagerDevice
    ) -> Result<(), ()> {
        match self.should_read_properties() {
            false => Ok(()),
            true => match device.read_properties() {
                Ok(properties) => {
                    self.properties = Arc::new(properties);
                    self.last_properties_read = Instant::now();
                    Ok(())
                },
                Err(_) => {
                    Err(())
                }
            }
        }
    }

    pub fn get_properties(&self) -> Arc<ImagerProperties> {
        self.properties.clone()
    }
}

// =========================================== PRIVATE =============================================

const PROPERTIES_READ_INTERVAL: f64 = 2.0; // Seconds

impl PropertiesController {
    fn should_read_properties(&self) -> bool {
        self.last_properties_read.elapsed().as_secs_f64() >= PROPERTIES_READ_INTERVAL
    }
}