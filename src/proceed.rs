use std::sync::mpsc::{channel, Sender};
use std::thread;

use image::DynamicImage;
use tokio::runtime::Handle;

use crate::gui::app::ThreadConnection;
use crate::job::{accumulate, download, parse};

pub fn create_threads(result_image_tx: Sender<Option<DynamicImage>>, thread_number: usize) ->
(Vec<Sender<Option<String>>>, Vec<ThreadConnection>) {
    let rt = Handle::current();
    let mut query_senders = vec![];
    let mut thread_connections = vec![];

    for i in 1..thread_number {
        let (query_tx, query_rx) = channel();
        let (parse_log_tx, parse_log_rx) = channel();
        query_senders.push(query_tx);

        let (url_tx, url_rx) = channel();
        let (image_tx, image_rx) = channel();

        thread::spawn(move || {
            parse::job(query_rx, url_tx, parse_log_tx);
        });
        thread_connections.push(ThreadConnection::new(
            format!("Parse_{}", i),
            parse_log_rx,
        ));

        rt.spawn(async move {
            download::job(url_rx, image_tx).await;
        });

        let main_sender = result_image_tx.clone();
        thread::spawn(move || {
            accumulate::job(image_rx, main_sender);
        });
    }

    (query_senders, thread_connections)
}