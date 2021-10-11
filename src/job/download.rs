use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};

use futures::future::join_all;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use reqwest::header::USER_AGENT;
use reqwest::Response;

use crate::sync::thread_info_sender::ThreadInfoSender;

pub async fn job(
    rx: Receiver<Option<String>>,
    tx: Sender<Option<DynamicImage>>,
    sender: ThreadInfoSender,
) {
    let mut downloads: Vec<_> = vec![];

    for url in rx {
        match url {
            None => {
                join_all(downloads).await;
                tx.send(None).expect("Can't send end of download channel");
                sender.closed();

                break;
            }
            Some(url) => {
                let image_sender = tx.clone();
                let log_sender = sender.clone();
                sender.info(format!("Downloading: {}", url));
                downloads.push(download(url, image_sender, log_sender));
            }
        }
    }
}

async fn download(url: String, tx: Sender<Option<DynamicImage>>, sender: ThreadInfoSender) {
    let response = get_request(&url).await.expect("Can't download picture");

    let bytes = response
        .bytes()
        .await
        .expect("Can't get bytes from request")
        .to_vec();
    let image = decode(bytes);

    sender.info(format!("Downloaded: {}", url));
    sender.progress();

    tx.send(Some(image)).expect("Can't send picture to channel");
}

fn decode(bytes: Vec<u8>) -> DynamicImage {
    ImageReader::with_format(Cursor::new(bytes.as_slice()), ImageFormat::Jpeg)
        .decode()
        .expect("Can't decode image")
}

async fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()
        .expect("Can't build client");

    let req = client.get(url).header(USER_AGENT, "dick from the mountain");

    req.send().await
}
