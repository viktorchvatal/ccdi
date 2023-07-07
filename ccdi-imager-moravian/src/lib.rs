use std::fmt::Debug;

use ccdi_common::to_string;
use ccdi_driver_moravian::{get_any_camera_id, CameraDriver, connect_usb_camera, CameraError};
use ccdi_imager_interface::{
    ImagerDriver, ImagerDevice, ImagerProperties, DeviceDescriptor, DeviceProperty, BasicProperties, ExposureParams, TemperatureRequest
};

// ============================================ PUBLIC =============================================

pub struct MoravianImagerDriver {

}

impl MoravianImagerDriver {
    pub fn new() -> Self {
        Self { }
    }
}

impl ImagerDriver for MoravianImagerDriver {
    fn list_devices(&mut self) -> Result<Vec<DeviceDescriptor>, String> {
        Ok(match get_any_camera_id() {
            Some(id) => vec![
                DeviceDescriptor { id, name: String::from("Camera #0") }
            ],
            None => vec![],
        })
    }

    fn connect_device(
        &mut self,
        descriptor: &DeviceDescriptor
    ) ->  Result<Box<dyn ImagerDevice>, String> {
        Ok(Box::new(
            MoravianImagerDevice {
                device: connect_usb_camera(descriptor.id).map_err(to_string)?
            }
        ))
    }
}

pub struct MoravianImagerDevice {
    device: CameraDriver
}

impl ImagerDevice for MoravianImagerDevice {
    fn read_properties(&mut self) -> Result<ImagerProperties, String> {
        Ok(ImagerProperties {
            basic: read_basic_properties(&self.device).map_err(to_string)?,
            other: read_all_properties(&self.device).map_err(to_string)?
        })
    }

    fn close(&mut self) {

    }

    fn start_exposure(&mut self, params: &ExposureParams) -> Result<(), String> {
        self.device.set_gain(params.gain).map_err(to_string)?;
        let (x, y, w, h) = params.area.into_tuple();
        self.device.start_exposure(params.time, true, x, y, w, h).map_err(to_string)
    }

    fn image_ready(&mut self, ) -> Result<bool, String> {
        self.device.image_ready().map_err(to_string)
    }

    fn download_image(&mut self, params: &ExposureParams) -> Result<Vec<u16>, String> {
        self.device.read_image(params.area.pixel_count()).map_err(to_string)
    }

    fn set_temperature(&mut self, request: TemperatureRequest) -> Result<(), String> {
        self.device.set_temperature_ramp(request.speed).map_err(to_string)?;
        self.device.set_temperature(request.temperature).map_err(to_string)
    }
}

fn read_basic_properties(device: &CameraDriver) -> Result<BasicProperties, CameraError> {
    Ok(BasicProperties{
        width: device.read_chip_width()? as usize,
        height: device.read_chip_height()? as usize,
        temperature: device.read_camera_temperature()?,
    })
}

fn read_all_properties(device: &CameraDriver) -> Result<Vec<DeviceProperty>, CameraError> {
    Ok(vec![
        prop_f32("Chip Temperature", device.read_chip_temperature()?, 2),
        prop_f32("Hot Temperature", device.read_hot_temperature()?, 2),
        prop_f32("Env Temperature", device.read_environment_temperature()?, 2),
        prop_f32("Supply Voltage", device.read_supply_voltage()?, 2),
        prop("Power Utilization", device.read_power_utilization()?),
        prop("ADC Gain", device.read_adc_gain()?),
        prop("Camera ID", device.read_camera_id()?),
        prop("Min Exposure Time", device.read_min_exposure()?),
        prop("Max Exposure Time", device.read_max_exposure()?),
        prop("Max Gain", device.read_max_gain()?),
    ])
}

fn prop<T: Debug>(name: &str, value: T) -> DeviceProperty {
    DeviceProperty {
        name: name.to_owned(),
        value: format!("{:?}", value)
    }
}

fn prop_f32(name: &str, value: f32, precision: usize) -> DeviceProperty {
    DeviceProperty {
        name: name.to_owned(),
        value: format!("{:0.prec$}", value, prec = precision)
    }
}