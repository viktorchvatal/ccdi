use ccdi_common::{ClientMessage, StateMessage, ViewState, LogicStatus};
use ccdi_imager_demo::DemoImagerDriver;

use crate::camera::CameraController;

// ============================================ PUBLIC =============================================

pub struct State {
    camera: CameraController
}

impl State {
    pub fn new() -> Self {
        Self {
            camera: CameraController::new(
                Box::new(DemoImagerDriver::new())
            )
        }
    }

    /// Process incoming message and return messages to be sent to clients
    pub fn process(&mut self, message: StateMessage) -> Result<Vec<ClientMessage>, String> {
        use StateMessage::*;

        Ok(match message {
            ClientTest(number) => vec![ClientMessage::ClientTestResponse(number*2)],
            ClientConnected => vec![
                ClientMessage::View(self.get_view()),
                ClientMessage::JpegImage(TEST_IMAGE.to_vec()),
            ]
        })
    }

    /// Called periodically to perform any tasks needed and return messages for clients
    pub fn periodic(&mut self) -> Result<Vec<ClientMessage>, String> {
        self.camera.periodic();
        Ok(vec![ClientMessage::View(self.get_view()),])
    }
}

// =========================================== PRIVATE =============================================

impl State {
    fn get_view(&self) -> ViewState {
        ViewState {
            detail: self.camera.detail(),
            status: LogicStatus {
                camera: self.camera.connection_state(),
            },
            camera_properties: self.camera.properties(),
        }
    }
}

const TEST_IMAGE: &[u8] = include_bytes!("test-image.jpg");