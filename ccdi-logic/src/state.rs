use std::sync::{Arc, mpsc::Sender};

use ccdi_common::{ClientMessage, StateMessage, RgbImage, ProcessMessage};

use crate::{camera::CameraController, ServiceConfig};

// ============================================ PUBLIC =============================================

pub struct BackendState {
    camera: CameraController,
    /// Last image sent to clients
    image: Option<Arc<RgbImage<u16>>>,
}

impl BackendState {
    pub fn new(
        demo_mode: bool,
        process_tx: Sender<ProcessMessage>,
        config: Arc<ServiceConfig>,
    ) -> Self {
        Self {
            camera: CameraController::new(
                match demo_mode {
                    false => Box::new(ccdi_imager_moravian::MoravianImagerDriver::new()),
                    true => Box::new(ccdi_imager_demo::DemoImagerDriver::new()),
                },
                process_tx,
                config
            ),
            image: None,
        }
    }

    /// Process incoming message and return messages to be sent to clients
    pub fn process(&mut self, message: StateMessage) -> Result<Vec<ClientMessage>, String> {
        use StateMessage::*;

        Ok(match message {
            ImageDisplayed(image) => {
                self.image = Some(image);
                vec![]
            },
            CameraParam(message) => {
                self.camera.update_camera_params(message);
                vec![ClientMessage::View(self.camera.get_view())]
            },
            ExposureMessage(command) => {
                self.camera.exposure_command(command);
                vec![ClientMessage::View(self.camera.get_view())]
            },
            ClientConnected => {
                let view_msg = ClientMessage::View(self.camera.get_view());

                match self.image.as_ref() {
                    None => vec![view_msg],
                    Some(image) => vec![view_msg, ClientMessage::RgbImage(image.clone())],
                }
            }
            UpdateStorageState(storage_state) => {
                self.camera.update_storage_status(storage_state);
                vec![ClientMessage::View(self.camera.get_view())]
            },
        })
    }

    /// Called periodically to perform any tasks needed and return messages for clients
    pub fn periodic(&mut self) -> Result<Vec<ClientMessage>, String> {
        Ok(self.camera.periodic())
    }
}

// =========================================== PRIVATE =============================================
