use std::sync::mpsc::{channel, Sender};
use std::thread;

use tokio::runtime::Handle;

use crate::job::{accumulate, download, parse};
use image::DynamicImage;

pub fn create_threads(result_image_tx: Sender<Option<DynamicImage>>, thread_number: usize) -> Vec<Sender<Option<String>>> {
    let rt = Handle::current();
    let mut query_senders = vec![];

    for _a in 0..thread_number {
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

    query_senders
}