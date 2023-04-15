use std::fmt::Debug;

use ccdi_imager_interface::{
    ImagerDriver, ImagerDevice, ImagerProperties, DeviceDescriptor, DeviceProperty
};

// ============================================ PUBLIC =============================================

pub struct DemoImagerDriver {

}

impl DemoImagerDriver {
    pub fn new() -> Self {
        Self { }
    }
}

impl ImagerDriver for DemoImagerDriver {
    fn list_devices(&mut self) -> Result<Vec<DeviceDescriptor>, String> {
        Ok(vec![
            DeviceDescriptor { id: 0, name: String::from("Demo Camera #0") }
        ])
    }

    fn connect_device(&mut self, _descriptor: &DeviceDescriptor) -> Result<Box<dyn ImagerDevice>, String> {
        Ok(Box::new(DemoImagerDevice { offset: 0.0 }))
    }
}

pub struct DemoImagerDevice {
    offset: f32
}

impl ImagerDevice for DemoImagerDevice {
    fn read_properties(&mut self) -> Result<ImagerProperties, String> {
        self.offset += 0.001;
        Ok(ImagerProperties {
            other: list_demo_properties(&self)
        })
    }

    fn close(&mut self) {

    }
}

fn list_demo_properties(device: &DemoImagerDevice) -> Vec<DeviceProperty> {
    vec![
        prop("Chip Temperature", 1.000 + device.offset),
        prop("Hot Temperature", 1.000 + device.offset),
        prop("Camera Temperature", 1.000 + device.offset),
        prop("Env Temperature", 1.000 + device.offset),
        prop("Supply Voltage", 1.000 + device.offset),
        prop("Power Utilization", 1.000 + device.offset),
        prop("ADC Gain", 1.000 + device.offset),
        prop("Camera ID", 1.000 + device.offset),
        prop("Camera Chip Width", 1.000 + device.offset),
        prop("Camera Chip Height", 1.000 + device.offset),
        prop("Min Exposure Time", 1.000 + device.offset),
        prop("Max Exposure Time", 1.000 + device.offset),
        prop("Max Gain", 1.000 + device.offset),
    ]
}

fn prop<T: Debug>(name: &str, value: T) -> DeviceProperty {
    DeviceProperty {
        name: name.to_owned(),
        value: format!("{:?}", value)
    }
}