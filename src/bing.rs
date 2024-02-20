use crate::util;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use url::Url;

#[derive(Serialize)]
struct Query {
    format: &'static str,
    #[serde(rename = "idx")]
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
    let query = Query {
        format: "js",
        index,
        number,
    };

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

pub async fn get_images() -> Result<Vec<Url>, Box<dyn Error>> {
    Ok(query(0, 8).await?)
}

/// Copies images to a specified directory.
pub async fn copy_images_to<P: AsRef<Path>>(dst: P) -> Result<(), Box<dyn Error>> {
    let tasks = get_images().await?.into_iter().filter_map(|url| {
        if let Some(id) = url.query_pairs().find(|(key, _)| key == "id") {
            let dst = dst.as_ref().join(id.1.into_owned());
            if dst.exists() {
                return None;
            }

            Some(tokio::spawn(async move {
                util::download_file(&url, dst).await.unwrap_or_else(|e| {
                    eprintln!("failed to download {url}: {e}");
                })
            }))
        } else {
            eprintln!("The query parameter id to be used as filename does not exist.");
            None
        }
    });

    futures::future::join_all(tasks).await;
    Ok(())
}
