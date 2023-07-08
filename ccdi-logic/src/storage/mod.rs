use std::{sync::Arc, process::Command, path::PathBuf};

use ccdi_common::{StorageMessage, StateMessage, StorageState, StorageDetails, to_string};
use log::info;

use crate::ServiceConfig;

// ============================================ PUBLIC =============================================

pub struct Storage {
    config: Arc<ServiceConfig>,
    last_storage_state: StorageState,
    counter: usize,
    storage_name: Option<String>,
}

impl Storage {
    pub fn new(config: Arc<ServiceConfig>) -> Self {
        Self {
            config,
            last_storage_state: StorageState::Unknown,
            counter: 0,
            storage_name: None,
        }
    }

    pub fn process(&mut self, message: StorageMessage) -> Result<Vec<StateMessage>, String> {
        match message {
            StorageMessage::EnableStore(name) => {
                self.storage_name = Some(name);
                self.counter = 0;
            },
            StorageMessage::DisableStore => {
                self.storage_name = None;
                self.counter = 0;
            },
            StorageMessage::ProcessImage(image) => {
                if let Some(dir) = self.current_dir() {
                    info!("Store: Dir: '{dir}' Id: {}", self.counter);
                    self.counter += 1;
                }
            }
        };

        Ok(vec![])
    }

    pub fn periodic_tasks(&mut self) -> Result<Vec<StateMessage>, String> {
        let storage_state = check_storage(&self.config.storage);

        if storage_state == self.last_storage_state {
            Ok(vec![])
        } else {
            self.last_storage_state = storage_state.clone();
            Ok(vec![StateMessage::UpdateStorageState(storage_state)])
        }
    }
}

// =========================================== PRIVATE =============================================

impl Storage {
    fn current_dir(&self) -> Option<String> {
        self.storage_name
            .clone()
            .and_then(
                |dir| PathBuf::from(self.config.storage.clone())
                    .join(PathBuf::from(dir))
                    .to_str()
                    .map(|path| path.to_owned())
            )
    }
}

fn check_storage(path: &str) -> StorageState {
    match Command::new("df").args([path]).output() {
        Ok(output) => match output.status.code() {
            Some(0) => match String::from_utf8(output.stdout) {
                Ok(stdout) => match parse_free_space(&stdout) {
                    Ok(details) => StorageState::Available(details),
                    Err(error) => StorageState::Error(error),
                },
                Err(error) => StorageState::Error(
                    format!("Could not parse stdout as utf8: {:?}", error)
                ),
            },
            Some(code) => StorageState::Error(
                format!(
                    "Storage check returned error code: {:?} {:?}",
                    code, String::from_utf8_lossy(&output.stderr)
                )
            ),
            status => StorageState::Error(
                format!("Storage check did not return successfully: {:?}", status)
            )
        },
        Err(error) => StorageState::Error(
            format!("Storage check call failed: {:?}", error)
        ),
    }
}

fn parse_free_space(stdout: &str) -> Result<StorageDetails, String> {
    let line = stdout.lines().nth(1).ok_or("df output second line missing")?;
    let total_gigabytes = kb_to_gb(parse_nth_token(line, 1)?);
    let free_gigabytes = kb_to_gb(parse_nth_token(line, 3)?);
    Ok(StorageDetails{total_gigabytes, free_gigabytes})
}

fn parse_nth_token(line: &str, index: usize) -> Result<f64, String> {
    let token = line.split_whitespace()
        .nth(index)
        .ok_or(format!("{}th token not present in '{}'", index, line))?;

    token.parse::<f64>().map_err(to_string)
}

fn kb_to_gb(kilobytes: f64) -> f64 {
    kilobytes/1024.0/1024.0
}

// ============================================= TEST ==============================================

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DF_OUTPUT: &str = indoc!{"
        Filesystem           1K-blocks  Used      Available  Use% Mounted on
        /dev/mapper/luks-a6e 1967861712 111750632 1756075448   6% /media/x/759
    "};

    #[test]
    fn parse_df_output() {
        let details = parse_free_space(TEST_DF_OUTPUT).expect("Parse details failed");
        assert_eq!(details.total_gigabytes, 1967861712.0/1024.0/1024.0);
        assert_eq!(details.free_gigabytes, 1756075448.0/1024.0/1024.0);
    }
}