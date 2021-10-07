extern crate num_cpus;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

use image::DynamicImage;
use tokio::runtime::Handle;

use crate::job::{accumulate, download};
use crate::job::parse;

mod job;

#[tokio::main]
async fn main() {
    let pages_to_parse = 3;
    let max_cpus = num_cpus::get();
    let (result_image_tx, result_image_rx) = channel();
    let rt = Handle::current();
    let start = Instant::now();
    let mut query_senders = vec![];

    for page in 0..max_cpus {
        let (query_tx, query_rx) = channel();
        query_senders.push(query_tx);

        let (url_tx, url_rx) = channel();
        let (image_tx, image_rx) = channel();

        thread::spawn(move || {
            parse::job(query_rx, url_tx);
        });

        rt.spawn(async move {
            download::job(url_rx, image_tx).await;
        });

        let main_sender = result_image_tx.clone();
        thread::spawn(move || {
            accumulate::job(image_rx, main_sender);
        });
    }


    let mut i = 0;
    for page in 1..pages_to_parse {
        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", "anime", page);
        let query_tx = &query_senders[i];
        query_tx.send(Some(query)).expect("Can't send query to channel");

        i += 1;
        if i == query_senders.len() {
            i = 0;
        }
    }

    for query_tx in query_senders {
        query_tx.send(None).expect("Can't send end of channel");
    }

    let mut results_received = 0;
    let mut valid_results_received = 1;
    let mut medium_picture = loop {
        if let Ok(medium_picture) = result_image_rx.recv() {
            results_received += 1;
            if let Some(medium_picture) = medium_picture {
                break medium_picture;
            }
        }
    };

    loop {
        if let Ok(picture) = result_image_rx.recv() {
            if let Some(picture) = picture {
                accumulate::accumulate(&picture, valid_results_received, &mut medium_picture);
                valid_results_received += 1;
            }

            results_received += 1;

            if results_received == max_cpus {
                println!("done in: {:?}", start.elapsed());

                medium_picture
                    .save("./result.jpeg")
                    .expect("Can't save image");

                break;
            }
        }
    }
}