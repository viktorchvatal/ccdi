use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{Utc, DateTime};

use ccdi_common::{RawImage, to_string};
use fitsio::FitsFile;
use fitsio::images::{ImageDescription, ImageType};

// ============================================ PUBLIC =============================================

pub fn save_fits_file(image: &RawImage, file_name: &str) -> Result<(), String> {
    let path = PathBuf::from(file_name);
    let prefix = path.parent().ok_or(format!("Invalid path parent"))?;
    std::fs::create_dir_all(prefix).map_err(to_string)?;

    let description = ImageDescription {
        data_type: ImageType::UnsignedShort,
        dimensions: &[image.params.area.height, image.params.area.width],
    };

    let mut fitsfile = FitsFile::create(&file_name)
        .with_custom_primary(&description)
        .overwrite()
        .open()
        .map_err(to_string)?;

    let hdu = fitsfile.primary_hdu().map_err(to_string)?;

    let date_obs = format_iso8601(image.params.start_time);

    hdu.write_key(&mut fitsfile, "DATE-OBS", date_obs).map_err(to_string)?;
    hdu.write_key(&mut fitsfile, "EXPTIME", image.params.time.to_string()).map_err(to_string)?;
    hdu.write_image(&mut fitsfile, &image.data).map_err(to_string)?;

    Ok(())
}

// =========================================== PRIVATE =============================================

fn format_iso8601(time: SystemTime) -> String {
    let chrono_time: DateTime<Utc> = time.into();
    format!("{}", chrono_time.format("%+"))
}