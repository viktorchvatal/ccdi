use crate::CameraError;
use crate::api::*;
use std::os::raw::c_int;

extern {
    fn enumerate_usb_callback(arg1: c_int);
    fn get_last_camera_id() -> c_int;
    fn reset_last_camera_id();
}

pub fn read_raw_camera_id() -> i32 {
    unsafe {
        reset_last_camera_id();
        gxccd_enumerate_usb(Some(enumerate_usb_callback));
        get_last_camera_id()
    }
}

#[macro_export]
macro_rules! read_float_value_fn {
    ($name: ident, $register: expr) => {
        pub fn $name(&self) -> Result<f32, CameraError> {
            read_float_value(self.camera_ptr, $register)
        }
    };
}

pub fn read_float_value(
    camera_ptr: *mut camera_t,
    register: u32
) -> Result<f32, CameraError> {
    let mut value: f32 = 0.0;

    let code = unsafe {
        gxccd_get_value(camera_ptr, register as c_int, &mut value)
    };

    match code {
        0 => Ok(value),
        _other => Err(CameraError::Unspecified),
    }
}

#[macro_export]
macro_rules! read_int_value_fn {
    ($name: ident, $register: expr) => {
        pub fn $name(&self) -> Result<i32, CameraError> {
            read_int_value(self.camera_ptr, $register)
        }
    };
}

pub fn read_int_value(
    camera_ptr: *mut camera_t,
    register: u32
) -> Result<i32, CameraError> {
    let mut value: c_int = 0;

    let code = unsafe {
        gxccd_get_integer_parameter(camera_ptr, register as c_int, &mut value)
    };

    match code {
        0 => Ok(value),
        _other => Err(CameraError::Unspecified),
    }
}