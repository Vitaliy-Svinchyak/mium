use std::sync::mpsc::{Receiver, Sender};

use anyhow::{Context, Error, Result};
use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

use crate::sync::thread_info_sender::ThreadInfoSender;

pub fn job(rx: Receiver<Option<String>>, tx: Sender<Option<String>>, sender: ThreadInfoSender) {
    for query in rx {
        match query {
            None => {
                match tx.send(None).context("Can't send end of channel") {
                    Ok(_) => {}
                    Err(e) => sender.error(e),
                }
                sender.closed();

                break;
            }
            Some(query) => {
                sender.info(format!("Parsing {}", query));

                match parse(&query) {
                    Ok(urls) => {
                        sender.info(format!("Parsed {}. Found {} pictures", query, urls.len()));
                        sender.progress();

                        for url in urls {
                            match tx.send(Some(url)).context("Can't send url to channel") {
                                Ok(_) => {}
                                Err(e) => sender.error(e),
                            }
                        }
                    }
                    Err(e) => {
                        sender.error(e);
                    }
                }
            }
        }
    }
}

fn parse(query: &str) -> Result<Vec<String>> {
    let response = get_request(query).context(format!("Failed to get query {}", query))?;
    let data = response
        .text()
        .context(format!("Failed to get data from request {}", query))?;
    let document = Html::parse_document(&data);
    let selector = Selector::parse("img.wallpapers__item__img").expect("Failed to parse selector");
    let pictures = document.select(&selector);
    let urls: Vec<String> = pictures
        .map(|v| v.value().attr("src"))
        .flatten()
        .map(|v| v.to_owned())
        .collect();

    Ok(urls)
}

fn get_request(url: &str) -> Result<Response> {
    let client = reqwest::blocking::Client::builder()
        .build()
        .context("Can't build client")?;

    let req = client.get(url).header(USER_AGENT, "dick from the mountain");

    req.send().map_err(Error::msg)
}
