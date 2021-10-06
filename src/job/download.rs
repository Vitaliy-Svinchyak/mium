use crate::job::get_request;

pub async fn job(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let response = get_request(&url).await?;

    Ok(response.bytes().await?.to_vec())
}
