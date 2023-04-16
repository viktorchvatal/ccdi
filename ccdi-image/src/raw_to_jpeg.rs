use std::{io::Cursor, cmp::min};

use ccdi_common::RawImage;
use image::imageops::{flip_vertical_in_place};

// ============================================ PUBLIC =============================================

pub fn simple_raw_image_to_jpeg(image: &RawImage, gain: u32) -> Result<Vec<u8>, String> {
    const SCALE: usize = 4;
    let width = image.params.area.width/SCALE;
    let height = image.params.area.height/SCALE;

    let mut dynamic = image::DynamicImage::new_rgb8(width as u32, height as u32);
    let data = &image.data[..];

    if let Some(ref mut gray) = dynamic.as_mut_rgb8() {
        for (x, y, pixel) in gray.enumerate_pixels_mut() {

            let y_offset_1: usize = (width*SCALE)*(y as usize*SCALE);
            let y_offset_2: usize = (width*SCALE)*(y as usize*SCALE + 1);
            let x_offset: usize = x as usize*SCALE;

            *pixel = image::Rgb([
                to_8bit(gain*data[y_offset_1 + x_offset + 1] as u32),
                to_8bit(gain*data[y_offset_2 + x_offset + 1] as u32),
                to_8bit(gain*data[y_offset_2 + x_offset] as u32),
            ]);
        }
    } else {
        return Err(format!("Cannot convert to rgb 8 image"));
    }

    flip_vertical_in_place(&mut dynamic);

    let mut cursor = Cursor::new(Vec::<u8>::new());

    match dynamic.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(95)) {
        Ok(_) => Ok(cursor.into_inner()),
        Err(err) => Err(format!("{:?}", err))
    }
}

fn to_8bit(input: u32) -> u8 {
    min(255, input >> 8) as u8
}