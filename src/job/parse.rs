use std::sync::mpsc::Sender;

use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

pub fn job(query: String, tx: Sender<String>) {
    let urls = parse(query);

    for url in urls {
        tx.send(url).expect("Can't send url to channel");
    }
}

fn parse(query: String) -> Vec<String> {
    let response = get_request(&query).unwrap();
    let data = response.text().unwrap();
    let document = Html::parse_document(&data);
    let selector = Selector::parse("img.wallpapers__item__img").unwrap();
    let pictures = document.select(&selector);
    let urls: Vec<String> = pictures.map(|v| v.value().attr("src").unwrap().to_owned()).collect();

    urls
}


fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::blocking::Client::builder().build().unwrap();

    let req = client
        .get(url)
        .header(USER_AGENT, "dick from the mountain");

    req.send()
}

