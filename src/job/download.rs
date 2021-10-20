use std::io::Cursor;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use anyhow::{Context, Result};
use crossbeam_channel::Receiver;
use futures_util::future::join_all;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use reqwest::header::USER_AGENT;

use crate::sync::thread_info_connection::ThreadInfoReceiver;
use crate::sync::thread_info_sender::ThreadInfoSender;
use bytes::Bytes;

pub fn thread(
    rx: Receiver<Option<String>>,
    tx: Sender<Option<DynamicImage>>,
    i: usize,
) -> ThreadInfoReceiver {
    let (download_log_tx, download_log_rx) = channel();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            job(rx, tx, ThreadInfoSender::new(download_log_tx)).await;
        });
    });

    ThreadInfoReceiver::new(format!("Get_{}", i), format!("G{}", i), download_log_rx)
}

async fn job(
    rx: Receiver<Option<String>>,
    tx: Sender<Option<DynamicImage>>,
    sender: ThreadInfoSender,
) {
    let mut downloads: Vec<_> = vec![];

    for url in rx {
        match url {
            None => {
                join_all(downloads).await;
                let send_result = tx.send(None).context("Can't send end of download channel");
                if let Err(e) = send_result {
                    sender.error(e);
                }
                sender.closed();

                break;
            }
            Some(url) => {
                let image_sender = tx.clone();
                let log_sender = sender.clone();
                sender.info(format!("Downloading: {}", url));
                downloads.push(do_job(url, image_sender, log_sender));
            }
        }
    }
}

async fn do_job(url: String, tx: Sender<Option<DynamicImage>>, sender: ThreadInfoSender) {
    match download(&url, tx).await {
        Ok(_) => {
            sender.info(format!("Downloaded: {}", url));
            sender.progress();
        }
        Err(e) => {
            sender.progress();
            sender.error(e);
        }
    }
}

async fn download(url: &str, tx: Sender<Option<DynamicImage>>) -> Result<()> {
    let bytes = get_request(url)
        .await
        .context(format!("Can't download picture: {}", url))?;
    let image = decode(&bytes)?;

    tx.send(Some(image))
        .context("Can't send picture to channel")?;

    Ok(())
}

fn decode(bytes: &[u8]) -> Result<DynamicImage> {
    ImageReader::with_format(Cursor::new(bytes), ImageFormat::Jpeg)
        .decode()
        .context("Can't decode image")
}

async fn get_request(url: &str) -> Result<Bytes> {
    let client = reqwest::Client::builder()
        .build()
        .context("Can't build client")?;

    let req = client.get(url).header(USER_AGENT, "dick from the mountain");

    let response = req.send().await.context("Can't send request")?;

    Ok(response
        .bytes()
        .await
        .context("Can't get bytes from request")?)
}
