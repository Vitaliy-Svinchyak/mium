use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

use image::DynamicImage;

use crate::sync::thread_event::ThreadEvent;
use crate::job::accumulate;
use crate::CliArgs;

pub fn job(
    args: CliArgs,
    result_image_rx: Receiver<Option<DynamicImage>>,
    log_tx: Sender<ThreadEvent>,
    thread_number: usize,
    start_time: Instant,
) {
    let result_picture = collect_result(result_image_rx, log_tx.clone(), args.pages, thread_number);
    log_tx.send(ThreadEvent::info(format!(
        "Done in: {:?}",
        start_time.elapsed()
    )));

    result_picture
        .save(format!("./{}.jpeg", args.file))
        .expect("Can't save image");
}

fn collect_result(
    result_image_rx: Receiver<Option<DynamicImage>>,
    log_tx: Sender<ThreadEvent>,
    pages: usize,
    thread_number: usize,
) -> DynamicImage {
    let mut results_received = 0;
    let mut valid_results_received = 1;
    let mut medium_picture = loop {
        if let Ok(medium_picture) = result_image_rx.recv() {
            results_received += 1;

            log_tx.send(ThreadEvent::progress());
            if let Some(medium_picture) = medium_picture {
                log_tx.send(ThreadEvent::info("Received first image".to_owned()));

                break medium_picture;
            }

            log_tx.send(ThreadEvent::info(format!(
                "None {} / {}",
                results_received, thread_number
            )));
        }
    };

    if pages == 1 {
        log_tx.send(ThreadEvent::close());
        return medium_picture;
    }

    loop {
        if let Ok(picture) = result_image_rx.recv() {
            if let Some(picture) = picture {
                accumulate::accumulate(&picture, valid_results_received, &mut medium_picture);
                valid_results_received += 1;
            }

            results_received += 1;
            log_tx.send(ThreadEvent::info(format!(
                "Some {} / {}",
                results_received, thread_number
            )));
            log_tx.send(ThreadEvent::progress());

            if results_received == thread_number {
                log_tx.send(ThreadEvent::close());

                medium_picture
                    .save("./result.jpeg")
                    .expect("Can't save image");

                break medium_picture;
            }
        }
    }
}
