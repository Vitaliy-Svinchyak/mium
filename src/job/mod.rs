use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, USER_AGENT};
use reqwest::Response;

pub mod parse;
pub mod download;
pub mod decode;

async fn get_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder().build().unwrap();

    let req = client
        .get(url)
        .header(USER_AGENT, "PostmanRuntime/7.28.4")
        .header(ACCEPT_LANGUAGE, "en-gb")
        .header(ACCEPT_ENCODING, "*")
        .header(ACCEPT, "*/*")
        .header(CONNECTION, "keep-alive");

    req.send().await
}