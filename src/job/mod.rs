use reqwest::header::USER_AGENT;
use reqwest::Response;

pub mod parse;
pub mod download;
pub mod decode;
pub mod accumulate;

async fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder().build().unwrap();

    let req = client
        .get(url)
        .header(USER_AGENT, "dick from the mountain");

    req.send().await
}