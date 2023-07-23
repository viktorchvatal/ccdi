use std::{path::PathBuf, sync::Arc};
use nanocv::ImgSize;
use serde_derive::{Serialize, Deserialize};

use ccdi_common::{to_string, GuiConfig, save_text_file, read_text_file};
use directories::ProjectDirs;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub storage: String,
    pub render_size: ImgSize,
    pub gui: GuiConfig,
    pub io: IoConfig,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            storage: String::from("~/storage/"),
            render_size: ImgSize::new(900, 600),
            gui: Default::default(),
            io: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct IoConfig {
    pub trigger_input: String,
    pub trigger_status: String,
    pub heating_pwm: String,
    pub main_status: String,
}

impl Default for IoConfig {
    fn default() -> Self {
        Self {
            trigger_input: String::from("/sys/class/gpio/gpio17/value"),
            trigger_status: String::from("/sys/class/gpio/gpio2/value"),
            heating_pwm: String::from("/sys/class/gpio/gpio4/value"),
            main_status: String::from("/sys/class/gpio/gpio3/value")
        }
    }
}

pub fn load_config_file() -> Result<Arc<ServiceConfig>, String> {
    let path = config_file_path()?;

    serde_yaml::from_str::<ServiceConfig>(&read_text_file(path.as_path())?)
        .map_err(|err| format!("Could not load config file {}: {}", path_as_string(&path), err))
        .map(|config| Arc::new(config))
}

pub fn create_default_config_file() -> Result<String, String> {
    let config_json = serde_yaml::to_string(&<ServiceConfig as Default>::default())
        .map_err(to_string)?;

    let path = default_file_path()?;

    match save_text_file(&config_json, path.as_path()) {
        Ok(_) => Ok(path_as_string(&path)),
        Err(error) => Err(error)
    }
}

// =========================================== PRIVATE =============================================

fn path_as_string(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

fn config_file_path() -> Result<PathBuf, String> {
    create_file_path("config.yaml")
}

fn default_file_path() -> Result<PathBuf, String> {
    create_file_path("default.yaml")
}

fn create_file_path(file_name: &str) -> Result<PathBuf, String> {
    Ok(
        ProjectDirs::from("", "",  "ccdi")
            .ok_or(String::from("Could not determine config directory path"))?
            .config_dir()
            .to_path_buf()
            .join(file_name)
    )
}
