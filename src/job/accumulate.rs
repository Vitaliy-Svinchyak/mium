use std::sync::mpsc::{channel, Sender};
use std::thread;

use anyhow::Context;
use crossbeam_channel::Receiver;
use image::{DynamicImage, GenericImageView, Rgb, RgbImage, Rgba};

use crate::sync::thread_info_connection::ThreadInfoReceiver;
use crate::sync::thread_info_sender::ThreadInfoSender;

pub fn thread(
    rx: Receiver<Option<DynamicImage>>,
    tx: Sender<Option<DynamicImage>>,
    i: usize,
) -> ThreadInfoReceiver {
    let (accumulate_log_tx, accumulate_log_rx) = channel();

    thread::spawn(move || {
        job(rx, tx, ThreadInfoSender::new(accumulate_log_tx));
    });

    ThreadInfoReceiver::new(format!("Heap_{}", i), format!("H{}", i), accumulate_log_rx)
}

fn job(
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

    let mut medium = medium.unwrap().into_rgb8();

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
        .send(Some(DynamicImage::ImageRgb8(medium.clone())))
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

#[inline(always)]
pub fn accumulate(image: &DynamicImage, iteration: u8, medium_image: &mut RgbImage) {
    let mut image_iterator = image.pixels();
    for (_, _, Rgb(medium_pixel)) in medium_image.enumerate_pixels_mut() {
        let (_, _, Rgba(pixel)) = &image_iterator.next().unwrap();
        calculate_new_color(pixel, medium_pixel, iteration);
    }
}

#[inline(always)]
fn calculate_new_color(pixel: &[u8; 4], medium_pixel: &mut [u8; 3], iteration: u8) {
    let pixel_is_white = pixel[0] >= 250 && pixel[1] >= 250 && pixel[2] >= 250;
    let medium_pixel_is_white =
        medium_pixel[0] >= 250 && medium_pixel[1] >= 250 && medium_pixel[2] >= 250;

    if pixel_is_white {
        ()
    } else if medium_pixel_is_white {
        medium_pixel[0] = pixel[0];
        medium_pixel[1] = pixel[1];
        medium_pixel[2] = pixel[2];
    } else {
        medium_pixel[0] = calculate_new_byte(pixel[0], medium_pixel[0], iteration);
        medium_pixel[1] = calculate_new_byte(pixel[1], medium_pixel[1], iteration);
        medium_pixel[1] = calculate_new_byte(pixel[2], medium_pixel[2], iteration);
    }
}

#[inline(always)]
fn calculate_new_byte(byte: u8, medium_byte: u8, i: u8) -> u8 {
    if byte > medium_byte {
        medium_byte + (byte - medium_byte) / (i + 1)
    } else {
        medium_byte - (medium_byte - byte) / (i + 1)
    }
}
