use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};

use image::{DynamicImage, ImageFormat};
use image::io::Reader as ImageReader;

pub fn job(rx: Receiver<Option<Vec<u8>>>, tx: Sender<Option<DynamicImage>>) {
    loop {
        for bytes in &rx {
            match bytes {
                None => {
                    tx.send(None).expect("Can't send bytes to channel");
                    break;
                }
                Some(bytes) => {
                    let image = decode(bytes);
                    tx.send(Some(image)).expect("Can't send bytes to channel");
                }
            }
        }
    }
}

fn decode(bytes: Vec<u8>) -> DynamicImage {
    ImageReader::with_format(Cursor::new(bytes.as_slice()), ImageFormat::Jpeg)
        .decode()
        .expect("Can't decode image")
}
