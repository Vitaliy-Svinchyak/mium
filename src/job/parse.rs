use std::sync::mpsc::{Receiver, Sender};

use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

pub fn job(rx: Receiver<Option<String>>, tx: Sender<Option<String>>) {
    for query in rx {
        match query {
            None => {
                tx.send(None).expect("Can't send end of channel");
            }
            Some(query) => {
                let urls = parse(query);

                for url in urls {
                    tx.send(Some(url)).expect("Can't send url to channel");
                }
            }
        }
    }
}

fn parse(query: String) -> Vec<String> {
    let response = get_request(&query)
        .expect("Failed to get query");
    let data = response.text()
        .expect("Failed to get data from request");
    let document = Html::parse_document(&data);
    let selector = Selector::parse("img.wallpapers__item__img")
        .expect("Failed to parse selector");
    let pictures = document.select(&selector);
    let urls: Vec<String> = pictures.map(
        |v| v.value().attr("src")
            .expect("Failed to get src value")
            .to_owned()
    ).collect();

    urls
}


fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::blocking::Client::builder().build().expect("Can't build client");

    let req = client
        .get(url)
        .header(USER_AGENT, "dick from the mountain");

    req.send()
}

