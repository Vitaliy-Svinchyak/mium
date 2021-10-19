use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use image::DynamicImage;

use crate::job::thread_broadcaster::broadcast_channel;
use crate::job::{accumulate, download, parse, summarize};
use crate::sync::thread_info_connection::ThreadInfoReceiver;
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
    let mut thread_connections = vec![];

    let (result_image_s, result_image_r) = channel();
    let (image_url_tx, image_url_rx) = broadcast_channel(1);
    let (image_s, image_r) = broadcast_channel(thread_number);
    let (query_tx, query_rx) = channel();

    thread_connections.push(parse::thread(query_rx, image_url_tx));

    for i in 1..thread_number + 1 {
        thread_connections.push(download::thread(image_url_rx.clone(), image_s.clone(), i));
        thread_connections.push(accumulate::thread(
            image_r.clone(),
            result_image_s.clone(),
            i,
        ));
    }

    let (receiver, result_image_rx_out) =
        summarize::thread(args, result_image_r, thread_number, start);
    thread_connections.push(receiver);

    (query_tx, thread_connections, result_image_rx_out)
}
