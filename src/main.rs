extern crate num_cpus;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use image::DynamicImage;

use crate::job::accumulate;

mod job;
mod proceed;

#[tokio::main]
async fn main() {
    let pages_to_parse = 3;
    let (result_image_tx, result_image_rx) = channel();
    let start = Instant::now();
    let query_senders = proceed::job(result_image_tx);
    send_jobs(query_senders, pages_to_parse);

    let result_picture = collect_result(result_image_rx);
    println!("done in: {:?}", start.elapsed());

    result_picture
        .save("./result.jpeg")
        .expect("Can't save image");
}

fn send_jobs(query_senders: Vec<Sender<Option<String>>>, pages_to_parse: usize) {
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
}

fn collect_result(result_image_rx: Receiver<Option<DynamicImage>>) -> DynamicImage {
    let max_cpus = num_cpus::get();

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
                medium_picture
                    .save("./result.jpeg")
                    .expect("Can't save image");

                break medium_picture;
            }
        }
    }
}