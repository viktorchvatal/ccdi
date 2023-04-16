use std::{thread, time::Duration};

use ccdi_common::{to_string, log_err};
use ccdi_driver_moravian::{get_any_camera_id, connect_usb_camera, CameraError, CameraDriver};
use fitsio::FitsFile;


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

    for (index, mode) in camera.enumerate_read_modes()?.iter().enumerate() {
        println!("Read mode {}: {}", index, mode)
    }

    let width = camera.read_chip_width()?;
    let height = camera.read_chip_height()?;

    camera.start_exposure(0.1, true, 0, 0, width as usize, height as usize)?;

    while !(camera.image_ready()?) {
        println!("Image not ready, waiting ...");
        thread::sleep(Duration::from_millis(100));
    }

    println!("Starting image download");
    let image_data = camera.read_image((width*height) as usize)?;
    println!("Image downloaded, pixels: {}", image_data.len());

    log_err(
        "Save FITS",
        save_fits_file(image_data, width as usize, height as usize, "test.fits")
    );

    Ok(())
}

fn save_fits_file(data: Vec<u16>, width: usize, height: usize, path: &str) -> Result<(), String> {
    use fitsio::images::{ImageDescription, ImageType};

    let description = ImageDescription {
        data_type: ImageType::Short,
        dimensions: &[height, width],
    };

    let mut fitsfile = FitsFile::create(&path)
        .with_custom_primary(&description)
        .overwrite()
        .open()
        .map_err(to_string)?;

    let hdu = fitsfile.primary_hdu().map_err(to_string)?;
    hdu.write_image(&mut fitsfile, &data).map_err(to_string)?;

    Ok(())
}