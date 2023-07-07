use std::{fmt::Debug};

use ccdi_imager_interface::{
    ImagerDriver, ImagerDevice, ImagerProperties, DeviceDescriptor, DeviceProperty, BasicProperties, ExposureParams, TemperatureRequest
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
        Ok(Box::new(DemoImagerDevice { offset: 0.0, temperature: 30.0 }))
    }
}

pub struct DemoImagerDevice {
    offset: f32,
    temperature: f32,
}

impl ImagerDevice for DemoImagerDevice {
    fn read_properties(&mut self) -> Result<ImagerProperties, String> {
        self.offset += 0.001;
        Ok(ImagerProperties {
            basic: BasicProperties {
                width: 3000,
                height: 2000,
                temperature: self.temperature
            },
            other: list_demo_properties(&self)
        })
    }

    fn close(&mut self) {

    }

    fn start_exposure(&mut self, _params: &ExposureParams) -> Result<(), String> {
        Ok(())
    }

    fn image_ready(&mut self, ) -> Result<bool, String> {
        Ok(true)
    }

    fn download_image(&mut self, params: &ExposureParams) -> Result<Vec<u16>, String> {
        Ok(
            (0..(params.area.pixel_count()))
                .map(|x| ((x/30) % 12000) as u16)
                .collect()
        )
    }

    fn set_temperature(&mut self, request: TemperatureRequest) -> Result<(), String> {
        dbg!("Setting temperature: ", request.temperature, request.speed);
        self.temperature = request.temperature;
        Ok(())
    }
}

fn list_demo_properties(device: &DemoImagerDevice) -> Vec<DeviceProperty> {
    vec![
        prop("Chip Temperature", 1.000 + device.offset),
        prop("Hot Temperature", 2.000 + device.offset),
        prop("Camera Temperature", 3.000 + device.offset),
        prop("Env Temperature", 4.000 + device.offset),
        prop("Supply Voltage", 5.000 + device.offset),
        prop("Power Utilization", 6.000 + device.offset),
        prop("ADC Gain", 7.000 + device.offset),
        prop("Camera ID", 8.000 + device.offset),
        prop("Camera Chip Width", 9.000 + device.offset),
        prop("Camera Chip Height", 10.000 + device.offset),
        prop("Min Exposure Time", 11.000 + device.offset),
        prop("Max Exposure Time", 12.000 + device.offset),
        prop("Max Gain", 13.000 + device.offset),
    ]
}

fn prop<T: Debug>(name: &str, value: T) -> DeviceProperty {
    DeviceProperty {
        name: name.to_owned(),
        value: format!("{:?}", value)
    }
}