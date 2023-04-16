use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

pub trait ImagerDriver {
    fn list_devices(&mut self) -> Result<Vec<DeviceDescriptor>, String>;
    fn connect_device(&mut self, descriptor: &DeviceDescriptor) -> Result<Box<dyn ImagerDevice>, String>;
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DeviceDescriptor {
    pub id: i32,
    pub name: String,
}

pub trait ImagerDevice {
    fn read_properties(&mut self) -> Result<ImagerProperties, String>;
    fn close(&mut self);
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ImagerProperties {
    pub basic: BasicProperties,
    pub other: Vec<DeviceProperty>,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct BasicProperties {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DeviceProperty {
    pub name: String,
    pub value: String,
}