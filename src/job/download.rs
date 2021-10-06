use std::sync::mpsc::{Receiver, Sender};

use crate::job::get_request;

pub async fn job(rx: Receiver<String>, tx: Sender<Vec<u8>>, max_images: usize) {
    let mut i = 0;
    for url in rx {
        let bytes = download(&url).await.expect("Failed to download picture");
        tx.send(bytes).expect("Can't send bytes to channel");

        i += 1;
        if i == max_images {
            break;
        }
    }
}

async fn download(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = get_request(url).await?;

    Ok(response.bytes().await?.to_vec())
}
