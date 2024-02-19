use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Serialize)]
struct Query {
    format: &'static str,
    #[serde(rename ="idx")]
    index: usize,
    #[serde(rename = "n")]
    number: usize,
}

#[derive(Deserialize)]
struct ImageInfo {
    url: String,
}

#[derive(Deserialize)]
struct ImagesResponse {
    images: Vec<ImageInfo>,
}

pub async fn query(index: usize, number: usize) -> Result<Vec<Url>, Box<dyn Error>> {
    let query = Query { format: "js", index, number };

    let resp = reqwest::Client::new()
        .get("https://cn.bing.com/HPImageArchive.aspx")
        .query(&query)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(format!("failed to get images response: {}", resp.status()).into());
    }

    let base_url = Url::parse("https://cn.bing.com/")?;
    let urls = resp
        .json::<ImagesResponse>()
        .await?
        .images
        .into_iter()
        .map(|image| base_url.join(image.url.as_str()).unwrap())
        .collect::<Vec<_>>();

    Ok(urls)
}
