use std::sync::mpsc::Sender;

use scraper::{Html, Selector};

use crate::job::get_request;

pub async fn job(query: String, tx: Sender<String>) {
    let urls = parse(query).await;

    for url in urls {
        tx.send(url).expect("Can't send url to channel");
    }
}

async fn parse(query: String) -> Vec<String> {
    let response = get_request(&query).await.unwrap();
    let data = response.text().await.unwrap();
    let document = Html::parse_document(&data);
    let selector = Selector::parse("img.wallpapers__item__img").unwrap();
    let pictures = document.select(&selector);
    let urls: Vec<String> = pictures.map(|v| v.value().attr("src").unwrap().to_owned()).collect();

    urls
}


