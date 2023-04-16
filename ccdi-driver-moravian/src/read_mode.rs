use std::{os::raw::c_char, ffi::{CStr}};
use super::*;

pub fn enumerate_read_modes(camera_ptr: *mut camera_t) -> Result<Vec<String>, CameraError> {
    const MAX_LEN: usize = 256;
    let mut result: Vec<String> = Vec::new();

    let mut index = 0;
    let mut read_result = 1;

    while read_result > 0 {
        let mut buffer: [c_char; MAX_LEN] = [0; MAX_LEN];

        read_result = unsafe {
            gxccd_enumerate_read_modes(
                camera_ptr,
                index,
                buffer.as_mut_ptr(),
                buffer.len() as u64
            )
        };

        index += 1;

        if read_result != 0 {
            let c_str = unsafe { CStr::from_ptr(buffer.as_ptr())};

            let rust_str = match c_str.to_str() {
                Ok(value) => value,
                Err(_) => return Err(CameraError::UnableToConvertCString)
            };

            result.push(rust_str.to_owned())
        }
    }

    Ok(result)
}