use std::{path::{PathBuf, Path}, fs::File, io::{BufReader, Read, BufWriter, Write}, sync::Arc};
use nanocv::ImgSize;
use serde_derive::{Serialize, Deserialize};

use ccdi_common::{to_string, GuiConfig};
use directories::ProjectDirs;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub storage: String,
    pub render_size: ImgSize,
    pub gui: GuiConfig,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            storage: String::from("~/storage/"),
            render_size: ImgSize::new(900, 600),
            gui: Default::default(),
        }
    }
}

pub fn load_config_file() -> Result<Arc<ServiceConfig>, String> {
    let path = config_file_path()?;

    serde_yaml::from_str::<ServiceConfig>(&load_text_file(path.as_path())?)
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

fn save_text_file(
    data: &str, path: &Path
) -> Result<(), String> {
    let prefix = path.parent().ok_or(format!("Invalid path parent"))?;
    std::fs::create_dir_all(prefix).map_err(to_string)?;
    let file = File::create(path).map_err(to_string)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data.as_bytes()).map_err(to_string)?;
    Ok(())
}

fn load_text_file(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(to_string)?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).map_err(to_string)?;
    Ok(data)
}

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
