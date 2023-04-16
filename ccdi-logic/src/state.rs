use ccdi_common::{ClientMessage, StateMessage};

use crate::camera::CameraController;

// ============================================ PUBLIC =============================================

pub struct State {
    camera: CameraController
}

impl State {
    pub fn new() -> Self {
        Self {
            camera: CameraController::new(Box::new(
                ccdi_imager_demo::DemoImagerDriver::new()
                // ccdi_imager_moravian::MoravianImagerDriver::new()
            ))
        }
    }

    /// Process incoming message and return messages to be sent to clients
    pub fn process(&mut self, message: StateMessage) -> Result<Vec<ClientMessage>, String> {
        use StateMessage::*;

        Ok(match message {
            ClientTest(number) => vec![ClientMessage::ClientTestResponse(number*2)],
            ClientConnected => vec![
                ClientMessage::View(self.camera.get_view()),
                ClientMessage::JpegImage(TEST_IMAGE.to_vec()),
            ]
        })
    }

    /// Called periodically to perform any tasks needed and return messages for clients
    pub fn periodic(&mut self) -> Result<Vec<ClientMessage>, String> {
        self.camera.periodic();
        Ok(vec![ClientMessage::View(self.camera.get_view()),])
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================== PRIVATE =============================================

const TEST_IMAGE: &[u8] = include_bytes!("test-image.jpg");