mod api;
mod read;
mod read_mode;

use crate::api::*;
use read::*;
use read_mode::enumerate_read_modes;

// ============================================ PUBLIC =============================================

pub struct CameraDriver {
    camera_ptr: *mut camera_t
}

/// Low level camera driver
#[derive(Debug)]
pub enum CameraError {
    Unspecified,
    UnableToConvertCString,
}

pub fn get_any_camera_id() -> Option<i32> {
    let raw_id = read_raw_camera_id();
    if raw_id < 0 { None } else { Some(raw_id) }
}

pub fn connect_usb_camera(id: i32) -> Result<CameraDriver, CameraError> {
    let camera_ptr = unsafe { gxccd_initialize_usb(id) };

    match camera_ptr.is_null() {
        true => Err(CameraError::Unspecified),
        false => Ok(CameraDriver { camera_ptr })
    }
}

impl CameraDriver {
    read_float_value_fn!(read_chip_temperature, GV_CHIP_TEMPERATURE);
    read_float_value_fn!(read_hot_temperature, GV_HOT_TEMPERATURE);
    read_float_value_fn!(read_camera_temperature, GV_CAMERA_TEMPERATURE);
    read_float_value_fn!(read_environment_temperature, GV_ENVIRONMENT_TEMPERATURE);
    read_float_value_fn!(read_supply_voltage, GV_SUPPLY_VOLTAGE);
    read_float_value_fn!(read_power_utilization, GV_POWER_UTILIZATION);
    read_float_value_fn!(read_adc_gain, GV_ADC_GAIN);

    read_int_value_fn!(read_camera_id, GIP_CAMERA_ID);
    read_int_value_fn!(read_chip_width, GIP_CHIP_W);
    read_int_value_fn!(read_chip_height, GIP_CHIP_D);
    read_int_value_fn!(read_min_exposure, GIP_MINIMAL_EXPOSURE);
    read_int_value_fn!(read_max_exposure, GIP_MAXIMAL_EXPOSURE);
    read_int_value_fn!(read_max_gain, GIP_MAX_GAIN);

    pub fn enumerate_read_modes(&self) -> Result<Vec<String>, CameraError> {
        enumerate_read_modes(self.camera_ptr)
    }
}
