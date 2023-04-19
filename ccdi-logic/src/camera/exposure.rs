
use std::mem::swap;

use ccdi_common::{ExposureCommand, ClientMessage, RawImage, debayer_scale_fast};
use ccdi_imager_interface::{BasicProperties, ImagerDevice, ExposureParams, ExposureArea};
use log::debug;
use nanocv::ImgSize;

// ============================================ PUBLIC =============================================

pub struct ExposureController {
    properties: BasicProperties,
    gain: u16,
    time: f64,
    current_exposure: Option<ExposureParams>,
}

impl ExposureController {
    pub fn new(properties: BasicProperties) -> Self {
        Self {
            properties,
            gain: 0,
            time: 1.0,
            current_exposure: None
        }
    }

    pub fn periodic(
        &mut self,
        device: &mut dyn ImagerDevice
    ) -> Result<Vec<ClientMessage>, String> {
        if self.current_exposure.is_some() && device.image_ready()? {
            debug!("Image ready to download");
            let mut exposure = None;
            swap(&mut exposure, &mut self.current_exposure);

            if let Some(params) = exposure {
                let data = device.download_image(&params)?;
                let raw_image = RawImage { params, data };
                // TODO: Compute size
                debug!("Image downloaded");
                let rgb_image = debayer_scale_fast(&raw_image, ImgSize::new(900, 600));
                debug!("Image resized");
                Ok(vec![ClientMessage::RgbImage(rgb_image)])
            } else {
                Ok(vec![])
            }
        } else {
            Ok(vec![])
        }
    }

    pub fn exposure_command(
        &mut self,
        device: &mut dyn ImagerDevice,
        command: ExposureCommand
    ) -> Result<(), String> {
        Ok(match command {
            ExposureCommand::Start => self.start_exposure(device)?,
            ExposureCommand::SetGain(gain) => self.gain = gain,
            ExposureCommand::SetTime(time) => self.time = time,
        })
    }

    pub fn exposure_active(&self) -> bool {
        self.current_exposure.is_some()
    }
}

// =========================================== PRIVATE =============================================

impl ExposureController {
    fn start_exposure(&mut self, device: &mut dyn ImagerDevice) -> Result<(), String> {
        debug!("Starting exposure");
        if self.current_exposure.is_some() {
            return Err(format!("Exposure already in progress."))
        }

        let params = self.make_exposure_description();
        let result = device.start_exposure(&params);

        if result.is_ok() {
            self.current_exposure = Some(params)
        }

        debug!("Exposure started");
        result
    }

    fn make_exposure_description(&self) -> ExposureParams {
        ExposureParams {
            gain: self.gain,
            time: self.time,
            area: ExposureArea {
                x: 0,
                y: 0,
                width: self.properties.width,
                height: self.properties.height
            }
        }
    }
}
