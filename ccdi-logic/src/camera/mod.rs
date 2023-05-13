mod properties;
mod exposure;
mod connected;

use std::sync::{mpsc::Sender, Arc};

use ccdi_common::{
    ConnectionState, ViewState, LogicStatus, ExposureCommand, ClientMessage, ProcessMessage,
    CameraParams, CameraParamMessage, StorageState
};
use ccdi_imager_interface::{ImagerDriver, DeviceDescriptor};
use log::{info};

use crate::{ServiceConfig, storage::Storage};

use self::{connected::ConnectedCameraController};

// ============================================ PUBLIC =============================================

pub struct CameraController {
    driver: Box<dyn ImagerDriver>,
    state: State,
    detail: String,
    connected: Option<ConnectedCameraController>,
    view: Option<ViewState>,
    camera_params: CameraParams,
    process_tx: Sender<ProcessMessage>,
    storage_status: StorageState,
}

impl CameraController {
    pub fn new(
        driver: Box<dyn ImagerDriver>,
        process_tx: Sender<ProcessMessage>,
        config: Arc<ServiceConfig>,
    ) -> Self {
        Self {
            driver,
            state: State::Error,
            connected: None,
            detail: String::from("Started"),
            view: None,
            camera_params: CameraParams::new(config.render_size),
            process_tx,
            storage_status: StorageState::Unknown,
        }
    }

    pub fn periodic(&mut self) -> Vec<ClientMessage> {
        let old_state = self.state;

        self.state = match self.state {
            State::Error => self.handle_error_state(),
            State::Connected => self.handle_connected_state(),
        };

        if self.state != old_state {
            info!("Camera state {:?} -> {:?}", old_state, self.state);
        }

        let new_view = self.get_view();

        let mut messages = vec![];

        if self.view != Some(new_view.clone()) {
            self.view = Some(new_view.clone());
            messages.push(ClientMessage::View(new_view))
        }

        if let Some(ref mut camera) = self.connected {
            messages.append(&mut camera.flush_messages());
        }

        messages
    }

    pub fn get_view(&self) -> ViewState {
        ViewState {
            detail: self.detail.clone(),
            status: LogicStatus {
                camera: self.connection_state(),
                exposure: self.connected.as_ref().map(|cam| cam.exposure_status())
                    .unwrap_or(ConnectionState::Disconnected),
                storage: self.storage_status.clone()
            },
            camera_properties: self.connected.as_ref().map(|cam| cam.get_properties()),
            camera_params: self.camera_params.clone(),
        }
    }

    pub fn update_camera_params(&mut self, message: CameraParamMessage) {
        use CameraParamMessage::*;

        match message {
            EnableLoop(value) => self.camera_params.loop_enabled = value,
            SetGain(gain) => self.camera_params.gain = gain,
            SetTime(time) => self.camera_params.time = time,
            SetRenderingType(rendering) => self.camera_params.rendering = rendering,
        }

        if let Some(camera) =  self.connected.as_mut() {
            camera.update_camera_params(self.camera_params.clone());
        }
    }

    pub fn exposure_command(&mut self, command: ExposureCommand) {
        match self.connected.as_mut() {
            None => self.set_detail("Not connected - cannot handle exposure command"),
            Some(connected) => match connected.exposure_command(command) {
                Ok(_) => {},
                Err(message) => self.set_detail(
                    &format!("Exposure command failed: {}", message)
                ),
            }
        }
    }

    pub fn update_storage_status(&mut self, message: StorageState) {
        self.storage_status = message;
    }
}

// =========================================== PRIVATE =============================================

impl CameraController {
    fn connection_state(&self) -> ConnectionState {
        match self.state {
            State::Error => ConnectionState::Connecting,
            State::Connected => ConnectionState::Established,
        }
    }

    fn set_detail(&mut self, detail: &str) {
        if detail != self.detail {
            info!("Detail updated: {}", detail);
        }

        self.detail = detail.to_owned();
    }

    fn handle_error_state(&mut self) -> State {
        if let Some(old_device) = self.connected.take() {
            old_device.close();
            self.set_detail("Closing old device");
        }

        match self.driver.list_devices() {
            Err(_) => {
                self.set_detail("Could not list devices");
                State::Error
            }
            Ok(devices) => match devices.as_slice() {
                [] => {
                    self.set_detail("No devices present in list");
                    State::Error
                }
                [device_id, ..] => self.connect_and_init(device_id)
            }
        }
    }

    fn connect_and_init(&mut self, id: &DeviceDescriptor) -> State {
        match self.driver.connect_device(id) {
            Err(_) => {
                self.set_detail("Connect device failed");
                State::Error
            },
            Ok(device) => {
                self.set_detail("Device connected, reading basic info");
                match ConnectedCameraController::new(device, self.process_tx.clone()) {
                    Ok(connected) => {
                        self.set_detail("Camera initialized");
                        self.connected = Some(connected);
                        State::Connected
                    },
                    Err(message) => {
                        self.set_detail(&format!("Init failed: {}", message));
                        self.connected = None;
                        State::Error
                    }
                }
            }
        }
    }

    fn handle_connected_state(&mut self) -> State {
        if let Some(ref mut controller) = self.connected {
            match controller.periodic() {
                Ok(_) => {
                    State::Connected
                },
                Err(message) => {
                    self.set_detail(&format!("Periodic task failed: {}", message));
                    self.connected = None;
                    State::Error
                }
            }
        } else {
            State::Error
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Error,
    Connected
}