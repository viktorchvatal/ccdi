use nanocv::ImgMut;

use crate::RgbImage;

// ============================================ PUBLIC =============================================

pub fn draw_thirds_grid(image: &mut RgbImage<u16>) {
    let width_third = image.width()/3;
    let height_third = image.height()/3;

    for channel in image.channels_mut() {
        vertical_line(channel, width_third, u16::MAX);
        vertical_line(channel, width_third*2, u16::MAX);
        horizontal_line(channel, height_third, u16::MAX);
        horizontal_line(channel, height_third*2, u16::MAX);
    }
}

// =========================================== PRIVATE =============================================

fn horizontal_line(image: &mut dyn ImgMut<u16>, position: usize, value: u16) {
    if position < image.height() {
        let line = image.line_mut(position);

        for pixel in line.iter_mut() {
            *pixel = value;
        }
    }
}

fn vertical_line(image: &mut dyn ImgMut<u16>, position: usize, value: u16) {
    for line_index in 0..image.height() {
        let line = image.line_mut(line_index);

        if let Some(pixel) = line.get_mut(position) {
            *pixel = value;
        }
    }
}