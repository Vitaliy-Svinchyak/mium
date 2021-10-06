extern crate num_cpus;

use std::sync::mpsc::channel;
use std::thread;

use tokio::runtime::Handle;

use crate::job::{accumulate, decode, download};
use crate::job::parse;

mod job;

const MAX_IMAGES_PER_PAGE: usize = 5;
const MAX_PAGES: usize = 1;

#[tokio::main]
async fn main() {
    let max_cpus = num_cpus::get();
    let max_cpus = 2;
    let (result_image_tx, result_image_rx) = channel();
    let rt = Handle::current();

    for page in 1..max_cpus {
        let (url_tx, url_rx) = channel();
        let (bytes_tx, bytes_rx) = channel();
        let (image_tx, image_rx) = channel();
        let (acc_image_tx, acc_image_rx) = channel();
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", "nature", page);

        thread::spawn(move || {
            parse::job(query, url_tx);
        });

        rt.spawn(async move {
            download::job(url_rx, bytes_tx, MAX_IMAGES_PER_PAGE).await;
        });

        thread::spawn(move || {
            decode::job(bytes_rx, image_tx);
        });

        thread::spawn(move || {
            accumulate::job(image_rx, acc_image_tx, MAX_IMAGES_PER_PAGE);
        });

        let main_sender = result_image_tx.clone();
        thread::spawn(move || {
            accumulate::job(acc_image_rx, main_sender, MAX_PAGES);
        });
    }

    loop {
        match result_image_rx.recv() {
            Ok(medium_picture) => {
                medium_picture.save("./result.jpeg")
                    .expect("Can't save image");
                break;
            }
            Err(_) => {}
        }
    }
}