use std::sync::Arc;

use ccdi_common::{ExposureCommand, ClientMessage, ConnectionState};
use ccdi_imager_interface::{ImagerDevice, ImagerProperties};

use super::{properties::{PropertiesController}, exposure::ExposureController};

// ============================================ PUBLIC =============================================

pub struct ConnectedCameraController {
    device: Box<dyn ImagerDevice>,
    properties: PropertiesController,
    exposure: ExposureController,
    messages: Vec<ClientMessage>,
}

impl ConnectedCameraController {
    pub fn new(
        mut device: Box<dyn ImagerDevice>,
    ) -> Result<Self, String> {
        let properties = PropertiesController::new(device.as_mut())?;
        let exposure = ExposureController::new(properties.get_properties().basic);
        Ok(Self {properties, exposure, device, messages: vec![]})
    }

    pub fn close(mut self) {
        self.device.close()
    }

    pub fn periodic(&mut self) -> Result<(), String> {
        self.messages.append(&mut self.exposure.periodic(self.device.as_mut())?);

        self.properties
            .read_properties(self.device.as_mut())
            .map_err(|_| format!("Periodic read properties failed"))
    }

    pub fn get_properties(&self) -> Arc<ImagerProperties> {
        self.properties.get_properties()
    }

    pub fn exposure_command(&mut self, command: ExposureCommand) -> Result<(), String> {
        self.exposure.exposure_command(self.device.as_mut(), command)
    }

    pub fn flush_messages(&mut self) -> Vec<ClientMessage> {
        let mut result = Vec::new();
        result.append(&mut self.messages);
        result

    }

    pub fn exposure_status(&self) -> ConnectionState {
        match self.exposure.exposure_active() {
            false => ConnectionState::Disconnected,
            true => ConnectionState::Established
        }
    }
}