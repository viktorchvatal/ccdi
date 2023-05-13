use std::collections::VecDeque;

use nanocv::{Img, ImgSize, ImgBuf, ImgMut};

use crate::RgbImage;

// ============================================ PUBLIC =============================================

pub fn rgb_image_to_bytes(image: &RgbImage<u16>) -> Vec<u8> {
    let mut writer = BytesWriter::new();

    writer.write_header(image.width(), image.height());
    writer.write_channel(image.red());
    writer.write_channel(image.green());
    writer.write_channel(image.blue());

    writer.into_buffer()
}

pub fn rgb_image_from_bytes(bytes: Vec<u8>) -> Result<RgbImage<u16>, String> {
    let mut reader = BytesReader::new(bytes);

    reader.read_header()?;
    let dimensions = reader.read_dimensions()?;
    let red = reader.read_channel(dimensions)?;
    let green = reader.read_channel(dimensions)?;
    let blue = reader.read_channel(dimensions)?;

    RgbImage::from(red, green, blue)
}

// =========================================== PRIVATE =============================================

const HEADER_1: u16 = 45812;
const HEADER_2: u16 = 19724;

struct BytesWriter {
    buffer: Vec<u8>
}

impl BytesWriter {
    fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    fn write_header(&mut self, width: usize, height: usize) {
        self.buffer.reserve(((width*height)*3 + 4)*2);
        self.write_u16(HEADER_1);
        self.write_u16(HEADER_2);
        self.write_u16(width as u16);
        self.write_u16(height as u16);
    }

    fn write_channel(&mut self, image: &dyn Img<u16>) {
        for line in 0..image.height() {
            let pixels = image.line_ref(line);

            for &pixel in pixels {
                self.write_u16(pixel);
            }
        }
    }

    fn write_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
    }

    fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

struct BytesReader {
    buffer: VecDeque<u8>
}

impl BytesReader {
    fn new(bytes: Vec<u8>) -> Self {
        Self { buffer: VecDeque::from(bytes) }
    }

    fn read_header(&mut self) -> Result<(), String> {
        let header1 = self.read_u16()?;
        let header2 = self.read_u16()?;

        match (header1, header2) == (HEADER_1, HEADER_2) {
            true => Ok(()),
            false => Err(format!("Invalid header: [{}, {}]", header1, header2))
        }
    }

    fn read_dimensions(&mut self) -> Result<ImgSize, String> {
        let width = self.read_u16()?;
        let height = self.read_u16()?;
        Ok(ImgSize::new(width as usize, height as usize))
    }

    fn read_channel(&mut self, dimensions: ImgSize) -> Result<ImgBuf<u16>, String> {
        let mut channel = ImgBuf::new(dimensions);

        for line in 0..channel.height() {
            let pixels = channel.line_mut(line);
            let line = self.read_line(pixels.len())?;
            pixels.clone_from_slice(&line);
        }

        Ok(channel)
    }

    fn read_line(&mut self, length: usize) -> Result<Vec<u16>, String> {
        (0..length).map(|_| self.read_u16()).collect::<Result<Vec<u16>, String>>()
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        const ERR: &str = "Data too short";

        let two_bytes: [u8; 2] = [
            self.buffer.pop_front().ok_or(ERR.to_owned())?,
            self.buffer.pop_front().ok_or(ERR.to_owned())?,
        ];

        Ok(u16::from_le_bytes(two_bytes))
    }
}

// ============================================= TEST ==============================================

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use nanocv::{ImgBuf, ImgSize};

    use super::*;

    #[test]
    fn test_to_bytes_and_back() {
        let size = ImgSize::new(5, 4);

        let image = RgbImage::from(
            ImgBuf::from_vec(size, vec![1u16; 20]),
            ImgBuf::from_vec(size, vec![2u16; 20]),
            ImgBuf::from_vec(size, vec![3u16; 20]),
        ).expect("Invalid data size");

        let bytes = rgb_image_to_bytes(&image);

        let converted = rgb_image_from_bytes(bytes).expect("Conversion failed");
        assert_eq!(image, converted);
    }
}