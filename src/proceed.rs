use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Instant;

use image::DynamicImage;
use tokio::runtime::Handle;

use crate::CliArgs;
use crate::gui::app::ThreadConnection;
use crate::job::{accumulate, download, parse, summarize};

pub fn create_threads(args: CliArgs, thread_number: usize) -> (Vec<Sender<Option<String>>>, Vec<ThreadConnection>) {
    let start = Instant::now();

    let (result_image_tx, result_image_rx) = channel();

    let rt = Handle::current();
    let mut query_senders = vec![];
    let mut thread_connections = vec![];

    for i in 1..thread_number + 1 {
        let (query_tx, query_rx) = channel();
        query_senders.push(query_tx);
        let (url_tx, url_rx) = channel();
        let (image_tx, image_rx) = channel();

        let (parse_log_tx, parse_log_rx) = channel();
        let (download_log_tx, download_log_rx) = channel();
        let (accumulate_log_tx, accumulate_log_rx) = channel();

        thread::spawn(move || {
            parse::job(query_rx, url_tx, parse_log_tx);
        });
        thread_connections.push(ThreadConnection::new(
            format!("Parse_{}", i),
            parse_log_rx,
        ));

        rt.spawn(async move {
            download::job(url_rx, image_tx, download_log_tx).await;
        });
        thread_connections.push(ThreadConnection::new(
            format!("Download_{}", i),
            download_log_rx,
        ));

        let main_sender = result_image_tx.clone();
        thread::spawn(move || {
            accumulate::job(image_rx, main_sender, accumulate_log_tx);
        });
        thread_connections.push(ThreadConnection::new(
            format!("Accumulate_{}", i),
            accumulate_log_rx,
        ));
    }

    let (summarize_log_tx, summarize_log_rx) = channel();
    thread::spawn(move || {
        summarize::job(args, result_image_rx,summarize_log_tx, thread_number, start);
    });

    thread_connections.push(ThreadConnection::new(
        "Summarize".to_owned(),
        summarize_log_rx,
    ));

    (query_senders, thread_connections)
}