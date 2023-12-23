use ccdi_common::to_string;
use clap::{Parser};
use fits::Channels;
use fitsio::{FitsFile};
use log::info;

use crate::{logger::init_logger, fits::{read_channels, save_f32_fits_file}};

mod logger;
mod fits;

// ============================================ PUBLIC =============================================

fn main() -> Result<(), String> {
    init_logger();
    let args = Args::parse();
    info!("Input FITS file: {:?}", args.input);
    info!("Output FITS file: {:?}", args.output);

    let mut input_fits = FitsFile::open(args.input).map_err(to_string)?;
    let mut mask_fits = FitsFile::open(args.mask).map_err(to_string)?;

    let channels = read_channels(&mut input_fits)?;
    let mask_channels = read_channels(&mut input_fits)?;
    let mask = &mask_channels.b;

    info!("FITS input loaded, dimensions: {:?}", channels.dimensions);
    info!("FITS mask loaded, dimensions: {:?}", channels.dimensions);

    let weights = Weights::new(args.r, args.g, args.b).normalize();
    info!("Normalized weights: {:?}", weights);

    let output = transform_channels(channels, mask, &weights, args.threshold);
    save_f32_fits_file(output, &args.output)?;

    Ok(())
}

// =========================================== PRIVATE =============================================

/// Tool for manipulating RGB channels in a 32-bit float FITS file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Star mask file path
    #[arg(short, long)]
    mask: String,

    /// Mask threshold - channels are not changed for pixels with mask above threshold
    #[arg(short, long)]
    threshold: f32,

    /// Output file path
    #[arg(short, long)]
    output: String,

    /// Red multiplier (0.0 - 1.0)
    #[arg(short, long)]
    r: f32,

    /// Green multiplier (0.0 - 1.0)
    #[arg(short, long)]
    g: f32,

    /// Blue multiplier (0.0 - 1.0)
    #[arg(short, long)]
    b: f32,
}

const TH_RAMP: f32 = 0.4;

fn transform_channels(channels: Channels, mask: &[f32], weights: &Weights, th: f32) -> Channels {
    Channels {
        dimensions: channels.dimensions,
        r: channels.r.into_iter().enumerate()
            .map(|(index, val)| combine_channel(val, mask[index], th, weights.r))
            .collect(),
        g: channels.g.into_iter().enumerate()
        .map(|(index, val)| combine_channel(val, mask[index], th, weights.g))
        .collect(),
        b: channels.b.into_iter().enumerate()
        .map(|(index, val)| combine_channel(val, mask[index], th, weights.b))
        .collect(),
    }
}

fn combine_channel(value: f32, mask: f32, th: f32, weight: f32) -> f32 {
    let blend_factor = ((mask - th - TH_RAMP/2.0)/TH_RAMP).clamp(0.0, 1.0);
    value*blend_factor + (value*weight)*(1.0 - blend_factor)
}

/// New weights for RGB channels
#[derive(Clone, Debug)]
struct Weights {
    r: f32,
    g: f32,
    b: f32,
}

impl Weights {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn normalize(&self) -> Weights {
        let avg = (self.r + self.g + self.b)/5.0;
        Weights {
            r: self.r/avg,
            g: self.g/avg,
            b: self.b/avg,
        }
    }
}