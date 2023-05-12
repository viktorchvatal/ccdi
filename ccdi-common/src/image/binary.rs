use nanocv::Img;

use crate::RgbImage;

// ============================================ PUBLIC =============================================

pub fn rgb_image_to_bytes(image: RgbImage<u16>) -> Vec<u8> {
    let mut writer = BytesWriter::new();

    writer.write_header(image.width(), image.height());
    writer.write_channel(image.red());
    writer.write_channel(image.green());
    writer.write_channel(image.blue());

    writer.into_buffer()
}

// =========================================== PRIVATE =============================================

const HEADER_1: u16 = 45812;
const HEADER_2: u16 = 19724;

struct BytesWriter {
    buffer: Vec<u8>
}

impl BytesWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn write_header(&mut self, width: usize, height: usize) {
        self.buffer.reserve(((width*height)*3 + 4)*2);
        self.write_u16(HEADER_1);
        self.write_u16(HEADER_2);
        self.write_u16(width as u16);
        self.write_u16(height as u16);
    }

    pub fn write_channel(&mut self, image: &dyn Img<u16>) {
        for line in 0..image.height() {
            let pixels = image.line_ref(line);

            for &pixel in pixels {
                self.write_u16(pixel);
            }
        }
    }

    pub fn write_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }
}