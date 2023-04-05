use ccdi_driver_moravian::{get_any_camera_id, connect_usb_camera, CameraError, Camera};


fn main() -> Result<(), String> {
    let camera_id = get_any_camera_id().ok_or("No camera connected")?;

    if let Ok(camera) = connect_usb_camera(camera_id) {
        print_camera_info(&camera).map_err(|err| format!("{:?}", err))?;
    }

    dbg!(camera_id);
    Ok(())
}

fn print_camera_info(camera: &Camera) -> Result<(), CameraError> {
    if let Ok(value) = camera.chip_temperature() {
        println!("Chip temperature: {}", value);
    }

    Ok(())
}