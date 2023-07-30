use std::sync::{Arc, mpsc::Sender};

use ccdi_common::{ClientMessage, StateMessage, RgbImage, ProcessMessage, StorageMessage, IoMessage};

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
        storage_tx: Sender<StorageMessage>,
        config: Arc<ServiceConfig>,
    ) -> Self {
        Self {
            camera: CameraController::new(
                match demo_mode {
                    false => Box::new(ccdi_imager_moravian::MoravianImagerDriver::new()),
                    true => Box::new(ccdi_imager_demo::DemoImagerDriver::new()),
                },
                process_tx,
                storage_tx,
                config
            ),
            image: None,
        }
    }

    /// Process incoming message and return messages to be sent to clients
    pub fn process(&mut self, message: StateMessage) -> Result<BackendResult, String> {
        use StateMessage::*;

        Ok(match message {
            ImageDisplayed(image) => {
                self.image = Some(image);
                BackendResult::empty()
            },
            CameraParam(message) => {
                self.camera.update_camera_params(message);
                self.return_view()
            },
            ExposureMessage(command) => {
                self.camera.exposure_command(command);
                self.return_view()
            },
            ClientConnected => {
                let view_msg = ClientMessage::View(self.camera.get_view());

                BackendResult::client(
                    match self.image.as_ref() {
                        None => vec![view_msg],
                        Some(image) => vec![view_msg, ClientMessage::RgbImage(image.clone())],
                    }
                )
            }
            UpdateStorageState(storage_state) => {
                self.camera.update_storage_status(storage_state);
                self.return_view()
            },
            TriggerValueChanged(value) => {
                self.camera.update_trigger_status(value);
                // Trigger might be switched on, perform idle tasks immediately
                let (client, io) = self.camera.periodic();
                BackendResult::client_io(client, io)
            },
            StorageMessage(message) => {
                BackendResult {
                    client_messages: Vec::new(),
                    storage_messages: vec![message],
                    io_messages: Vec::new(),
                }
            },
            UpdateStorageDetail(detail) => {
                self.camera.update_storage_detail(detail);
                self.return_view()
            },
        })
    }

    /// Called periodically to perform any tasks needed and return messages for clients
    pub fn periodic(&mut self) -> Result<BackendResult, String> {
        let (client, io) = self.camera.periodic();
        Ok(BackendResult::client_io(client, io))
    }
}

pub struct BackendResult {
    pub client_messages: Vec<ClientMessage>,
    pub storage_messages: Vec<StorageMessage>,
    pub io_messages: Vec<IoMessage>,
}

impl BackendResult {
    pub fn empty() -> Self {
        BackendResult {
            client_messages: Vec::new(),
            storage_messages: Vec::new(),
            io_messages: Vec::new(),
        }
    }

    pub fn client(client_messages: Vec<ClientMessage>) -> Self {
        Self {
            client_messages,
            storage_messages: Vec::new(),
            io_messages: Vec::new(),
        }
    }

    pub fn client_io(
        client: Vec<ClientMessage>,
        io: Vec<IoMessage>,
    ) -> Self {
        Self {
            client_messages: client,
            io_messages: io,
            storage_messages: Vec::new(),
        }
    }
}

// =========================================== PRIVATE =============================================

impl BackendState {
    fn return_view(&self) -> BackendResult {
        BackendResult {
            client_messages: vec![ClientMessage::View(self.camera.get_view())],
            storage_messages: Vec::new(),
            io_messages: Vec::new(),
        }
    }
}