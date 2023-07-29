use std::{cmp::{min, max}};

use ccdi_common::{RgbImage};
use nanocv::Img;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug)]
pub struct ImageStats {
    pub total: ChannelStats,
    pub r: ChannelStats,
    pub g: ChannelStats,
    pub b: ChannelStats,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChannelStats {
    pub min: u16,
    pub max: u16,
}

pub fn compute_image_stats(image: &RgbImage<u16>) -> ImageStats {
    let r = compute_channel_stats(image.red());
    let g = compute_channel_stats(image.green());
    let b = compute_channel_stats(image.blue());
    let total = combine_stats(&r, &combine_stats(&b, &g));
    ImageStats { r, g, b, total }
}

// =========================================== PRIVATE =============================================

fn compute_channel_stats(channel: &dyn Img<u16>,) -> ChannelStats {
    let mut min_value = u16::MAX;
    let mut max_value = u16::MIN;

    for line in 0..channel.size().y {
        for pixel in channel.line_ref(line) {
            min_value = min(min_value, *pixel);
            max_value = max(min_value, *pixel);
        }
    }

    ChannelStats { min: min_value, max: max_value }
}

fn combine_stats(first: &ChannelStats, second: &ChannelStats) -> ChannelStats {
    ChannelStats {
        min: min(first.min, second.min),
        max: max(first.max, second.max),
    }
}