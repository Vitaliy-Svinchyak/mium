use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

use image::DynamicImage;

use crate::job::accumulate;
use crate::sync::thread_info_sender::ThreadInfoSender;
use crate::CliArgs;

pub fn job(
    args: CliArgs,
    result_image_rx: Receiver<Option<DynamicImage>>,
    result_image_tx: Sender<DynamicImage>,
    sender: ThreadInfoSender,
    thread_number: usize,
    start_time: Instant,
) {
    let result_picture = collect_result(result_image_rx, sender.clone(), args.pages, thread_number);
    sender.info(format!("Done in: {:?}", start_time.elapsed()));

    result_picture
        .save(format!("./{}.jpeg", args.file))
        .expect("Can't save image");

    result_image_tx.send(result_picture).expect("Can't send result picture to ui");
    sender.closed();
}

fn collect_result(
    result_image_rx: Receiver<Option<DynamicImage>>,
    sender: ThreadInfoSender,
    pages: usize,
    thread_number: usize,
) -> DynamicImage {
    let mut results_received = 0;
    let mut valid_results_received = 1;
    let mut medium_picture = loop {
        if let Ok(medium_picture) = result_image_rx.recv() {
            results_received += 1;

            sender.progress();
            if let Some(medium_picture) = medium_picture {
                sender.info("Received first image".to_owned());

                break medium_picture;
            }

            sender.info(format!("None {} / {}", results_received, thread_number));
        }
    };

    if pages == 1 {
        sender.closed();
        return medium_picture;
    }

    loop {
        if let Ok(picture) = result_image_rx.recv() {
            if let Some(picture) = picture {
                accumulate::accumulate(&picture, valid_results_received, &mut medium_picture);
                valid_results_received += 1;
            }

            results_received += 1;
            sender.info(format!("Some {} / {}", results_received, thread_number));
            sender.progress();

            if results_received == thread_number {
                medium_picture
                    .save("./result.jpeg")
                    .expect("Can't save image");

                break medium_picture;
            }
        }
    }
}
