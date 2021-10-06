use image::{DynamicImage, ImageFormat};
use image::io::Reader as ImageReader;
use std::io::Cursor;

pub fn job(bytes: Vec<u8>) -> DynamicImage {
    ImageReader::with_format(Cursor::new(bytes.as_slice()), ImageFormat::Jpeg)
        .decode()
        .expect("Can't decode image")
}
