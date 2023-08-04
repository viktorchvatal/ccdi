mod raw_to_bmp;
mod stats;
mod plot;

pub use raw_to_bmp::{rgb_image_to_bmp, Transform, TransformFunction};
pub use plot::render_histogram_as_bmp;
pub use stats::*;