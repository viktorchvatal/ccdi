
use ccdi_common::ExposureCommand;
use ccdi_imager_interface::BasicProperties;

// ============================================ PUBLIC =============================================

pub struct ExposureController {
    properties: BasicProperties,
    gain: i32,
    time: f64,
    state: State
}

impl ExposureController {
    pub fn new(properties: BasicProperties) -> Self {
        Self {
            properties,
            gain: 0,
            time: 1.0,
            state: State::Idle
        }
    }

    pub fn exposure_command(&mut self, command: ExposureCommand) {
        ::log::info!("Exposure demand")
    }
}

// =========================================== PRIVATE =============================================


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Idle,
    Exposing
}