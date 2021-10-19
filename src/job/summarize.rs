use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Instant;

use anyhow::Context;
use image::DynamicImage;

use crate::job::accumulate;
use crate::sync::thread_info_connection::ThreadInfoReceiver;
use crate::sync::thread_info_sender::ThreadInfoSender;
use crate::CliArgs;

pub fn thread(
    args: CliArgs,
    rx: Receiver<Option<DynamicImage>>,
    thread_number: usize,
    start: Instant,
) -> (ThreadInfoReceiver, Receiver<DynamicImage>) {
    let (result_image_tx_out, result_image_rx_out) = channel();
    let (summarize_log_tx, summarize_log_rx) = channel();

    thread::spawn(move || {
        job(
            args,
            rx,
            result_image_tx_out,
            ThreadInfoSender::new(summarize_log_tx),
            thread_number,
            start,
        );
    });

    let thread_receiver =
        ThreadInfoReceiver::new("Summarize".to_owned(), "Sum".to_owned(), summarize_log_rx);
    (thread_receiver, result_image_rx_out)
}

fn job(
    args: CliArgs,
    result_image_rx: Receiver<Option<DynamicImage>>,
    result_image_tx: Sender<DynamicImage>,
    sender: ThreadInfoSender,
    thread_number: usize,
    start_time: Instant,
) {
    match collect_and_send(
        args,
        result_image_rx,
        result_image_tx,
        &sender,
        thread_number,
        start_time,
    ) {
        Ok(_) => {}
        Err(e) => {
            sender.error(e);
        }
    };

    sender.closed();
}

fn collect_and_send(
    args: CliArgs,
    result_image_rx: Receiver<Option<DynamicImage>>,
    result_image_tx: Sender<DynamicImage>,
    sender: &ThreadInfoSender,
    thread_number: usize,
    start_time: Instant,
) -> anyhow::Result<()> {
    let result_picture =
        collect_result(result_image_rx, sender.clone(), args.pages, thread_number)?;
    sender.info(format!("Done in: {:?}", start_time.elapsed()));

    result_picture
        .save(format!("./{}.jpeg", args.file))
        .context("Can't save image")?;

    result_image_tx
        .send(result_picture)
        .context("Can't send result picture to ui")?;

    Ok(())
}

fn collect_result(
    result_image_rx: Receiver<Option<DynamicImage>>,
    sender: ThreadInfoSender,
    pages: usize,
    thread_number: usize,
) -> anyhow::Result<DynamicImage> {
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
        return Ok(medium_picture);
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
                    .context("Can't save image")?;

                break Ok(medium_picture);
            }
        }
    }
}
