use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use anyhow::{Context, Result};
use futures_util::future::join_all;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

use crate::sync::thread_info_connection::ThreadInfoReceiver;
use crate::sync::thread_info_sender::ThreadInfoSender;
use std::time::Instant;

pub fn thread(rx: Receiver<Option<String>>, tx: Sender<Option<String>>) -> ThreadInfoReceiver {
    let (parse_log_tx, parse_log_rx) = channel();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            job(rx, tx, ThreadInfoSender::new(parse_log_tx)).await;
        });
    });

    ThreadInfoReceiver::new("Parse".to_owned(), "Parse".to_owned(), parse_log_rx)
}

async fn job(rx: Receiver<Option<String>>, tx: Sender<Option<String>>, sender: ThreadInfoSender) {
    let mut downloads = vec![];
    let start = Instant::now();

    for query in rx {
        match query {
            None => {
                join_all(downloads).await;
                let send_result = tx.send(None).context("Can't send end of parse channel");
                if let Err(e) = send_result {
                    sender.error(e);
                }
                sender.closed();

                break;
            }
            Some(query) => {
                let image_sender = tx.clone();
                let log_sender = sender.clone();
                sender.info(format!("Parsing {}", query));
                downloads.push(do_job(query, image_sender, log_sender));
            }
        }
    }

    sender.info(format!("Done in: {:?}", start.elapsed()));
}

async fn do_job(query: String, tx: Sender<Option<String>>, sender: ThreadInfoSender) {
    match parse(&query, tx).await {
        Ok(_) => {
            sender.info(format!("Parsed {}", query));
            sender.progress();
        }
        Err(e) => {
            sender.progress();
            sender.error(e);
        }
    }
}

async fn parse(query: &str, tx: Sender<Option<String>>) -> Result<()> {
    let data = get_request(query)
        .await
        .context(format!("Failed to get query {}", query))?;
    let urls = extract_urls(data);

    for url in urls {
        tx.send(Some(url)).context("Can't send url to channel")?;
    }

    Ok(())
}

fn extract_urls(data: String) -> Vec<String> {
    let document = Html::parse_document(&data);
    let selector = Selector::parse("img.wallpapers__item__img").expect("Failed to parse selector");
    let pictures = document.select(&selector);

    pictures
        .map(|v| v.value().attr("src"))
        .flatten()
        .map(|v| v.to_owned())
        .collect()
}

async fn get_request(url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .build()
        .context("Can't build client")?;

    let req = client.get(url).header(USER_AGENT, "dick from the mountain");

    let r = req.send().await.context("Can't send request")?;

    r.text()
        .await
        .context(format!("Failed to get data from request {}", url))
}
