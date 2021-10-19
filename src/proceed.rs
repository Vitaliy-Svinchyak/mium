use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Instant;

use image::DynamicImage;
use tokio::runtime::Handle;

use crate::job::thread_broadcaster::broadcast_channel;
use crate::job::{accumulate, download, parse, summarize};
use crate::sync::thread_info_connection::ThreadInfoReceiver;
use crate::sync::thread_info_sender::ThreadInfoSender;
use crate::CliArgs;

pub fn create_threads(
    args: CliArgs,
    thread_number: usize,
) -> (
    Sender<Option<String>>,
    Vec<ThreadInfoReceiver>,
    Receiver<DynamicImage>,
) {
    let start = Instant::now();

    let (result_image_tx, result_image_rx) = channel();
    let (result_image_tx_out, result_image_rx_out) = channel();

    let rt = Handle::current();
    let mut thread_connections = vec![];

    let (image_url_tx, image_url_rx) = broadcast_channel(1);
    let (image_s, image_r) = broadcast_channel(thread_number);

    let (query_tx, query_rx) = channel();
    let (parse_log_tx, parse_log_rx) = channel();
    rt.spawn(async move {
        parse::job(query_rx, image_url_tx, ThreadInfoSender::new(parse_log_tx)).await;
    });
    thread_connections.push(ThreadInfoReceiver::new(
        "Parse".to_owned(),
        "Parse".to_owned(),
        parse_log_rx,
    ));

    for i in 1..thread_number + 1 {
        let image_tx = image_s.clone();
        let image_url_rx = image_url_rx.clone();
        let (download_log_tx, download_log_rx) = channel();
        rt.spawn(async move {
            download::job(image_url_rx, image_tx, ThreadInfoSender::new(download_log_tx)).await;
        });
        thread_connections.push(ThreadInfoReceiver::new(
            format!("Get_{}", i),
            format!("G{}", i),
            download_log_rx,
        ));

        let image_rx = image_r.clone();
        let (accumulate_log_tx, accumulate_log_rx) = channel();
        let main_sender = result_image_tx.clone();
        thread::spawn(move || {
            accumulate::job(
                image_rx,
                main_sender,
                ThreadInfoSender::new(accumulate_log_tx),
            );
        });
        thread_connections.push(ThreadInfoReceiver::new(
            format!("Heap_{}", i),
            format!("H{}", i),
            accumulate_log_rx,
        ));
    }

    let (summarize_log_tx, summarize_log_rx) = channel();
    thread::spawn(move || {
        summarize::job(
            args,
            result_image_rx,
            result_image_tx_out,
            ThreadInfoSender::new(summarize_log_tx),
            thread_number,
            start,
        );
    });

    thread_connections.push(ThreadInfoReceiver::new(
        "Summarize".to_owned(),
        "Sum".to_owned(),
        summarize_log_rx,
    ));

    (query_tx, thread_connections, result_image_rx_out)
}
