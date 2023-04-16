// ============================================ PUBLIC =============================================

use std::{sync::Arc, time::Instant};

use ccdi_common::ConnectionState;
use ccdi_imager_interface::{ImagerDriver, ImagerDevice, ImagerProperties};
use log::{info};

pub struct CameraController {
    driver: Box<dyn ImagerDriver>,
    device: Option<Box<dyn ImagerDevice>>,
    state: State,
    properties: Option<Arc<ImagerProperties>>,
    detail: String,
    last_properties_read: Option<Instant>
}

impl CameraController {
    pub fn new(driver: Box<dyn ImagerDriver>) -> Self {
        Self {
            driver,
            device: None,
            state: State::Error,
            properties: None,
            detail: String::from("Started"),
            last_properties_read: None,
        }
    }

    pub fn periodic(&mut self) {
        let old_state = self.state;

        self.state = match self.state {
            State::Error => self.handle_error_state(),
            State::Connected => self.handle_connected_state(),
        };

        if self.state != old_state {
            info!("Camera state {:?} -> {:?}", old_state, self.state);
        }
    }

    pub fn properties(&self) -> Option<Arc<ImagerProperties>> {
        self.properties.clone()
    }

    pub fn detail(&self) -> String {
        self.detail.clone()
    }

    pub fn connection_state(&self) -> ConnectionState {
        match self.state {
            State::Error => ConnectionState::Connecting,
            State::Connected => ConnectionState::Established,
        }
    }
}

// =========================================== PRIVATE =============================================

const PROPERTIES_READ_INTERVAL: f64 = 2.0; // Seconds

impl CameraController {
    fn set_detail(&mut self, detail: &str) {
        info!("Detail updated: {}", detail);
        self.detail = detail.to_owned();
    }

    fn handle_error_state(&mut self) -> State {
        if let Some(mut old_device) = self.device.take() {
            self.set_detail("Closing old device");
            old_device.close();
            self.last_properties_read = None;
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
                [device_id, ..] => {
                    match self.driver.connect_device(device_id) {
                        Err(_) => {
                            self.set_detail("Connect device failed");
                            State::Error
                        },
                        Ok(device) => {
                            self.set_detail("Device connected");
                            self.device = Some(device);
                            State::Connected
                        }
                    }
                }
            }
        }
    }

    fn handle_connected_state(&mut self) -> State {
        if self.should_read_properties() {
            self.connected_read_properties()
        } else {
            State::Connected
        }
    }

    fn connected_read_properties(&mut self) -> State {
        if let Some(ref mut device) = self.device {
            match device.read_properties() {
                Ok(properties) => {
                    self.properties = Some(Arc::new(properties));
                    self.set_detail("Camera properties loaded");
                    self.last_properties_read = Some(Instant::now());
                    State::Connected
                },
                Err(_) => {
                    self.set_detail("Failed to read properties");
                    State::Error
                }
            }
        } else {
            State::Error
        }
    }

    fn should_read_properties(&self) -> bool {
        match self.last_properties_read {
            None => true,
            Some(last_time) => {
                last_time.elapsed().as_secs_f64() >= PROPERTIES_READ_INTERVAL
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Error,
    Connected
}