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
    let (result_image_s, result_image_r) = channel();

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
        thread_connections.push(download::thread(image_url_rx.clone(), image_s.clone(), i));
        thread_connections.push(accumulate::thread(
            image_r.clone(),
            result_image_s.clone(),
            i,
        ));
    }

    let (result_image_tx_out, result_image_rx_out) = channel();
    let (summarize_log_tx, summarize_log_rx) = channel();
    thread::spawn(move || {
        summarize::job(
            args,
            result_image_r,
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
