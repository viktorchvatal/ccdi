use std::cmp::{min, max};

use ccdi_common::RgbImage;
use nanocv::Img;

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug)]
pub struct ImageStats {
    pub total: ChannelStats,
    pub r: Histogram,
    pub g: Histogram,
    pub b: Histogram,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChannelStats {
    pub min: u16,
    pub max: u16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Histogram {
    pub bins: Vec<u32>,
    pub min: u16,
    pub max: u16,
}

impl Histogram {
    pub fn stats(&self) -> ChannelStats {
        ChannelStats { min: self.min, max: self.max }
    }

    pub fn max_count(&self) -> usize {
        self.bins.iter().cloned().max().unwrap_or(0) as usize
    }
}

impl ImageStats {
    pub fn min_pixel_value(&self) -> u16 {
        let (r, g, b) = (self.r.min, self.g.min, self.b.min);
        r.min(g.min(b))
    }

    pub fn abg_min_pixel_value(&self) -> u16 {
        let (r, g, b) = (self.r.min, self.g.min, self.b.min);
        (r + g + b)/3
    }
}

pub fn compute_image_stats(image: &RgbImage<u16>, size: usize) -> ImageStats {
    let r = compute_histogram(image.red(), size, compute_channel_stats(image.red()));
    let g = compute_histogram(image.green(), size, compute_channel_stats(image.green()));
    let b = compute_histogram(image.blue(), size, compute_channel_stats(image.blue()));
    let total = combine_stats(&r.stats(), &combine_stats(&b.stats(), &g.stats()));
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

fn compute_histogram(
    channel: &dyn Img<u16>,
    size: usize,
    stats: ChannelStats
) -> Histogram {
    let mut bins = vec![0; size];
    let divisor = max(1, stats.max - stats.min) as usize;

    for line in 0..channel.height() {
        for pixel in channel.line_ref(line) {
            if let Some(bin) = bins.get_mut((pixel - stats.min) as usize*size as usize/divisor) {
                *bin += 1;
            }
        }
    }

    Histogram { bins, min: stats.min, max: stats.max }
}

fn combine_stats(first: &ChannelStats, second: &ChannelStats) -> ChannelStats {
    ChannelStats {
        min: min(first.min, second.min),
        max: max(first.max, second.max),
    }
}