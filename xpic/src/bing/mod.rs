mod query;

use crate::bing::query::{query, ImageInfo, Query};
use crate::util;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub url: Url,
    pub date: String,
    pub title: String,
    pub copyright: String,
}

impl TryFrom<ImageInfo> for Image {
    type Error = Box<dyn Error>;

    fn try_from(info: ImageInfo) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?x)
(?P<title>.*?)
\s*\(
(?P<copyright>.*?)
\)
")?;

        let captures = re.captures(&info.copyright).ok_or("")?;

        let r = Self {
            url: Url::parse("https://www.bing.com/")?
                .join(&info.url)?,
            date: info.start_date,
            title: captures["title"].to_string(),
            copyright: captures["copyright"].to_string(),
        };

        Ok(r)
    }
}

#[derive(Debug, PartialEq)]
pub struct ImageDetail {
    pub name: String,
    pub market: String,
    pub number: usize,
    pub uhd: bool,
    pub width: usize,
    pub height: usize,
    pub extension: String,
}

impl Default for ImageDetail {
    fn default() -> Self {
        Self {
            name: String::default(),
            market: String::default(),
            number: 0,
            uhd: false,
            width: 0,
            height: 0,
            extension: String::default(),
        }
    }
}

impl FromStr for ImageDetail {
    type Err = Box<dyn Error>;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?x)
^OHR
\.
(?P<name>\w+)
_
(?P<market>ROW|\w{2}-\w{2})
(?P<number>\d+)
_
(
(?P<width>\d+)x(?P<height>\d+)
|
(?P<uhd>UHD)
)
\.
(?P<extension>\w+)$",
        )?;

        let r = match re.captures(id) {
            Some(captures) => {
                let uhd = captures.name("uhd").is_some();
                Self {
                    name: String::from(&captures["name"]),
                    market: String::from(&captures["market"]),
                    number: captures["number"].parse::<usize>()?,
                    uhd,
                    width: if uhd {
                        0
                    } else {
                        captures["width"].parse::<usize>()?
                    },
                    height: if uhd {
                        0
                    } else {
                        captures["height"].parse::<usize>()?
                    },
                    extension: String::from(&captures["extension"]),
                }
            }
            None => Default::default(),
        };

        Ok(r)
    }
}

impl Image {
    pub fn id(&self) -> Option<String> {
        self.url
            .query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, id)| id.into_owned())
    }

    pub fn detail(&self) -> Result<ImageDetail, Box<dyn Error>> {
        self.id().as_deref().unwrap_or("").parse()
    }
}


pub async fn get_images() -> Result<Vec<Url>, Box<dyn Error>> {
    Ok(query(Query::default())
        .await?
        .into_iter()
        .filter_map(|info| Image::try_from(info).ok())
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
            let image = Image::try_from(image).ok()?;
            let dst = dst.join(image.id()?);
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
