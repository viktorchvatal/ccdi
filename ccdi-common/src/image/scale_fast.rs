use nanocv::{ImgSize, ImgBuf, ImgMut};

use crate::{RawImage, RgbImage};

// ============================================ PUBLIC =============================================

pub fn debayer_scale_fast(input: &RawImage, size: ImgSize) -> RgbImage<u16> {
    let r = resize_channel(input, size, 1, 0);
    let g = resize_channel(input, size, 1, 1);
    let b = resize_channel(input, size, 0, 1);
    RgbImage::from(r, g, b).expect("Logic error")
}

// =========================================== PRIVATE =============================================

pub fn resize_channel(
    image: &RawImage,
    size: ImgSize,
    offset_x: usize,
    offset_y: usize,
) -> ImgBuf<u16> {
    let (w, h) = (image.params.area.width, image.params.area.height);
    let x_indices = scale_index_table(w, size.x, offset_x);
    let y_indices = scale_index_table(h, size.y, offset_y);
    let mut result = ImgBuf::<u16>::new_init(size, Default::default());

    for line in 0..size.y {
        let dst = result.line_mut(line);
        let input_line = y_indices[line];
        let src = &image.data[input_line*w .. (input_line + 1)*w];

        for x in 0..size.x {
            dst[x] = src[x_indices[x]];
        }
    }

    result
}

fn scale_index_table(
    source_size: usize,
    target_size: usize,
    offset: usize,
) -> Vec<usize> {
    (0..target_size)
        .map(|x| {
            let half = x*source_size/2/target_size;
            half*2 + offset
        })
        .collect()
}