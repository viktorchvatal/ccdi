use std::path::PathBuf;

use ccdi_common::{RawImage, to_string};
use fitsio::FitsFile;
use fitsio::images::{ImageDescription, ImageType};

// ============================================ PUBLIC =============================================

pub fn save_fits_file(image: &RawImage, file_name: &str) -> Result<(), String> {
    let path = PathBuf::from(file_name.clone());
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
    hdu.write_image(&mut fitsfile, &image.data).map_err(to_string)?;

    Ok(())
}