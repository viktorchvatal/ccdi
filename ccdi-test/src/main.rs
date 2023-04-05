use ccdi_driver_moravian::{get_any_camera_id, connect_usb_camera, CameraError, CameraDriver};


fn main() -> Result<(), String> {
    let camera_id = get_any_camera_id().ok_or("No camera connected")?;

    if let Ok(camera) = connect_usb_camera(camera_id) {
        print_camera_info(&camera).map_err(|err| format!("{:?}", err))?;
    }

    dbg!(camera_id);
    Ok(())
}

fn print_camera_info(camera: &CameraDriver) -> Result<(), CameraError> {
    println!("Chip temperature: {}", camera.read_chip_temperature()?);
    println!("Supply voltage: {}", camera.read_supply_voltage()?);
    println!("Resolution: {} x {}", camera.read_chip_width()?, camera.read_chip_height()?);

    Ok(())
}