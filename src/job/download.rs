use std::sync::mpsc::{Receiver, Sender};

use futures::future::join_all;

use crate::job::get_request;

pub async fn job(rx: Receiver<String>, tx: Sender<Vec<u8>>, max_images: usize) {
    let mut i = 0;
    let mut downloads: Vec<_> = vec![];

    for url in rx {
        downloads.push(download(url));

        i += 1;
        if i == max_images {
            for bytes in join_all(downloads).await {
                tx.send(
                    bytes.expect("Failed to download picture")
                ).expect("Can't send bytes to channel");
            }
            break;
        }
    }
}

async fn download(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let response = get_request(&url).await?;

    Ok(response.bytes().await?.to_vec())
}
