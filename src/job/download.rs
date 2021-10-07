use std::sync::mpsc::{Receiver, Sender};

use futures::future::join_all;

use crate::job::get_request;

pub async fn job(rx: Receiver<String>, tx: Sender<Vec<u8>>, max_images: usize) {
    let mut i = 0;
    let mut downloads: Vec<_> = vec![];

    for url in rx {
        let sender = tx.clone();
        downloads.push(download(url, sender));

        i += 1;
        if i == max_images {
            join_all(downloads).await;
            break;
        }
    }
}

async fn download(url: String, tx: Sender<Vec<u8>>) {
    let response = get_request(&url).await.unwrap();

    let result = response.bytes().await.unwrap().to_vec();
    tx.send(result).unwrap();
}