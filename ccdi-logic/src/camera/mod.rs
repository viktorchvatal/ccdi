mod properties;
mod exposure;
mod connected;

use std::sync::Arc;

use ccdi_common::{ConnectionState, ViewState, LogicStatus, ExposureCommand, ClientMessage, RgbImage};
use ccdi_imager_interface::{ImagerDriver, DeviceDescriptor};
use log::{info};

use self::{connected::ConnectedCameraController};

// ============================================ PUBLIC =============================================

pub struct CameraController {
    driver: Box<dyn ImagerDriver>,
    state: State,
    detail: String,
    connected: Option<ConnectedCameraController>,
    view: Option<ViewState>,
    last_gain: u16,
    last_time: f64,
}

impl CameraController {
    pub fn new(driver: Box<dyn ImagerDriver>) -> Self {
        Self {
            driver,
            state: State::Error,
            connected: None,
            detail: String::from("Started"),
            view: None,
            last_gain: 0,
            last_time: 1.0,
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
                    .unwrap_or(ConnectionState::Disconnected)
            },
            camera_properties: self.connected.as_ref().map(|cam| cam.get_properties()),
            gain: self.last_gain,
            time: self.last_time,
        }
    }

    pub fn last_image(&self) -> Option<Arc<RgbImage<u16>>> {
        self.connected.as_ref().and_then(|camera| camera.last_image())
    }

    pub fn exposure_command(&mut self, command: ExposureCommand) {
        match command {
            ExposureCommand::SetGain(gain) => self.last_gain = gain,
            ExposureCommand::SetTime(time) => self.last_time = time,
            _ => {},
        }

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
                match ConnectedCameraController::new(device) {
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