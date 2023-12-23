// ============================================ PUBLIC =============================================

use ccdi_common::to_string;
use fitsio::{hdu::{FitsHdu, HduInfo}, FitsFile};
use log::info;

pub fn read_channels(file: &mut FitsFile) -> Result<Channels, String> {
    let hdu = file.primary_hdu().map_err(to_string)?;
    let dimensions = read_dimensions(&hdu)?;

    if dimensions.channels != 3 {
        return Err(format!("Cannot read RGB channels, number of channels: {}", dimensions.channels))
    }

    let channel_size = dimensions.width*dimensions.height;

    Ok(Channels {
        dimensions,
        r: hdu.read_section(file, 0, channel_size).map_err(to_string)?,
        g: hdu.read_section(file, channel_size, 2*channel_size).map_err(to_string)?,
        b: hdu.read_section(file, 2*channel_size, 3*channel_size).map_err(to_string)?,
    })
}

pub fn save_f32_fits_file(channels: Channels, path: &str) -> Result<(), String> {
    use fitsio::images::{ImageDescription, ImageType};

    let description = ImageDescription {
        data_type: ImageType::Float,
        dimensions: &channels.dimensions.as_array(),
    };

    let mut fitsfile = FitsFile::create(&path)
        .with_custom_primary(&description)
        .overwrite()
        .open()
        .map_err(to_string)?;

    let hdu = fitsfile.primary_hdu().map_err(to_string)?;

    let all_data: Vec<f32> = channels.r.into_iter()
        .chain(channels.g.into_iter())
        .chain(channels.b.into_iter())
        .collect();

    hdu.write_image(&mut fitsfile, &all_data).map_err(to_string)?;

    Ok(())
}

// =========================================== PRIVATE =============================================

fn read_dimensions(hdu: &FitsHdu) -> Result<Dimensions, String> {
    match &hdu.info {
        HduInfo::ImageInfo { shape, image_type: _ } => {
            match shape.as_slice() {
                [channels, height, width] => Ok(
                    Dimensions { channels: *channels, width: *width, height: *height},
                ),
                other => Err(
                    format!("Invalid FITS dimensions: {other:?}"),
                )
            }
        },
        _other => Err(format!("Invalid HDU info.")),
    }
}

#[derive(Clone, Debug)]
pub struct Channels {
    pub dimensions: Dimensions,
    pub r: Vec<f32>,
    pub g: Vec<f32>,
    pub b: Vec<f32>,
}

#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    pub channels: usize,
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn as_array(&self) -> [usize; 3] {
        [self.channels, self.height, self.width]
    }
}