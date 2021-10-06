use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};

use image::{DynamicImage, ImageFormat, GenericImageView};
use image::io::Reader as ImageReader;

pub fn job(rx: Receiver<Vec<u8>>, tx: Sender<DynamicImage>) {
    loop {
        for bytes in &rx {
            let image = decode(bytes);
            tx.send(image).expect("Can't send bytes to channel");
        }
    }
}

fn decode(bytes: Vec<u8>) -> DynamicImage {
    ImageReader::with_format(Cursor::new(bytes.as_slice()), ImageFormat::Jpeg)
        .decode()
        .expect("Can't decode image")
}
