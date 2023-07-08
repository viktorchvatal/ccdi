
use std::{mem::swap, sync::{mpsc::Sender, Arc}};

use ccdi_common::{
    ExposureCommand, ClientMessage, RawImage, ProcessMessage, ConvertRawImage, log_err,
    CameraParams, StorageMessage
};
use ccdi_imager_interface::{BasicProperties, ImagerDevice, ExposureParams, ExposureArea};
use log::debug;

// ============================================ PUBLIC =============================================

pub struct ExposureController {
    properties: BasicProperties,
    camera_params: CameraParams,
    current_exposure: Option<ExposureParams>,
    process_tx: Sender<ProcessMessage>,
    storage_tx: Sender<StorageMessage>,
}

impl ExposureController {
    pub fn new(
        properties: BasicProperties,
        process_tx: Sender<ProcessMessage>,
        storage_tx: Sender<StorageMessage>,
    ) -> Self {
        Self {
            properties,
            camera_params: Default::default(),
            current_exposure: None,
            process_tx,
            storage_tx,
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
                debug!("Image downloaded");
                self.call_process_message(Arc::new(raw_image));
            }
        }

        if !self.exposure_active() && self.camera_params.loop_enabled {
            self.start_exposure(device)?;
        }

        Ok(vec![])
    }

    pub fn update_camera_params(&mut self, params: CameraParams) {
        self.camera_params = params;
    }

    pub fn exposure_command(
        &mut self,
        device: &mut dyn ImagerDevice,
        command: ExposureCommand
    ) -> Result<(), String> {
        Ok(match command {
            ExposureCommand::Start => self.start_exposure(device)?,
        })
    }

    pub fn exposure_active(&self) -> bool {
        self.current_exposure.is_some()
    }
}

// =========================================== PRIVATE =============================================

impl ExposureController {
    fn call_process_message(&self, image: Arc<RawImage>) {
        let rendering = self.camera_params.rendering;
        let size = self.camera_params.render_size;
        let message = StorageMessage::ProcessImage(image.clone());
        log_err("Self process message", self.storage_tx.send(message));
        let message = ProcessMessage::ConvertRawImage(ConvertRawImage{image, size, rendering});
        log_err("Self process message", self.process_tx.send(message));
    }

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
            gain: self.camera_params.gain,
            time: self.camera_params.time,
            area: ExposureArea {
                x: 0,
                y: 0,
                width: self.properties.width,
                height: self.properties.height
            }
        }
    }
}
