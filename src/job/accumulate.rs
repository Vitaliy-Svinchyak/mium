use std::sync::mpsc::Sender;

use anyhow::Context;
use crossbeam_channel::Receiver;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::sync::thread_info_sender::ThreadInfoSender;

pub fn job(
    rx: Receiver<Option<DynamicImage>>,
    tx: Sender<Option<DynamicImage>>,
    sender: ThreadInfoSender,
) {
    let medium = match rx.iter().next().context("Can't get picture from channel") {
        Ok(i) => i,
        Err(e) => {
            sender.error(e);
            None
        }
    };

    if medium.is_none() {
        if let Err(e) = tx.send(None).context("Can't send early result") {
            sender.error(e);
        }
        sender.closed();

        return;
    }

    sender.info("Got first.".to_owned());
    sender.progress();

    let mut medium = medium.unwrap();

    let mut i = 1;

    'outer: loop {
        for image in &rx {
            match image {
                None => {
                    break 'outer;
                }
                Some(image) => {
                    accumulate(&image, i, &mut medium);
                    sender.info(format!("Accumulated {}.", i + 1));
                    sender.progress();

                    i += 1;
                }
            }
        }
    }

    match tx
        .send(Some(medium.clone()))
        .context("Can't send accumulated result")
    {
        Ok(_) => {}
        Err(e) => {
            sender.error(e);
        }
    }

    match tx.send(None).context("Can't send end result") {
        Ok(_) => {}
        Err(e) => {
            sender.error(e);
        }
    }
    sender.closed();
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
    let medium_pixel_is_white =
        medium_pixel[0] >= 250 && medium_pixel[1] >= 250 && medium_pixel[2] >= 250;

    if pixel_is_white {
        Rgba(medium_pixel)
    } else if medium_pixel_is_white {
        Rgba(pixel)
    } else {
        Rgba([
            calculate_new_byte(pixel[0], medium_pixel[0], iteration),
            calculate_new_byte(pixel[1], medium_pixel[1], iteration),
            calculate_new_byte(pixel[2], medium_pixel[2], iteration),
            1,
        ])
    }
}

fn calculate_new_byte(byte: u8, medium_byte: u8, iteration: usize) -> u8 {
    ((medium_byte as usize * iteration + byte as usize) / (iteration + 1)) as u8
}
