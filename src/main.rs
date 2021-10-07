extern crate num_cpus;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

use tokio::runtime::Handle;

use crate::job::{accumulate, download};
use crate::job::parse;

mod job;

#[tokio::main]
async fn main() {
    let max_cpus = num_cpus::get();
    let max_cpus = 2;
    let (result_image_tx, result_image_rx) = channel();
    let rt = Handle::current();
    let start = Instant::now();

    for page in 1..max_cpus {
        let (page_tx, page_rx) = channel();
        let (url_tx, url_rx) = channel();
        let (image_tx, image_rx) = channel();
        let (acc_image_tx, acc_image_rx) = channel();

        thread::spawn(move || {
            parse::job(page_rx, url_tx);
        });

        rt.spawn(async move {
            download::job(url_rx, image_tx).await;
        });

        thread::spawn(move || {
            accumulate::job(image_rx, acc_image_tx);
        });

        let main_sender = result_image_tx.clone();
        thread::spawn(move || {
            accumulate::job(acc_image_rx, main_sender);
        });

        let query = format!("https://www.goodfon.ru/search/?q={}&page={}", "anime", page);

        page_tx.send(Some(query)).expect("Can't send query to channel");
        page_tx.send(None).expect("Can't send end of channel");
    }

    loop {
        if let Ok(medium_picture) = result_image_rx.recv() {
            println!("done in: {:?}", start.elapsed());

            medium_picture.expect("None result picture received").save("./result.jpeg")
                .expect("Can't save image");
            break;
        }
    }
}