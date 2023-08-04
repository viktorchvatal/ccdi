use std::process::Command;

use log::{info, warn};

// ============================================ PUBLIC =============================================

pub fn execute_command(command: &str) {
    match Command::new("sh").args(["-c", command]).spawn() {
        Ok(_) => info!("Command executed."),
        Err(error) => warn!("Command failed {:?}", error),
    }
}