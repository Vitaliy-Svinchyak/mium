use std::sync::mpsc::{Receiver, Sender};

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub fn job(rx: Receiver<Option<DynamicImage>>, tx: Sender<Option<DynamicImage>>) {
    let mut medium = rx.iter()
        .next()
        .expect("Can't get picture from channel")
        .expect("None first picture received");
    let mut i = 1;

    loop {
        for image in &rx {
            match image {
                None => {
                    tx.send(Some(medium.clone())).expect("Can't send accumulated result");
                    tx.send(None).expect("Can't send accumulated result");
                    break;
                }
                Some(image) => {
                    accumulate(&image, i, &mut medium);
                    i += 1;
                }
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
            let new_pixel = calculate_new_color(pixel, medium_pixel, iteration);

            medium_image.put_pixel(x, y, new_pixel);
        }
    }
}

fn calculate_new_color(pixel: [u8; 4], medium_pixel: [u8; 4], iteration: usize) -> Rgba<u8> {
    let pixel_is_white = pixel[0] >= 250 && pixel[1] >= 250 && pixel[2] >= 250;
    let medium_pixel_is_white = medium_pixel[0] >= 250 && medium_pixel[1] >= 250 && medium_pixel[2] >= 250;

    if pixel_is_white {
        Rgba(medium_pixel)
    } else if medium_pixel_is_white {
        Rgba(pixel)
    } else {
        Rgba([
            calculate_new_byte(pixel[0], medium_pixel[0], iteration),
            calculate_new_byte(pixel[1], medium_pixel[1], iteration),
            calculate_new_byte(pixel[2], medium_pixel[2], iteration),
            1
        ])
    }
}

fn calculate_new_byte(byte: u8, medium_byte: u8, iteration: usize) -> u8 {
    ((medium_byte as usize * iteration + byte as usize) / (iteration + 1)) as u8
}