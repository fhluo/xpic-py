use crate::util;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Serialize)]
pub struct Query {
    format: &'static str,

    #[serde(rename = "idx")]
    index: usize,

    #[serde(rename = "n")]
    number: usize,

    #[serde(rename = "mkt", skip_serializing_if = "Option::is_none")]
    market: Option<String>,
}

impl Query {
    pub fn new(format: &'static str, index: usize, number: usize, market: Option<String>) -> Self {
        Self {
            format,
            index,
            number,
            market,
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            format: "js",
            index: 0,
            number: 8,
            market: None,
        }
    }
}

#[derive(Deserialize)]
struct ImageInfo {
    #[serde(rename = "startdate")]
    start_date: String,

    url: String,
}

#[derive(Deserialize)]
struct ImagesResponse {
    images: Vec<ImageInfo>,
}

pub struct Image {
    url: Url,

    start_date: String,
}

impl From<ImageInfo> for Image {
    fn from(image_info: ImageInfo) -> Self {
        Self {
            url: Url::parse("https://cn.bing.com/")
                .unwrap()
                .join(image_info.url.as_str())
                .unwrap(),
            start_date: image_info.start_date,
        }
    }
}

pub struct ParsedID {
    pub name: String,
    pub market: String,
    pub number: usize,
    pub width: usize,
    pub height: usize,
    pub extension: String,
}

impl Default for ParsedID {
    fn default() -> Self {
        Self {
            name: String::default(),
            market: String::default(),
            number: 0,
            width: 0,
            height: 0,
            extension: String::default(),
        }
    }
}

impl From<&str> for ParsedID {
    fn from(id: &str) -> Self {
        let re = Regex::new(
            r"(?x)
^OHR
\.
(?P<name>\w+)
_
(?P<market>ROW|\w{2}-\w{2})
(?P<number>\d+)
_
(?P<width>\d+)
x
(?P<height>\d+)
\.
(?P<extension>\w+)$",
        )
        .unwrap();

        match re.captures(id) {
            Some(captures) => Self {
                name: String::from(&captures["name"]),
                market: String::from(&captures["market"]),
                number: captures["number"].parse::<usize>().unwrap(),
                width: captures["width"].parse::<usize>().unwrap(),
                height: captures["height"].parse::<usize>().unwrap(),
                extension: String::from(&captures["extension"]),
            },
            None => ParsedID::default(),
        }
    }
}

impl From<String> for ParsedID {
    fn from(id: String) -> Self {
        Self::from(id.as_str())
    }
}

impl Image {
    pub fn id(&self) -> String {
        let result = self.url.query_pairs().find(|(key, _)| key == "id");

        if let Some(id) = result {
            id.1.into_owned()
        } else {
            String::default()
        }
    }

    pub fn filename(&self) -> String {
        let parsed_id = ParsedID::from(self.id());

        format!(
            "{}_{}.{}",
            self.start_date, parsed_id.name, parsed_id.extension
        )
    }
}

pub async fn query(query: Query) -> Result<Vec<Image>, Box<dyn Error>> {
    let resp = reqwest::Client::new()
        .get("https://cn.bing.com/HPImageArchive.aspx")
        .query(&query)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(format!("failed to get images response: {}", resp.status()).into());
    }

    let images = resp
        .json::<ImagesResponse>()
        .await?
        .images
        .into_iter()
        .map(|info| Image::from(info))
        .collect::<Vec<_>>();

    Ok(images)
}

pub async fn get_images() -> Result<Vec<Url>, Box<dyn Error>> {
    Ok(query(Query::default())
        .await?
        .into_iter()
        .map(|image| image.url)
        .collect::<Vec<_>>())
}

/// Copies images to a specified directory.
pub async fn copy_images_to<P: AsRef<Path>>(dst: P) -> Result<(), Box<dyn Error>> {
    let dst = dst.as_ref();
    
    fs::create_dir_all(dst)
        .map_err(|err| format!("failed to create {}: {}", dst.display(), err))?;
    
    let tasks = query(Query::default())
        .await?
        .into_iter()
        .filter_map(|image| {
            let dst = dst.join(image.filename());
            if dst.exists() {
                return None;
            }

            Some(tokio::spawn(async move {
                if let Err(e) = util::download_file(&image.url, dst).await {
                    eprintln!("failed to download {}: {}", image.url, e);
                }
            }))
        });

    futures::future::join_all(tasks).await;
    Ok(())
}
