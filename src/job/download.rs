use std::sync::mpsc::{Receiver, Sender};

use futures::future::join_all;
use reqwest::header::USER_AGENT;
use reqwest::Response;

pub async fn job(rx: Receiver<Option<String>>, tx: Sender<Option<Vec<u8>>>) {
    let mut downloads: Vec<_> = vec![];

    for url in rx {
        match url {
            None => {
                join_all(downloads).await;
                tx.send(None).expect("Can't send end of download channel");
                break;
            }
            Some(url) => {
                let sender = tx.clone();
                downloads.push(download(url, sender));
            }
        }
    }
}

async fn download(url: String, tx: Sender<Option<Vec<u8>>>) {
    let response = get_request(&url).await.expect("Can't download picture");

    let result = response.bytes().await.expect("Can't get bytes from request").to_vec();
    tx.send(Some(result)).expect("Can't send picture to channel");
}

async fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder().build()
        .expect("Can't build client");
    println!("donwloading {}", url);

    let req = client
        .get(url)
        .header(USER_AGENT, "dick from the mountain");

    req.send().await
}