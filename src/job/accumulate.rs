use std::sync::mpsc::{Receiver, Sender};

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub fn job(rx: Receiver<DynamicImage>, tx: Sender<DynamicImage>, max: usize) {
    let mut medium = rx.iter().next().unwrap();
    let mut i = 1;
    if max == i {
        println!("Only 1 to accumulate. Sending to next");
        tx.send(medium.clone()).expect("Can't send accumulated result");
        return;
    }

    loop {
        for image in &rx {
            accumulate(&image, i, &mut medium);
            i += 1;

            if max == i {
                println!("Sending to next");
                tx.send(medium.clone()).expect("Can't send accumulated result");
                break;
            }
        }
    }
}

pub fn accumulate(image: &DynamicImage, iteration: usize, medium_image: &mut DynamicImage) {
    let (imgx, imgy) = image.dimensions();

    for x in 0..imgx {
        for y in 0..imgy {
            let Rgba(pixel) = image.get_pixel(x, y);
            let Rgba(medium_pixel) = medium_image.get_pixel(x, y);
            let new_pixel = Rgba([
                calculate_new_byte(pixel[0], medium_pixel[0], iteration),
                calculate_new_byte(pixel[1], medium_pixel[1], iteration),
                calculate_new_byte(pixel[1], medium_pixel[1], iteration),
                1
            ]);

            medium_image.put_pixel(x, y, new_pixel);
        }
    }
}

fn calculate_new_byte(byte: u8, medium_byte: u8, iteration: usize) -> u8 {
    ((medium_byte as usize * iteration + byte as usize) / (iteration + 1)) as u8
}