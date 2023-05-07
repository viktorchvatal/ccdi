use std::sync::Arc;

use ccdi_common::{ClientMessage, ProcessMessage, ConvertRawImage, debayer_scale_fast};

// ============================================ PUBLIC =============================================

pub fn handle_process_message(message: ProcessMessage) -> Vec<ClientMessage> {
    match message {
        ProcessMessage::ConvertRawImage(message) => vec![
            convert_raw_image(message)
        ],
    }
}

// =========================================== PRIVATE =============================================

fn convert_raw_image(message: ConvertRawImage) -> ClientMessage {
    let rgb_image = Arc::new(debayer_scale_fast(&message.image, message.size));
    ClientMessage::RgbImage(rgb_image)
}