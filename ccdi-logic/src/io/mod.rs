use std::{sync::Arc, path::{PathBuf, Path}};

use ccdi_common::{IoMessage, StateMessage, read_text_file};
use log::debug;

use crate::{ServiceConfig, IoConfig};

// ============================================ PUBLIC =============================================

pub struct IoManager {
    last_trigger_value: Option<bool>,
    trigger_input_path: PathBuf,
    trigger_status_path: PathBuf,
    heating_pwm_path: PathBuf,
    main_status_path: PathBuf,
}

impl IoManager {
    pub fn new(config: &IoConfig) -> Self {
        Self {
            last_trigger_value: None,
            trigger_input_path: PathBuf::from(config.trigger_input.clone()),
            trigger_status_path: PathBuf::from(config.trigger_status.clone()),
            heating_pwm_path: PathBuf::from(config.heating_pwm.clone()),
            main_status_path: PathBuf::from(config.main_status.clone()),
        }
    }

    pub fn process(&mut self, message: IoMessage) -> Result<Vec<StateMessage>, String> {
        Ok(vec![])
    }

    pub fn periodic_tasks(&mut self) -> Result<Vec<StateMessage>, String> {
        let prev_input = self.last_trigger_value;
        let actual_input = read_input(&self.trigger_input_path);

        if actual_input.is_some() {
            self.last_trigger_value = actual_input;
        }

        let output = match (prev_input, self.last_trigger_value) {
            (Some(prev), Some(actual)) if prev != actual => vec![
                StateMessage::TriggerValueChanged(actual)
            ],
            (None, Some(actual)) => vec![
                StateMessage::TriggerValueChanged(actual)
            ],
            _ => vec![],
        };

        Ok(output)
    }
}

// =========================================== PRIVATE =============================================

fn read_input(path: &Path) -> Option<bool> {
    let first_char = read_text_file(path)
        .map(|string| string.chars().nth(0).unwrap_or(' '));

    match first_char {
        Err(_) => None,
        Ok('0') => Some(true),
        Ok('1') => Some(false),
        Ok(other) => {
            debug!("Invalid status value: {}", other);
            None
        }
    }
}