#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern {
    fn enumerate_usb_callback(arg1: c_int);
    fn get_last_camera_id() -> c_int;
    fn reset_last_camera_id();
}

pub struct Camera {
    camera_ptr: *mut camera_t
}

#[derive(Debug)]
pub enum CameraError {
    Unspecified
}

pub fn get_any_camera_id() -> Option<i32> {
    let raw_id = unsafe {
        reset_last_camera_id();
        gxccd_enumerate_usb(Some(enumerate_usb_callback));
        get_last_camera_id()
    };

    if raw_id < 0 { None } else { Some(raw_id) }
}

pub fn connect_usb_camera(id: i32) -> Result<Camera, CameraError> {
    let camera_ptr = unsafe { gxccd_initialize_usb(id) };

    match camera_ptr.is_null() {
        true => Err(CameraError::Unspecified),
        false => Ok(Camera { camera_ptr })
    }
}

impl Camera {
    pub fn chip_temperature(&self) -> Result<f32, CameraError> {
        self.get_value(GV_CHIP_TEMPERATURE)
    }

    fn get_value(&self, register: u32) -> Result<f32, CameraError> {
        let mut value: f32 = 0.0;

        let code = unsafe {
            gxccd_get_value(self.camera_ptr, register as c_int, &mut value)
        };

        match code {
            0 => Ok(value),
            _other => Err(CameraError::Unspecified),
        }
    }
}

