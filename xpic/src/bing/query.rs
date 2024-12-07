use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub format: &'static str,

    #[serde(rename = "idx")]
    pub index: usize,

    #[serde(rename = "n")]
    pub number: usize,

    #[serde(rename = "mkt", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,

    #[serde(
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub uhd: Option<bool>,

    #[serde(rename = "uhdwidth", skip_serializing_if = "Option::is_none")]
    pub uhd_width: Option<usize>,

    #[serde(rename = "uhdheight", skip_serializing_if = "Option::is_none")]
    pub uhd_height: Option<usize>,

    #[serde(
        rename = "ensearch",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub en_search: Option<bool>,

    #[serde(rename = "setmkt", skip_serializing_if = "Option::is_none")]
    pub set_market: Option<String>,

    #[serde(rename = "setlang", skip_serializing_if = "Option::is_none")]
    pub set_lang: Option<String>,
}

fn bool_to_int<S>(b: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_i32(if b.unwrap_or(false) { 1 } else { 0 })
}

fn int_to_bool<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i: Option<i32> = Option::deserialize(d)?;

    Ok(i.map(|i| i == 1))
}

impl Query {
    pub fn new(format: &'static str, index: usize, number: usize) -> Self {
        Self {
            format,
            index,
            number,
            ..Self::default()
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
            uhd: Some(true),
            uhd_width: Some(3840),
            uhd_height: Some(2160),
            en_search: Some(true),
            set_market: Some("en-US".to_string()),
            set_lang: Some("en-US".to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "startdate")]
    pub start_date: String,

    #[serde(rename = "fullstartdate")]
    pub full_start_date: String,

    #[serde(rename = "enddate")]
    pub end_date: String,

    pub url: String,

    #[serde(rename = "urlbase")]
    pub url_base: String,

    pub copyright: String,

    #[serde(rename = "copyrightlink")]
    pub copyright_link: String,

    pub title: String,

    #[serde(rename = "quiz")]
    pub quiz_link: String,

    #[serde(rename = "wp")]
    pub wallpaper: bool,

    #[serde(rename = "hsh")]
    pub hash: String,

    #[serde(rename = "drk")]
    pub dark: isize,

    #[serde(rename = "top")]
    pub top: isize,

    #[serde(rename = "bot")]
    pub bottom: isize,

    #[serde(rename = "hs")]
    pub hotspots: Vec<Value>,
}

#[derive(Deserialize)]
struct ImagesResponse {
    images: Vec<ImageInfo>,
}

pub async fn query(query: Query) -> Result<Vec<ImageInfo>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    // Home Page Image Archive
    let request = client
        .get("https://global.bing.com/HPImageArchive.aspx")
        .query(&query)
        .build()?;

    let resp = client.execute(request).await?;

    if !resp.status().is_success() {
        return Err(format!("failed to get images response: {}", resp.status()).into());
    }

    let images = resp
        .json::<ImagesResponse>()
        .await?
        .images;

    Ok(images)
}
