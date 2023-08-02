mod api;
mod read;
mod read_mode;
mod image;

use std::ffi::c_void;

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

    pub fn start_exposure(
        &self, time: f64, use_shutter: bool, x: usize, y: usize, w: usize, h: usize
    ) -> Result<(), CameraError> {
        let result = unsafe {
            gxccd_start_exposure(
                self.camera_ptr,  time, use_shutter, x as i32, y as i32, w as i32, h as i32
            )
        };

        if result == 0 { Ok(()) } else { Err(CameraError::Unspecified) }
    }

    pub fn image_ready(&self) -> Result<bool, CameraError> {
        let mut image_ready: [bool; 1] = [false; 1];
        let result = unsafe { gxccd_image_ready(self.camera_ptr, image_ready.as_mut_ptr()) };
        if result == 0 { Ok(image_ready[0]) } else { Err(CameraError::Unspecified) }
    }

    pub fn read_image(&self, pixel_count: usize) -> Result<Vec<u16>, CameraError> {
        let mut buffer = vec![0u16; pixel_count];

        let result = unsafe {
            gxccd_read_image(
                self.camera_ptr, buffer.as_mut_ptr() as *mut c_void, 2*buffer.len()
            )
        };
        if result == 0 { Ok(buffer) } else { Err(CameraError::Unspecified) }
    }

    pub fn disconnect(self) {
        unsafe { gxccd_release(self.camera_ptr); }
    }

    pub fn set_gain(&self, gain: u16) -> Result<(), CameraError> {
        convert_simple(unsafe { gxccd_set_gain(self.camera_ptr, gain) })
    }

    pub fn set_temperature(&self, temperature: f32) -> Result<(), CameraError> {
        convert_simple(unsafe { gxccd_set_temperature(self.camera_ptr, temperature) })
    }

    pub fn set_temperature_ramp(&self, deg_per_minute: f32) -> Result<(), CameraError> {
        convert_simple(unsafe { gxccd_set_temperature_ramp(self.camera_ptr, deg_per_minute) })
    }
}

fn convert_simple(result: i32) -> Result<(), CameraError> {
    convert_result((), result)
}

fn convert_result<T>(value: T, result: i32) -> Result<T, CameraError> {
    if result == 0 { Ok(value) } else { Err(CameraError::Unspecified) }
}
