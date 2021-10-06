extern crate num_cpus;

use std::sync::mpsc::{channel};
use std::thread;

use futures::future::join_all;

use crate::job::{accumulate, decode, download};
use crate::job::parse;

mod job;

const MAX_IMAGES_PER_PAGE: usize = 2;
const MAX_PAGES: usize = 1;

#[tokio::main]
async fn main() {
    let max_cpus = num_cpus::get();
    let max_cpus = 2;
    let mut parse_jobs: Vec<_> = vec![];
    let mut download_jobs: Vec<_> = vec![];
    let (result_image_tx, result_image_rx) = channel();

    for page in 1..max_cpus {
        let (url_tx, url_rx) = channel();
        let (bytes_tx, bytes_rx) = channel();
        let (image_tx, image_rx) = channel();
        let (acc_image_tx, acc_image_rx) = channel();
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", "anime", page);

        let parse_job = thread::spawn(move || async move {
            parse::job(query, url_tx).await;
        }).join().expect("Failed to create parse thread");

        let download_job = thread::spawn(move || async move {
            download::job(url_rx, bytes_tx, MAX_IMAGES_PER_PAGE).await;
        }).join().expect("Failed to create download thread");

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

        parse_jobs.push(parse_job);
        download_jobs.push(download_job);
    }

    join_all(parse_jobs).await;
    join_all(download_jobs).await;

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

    // let mut medium_picture = pictures[0].clone();
    // for (i, picture) in pictures.iter().skip(1).enumerate() {
    //     medium_picture.save(format!("./result{}.jpeg", i))
    //         .expect("Can't save middle image");
    //     accumulate::accumulate(picture, i, &mut medium_picture);
    // }
    // medium_picture.save("./result.jpeg")
    //     .expect("Can't save image");
}