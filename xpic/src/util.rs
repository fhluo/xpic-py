use image::{DynamicImage, ImageFormat, ImageReader};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{fs, io};
use url::Url;

/// Returns image reader with guessed format.
fn new_image_reader<P: AsRef<Path>>(
    path: P,
) -> Result<ImageReader<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(&path).map_err(|e| format!("failed to open file: {e}"))?;

    ImageReader::new(BufReader::new(file))
        .with_guessed_format()
        .map_err(|e| format!("failed to read image: {e}").into())
}

/// Opens image with guessed format.
pub fn open_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, Box<dyn Error>> {
    new_image_reader(path)?
        .decode()
        .map_err(|e| format!("failed to decode image: {e}").into())
}

/// Returns image format.
pub fn get_image_format<P: AsRef<Path>>(path: P) -> Result<ImageFormat, Box<dyn Error>> {
    new_image_reader(path)?
        .format()
        .ok_or_else(|| "failed to get image format".into())
}

/// Copies image from src to dst.
pub fn copy_image<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
    set_extension: bool,
) -> Result<(), Box<dyn Error>> {
    if dst.as_ref().exists() {
        return Ok(());
    }

    // If dst is a directory, append src filename to dst.
    let mut dst = if dst.as_ref().is_dir() {
        dst.as_ref()
            .join(src.as_ref().file_name().ok_or("failed to get filename")?)
    } else {
        PathBuf::from(dst.as_ref())
    };

    // Set dst extension to match src image format.
    if set_extension {
        dst.set_extension(
            get_image_format(src.as_ref())?
                .extensions_str()
                .first()
                .ok_or("failed to get image extension")?,
        );
    }

    fs::copy(src, dst)?;
    Ok(())
}

/// Downloads file from url to dst.
pub async fn download_file<P: AsRef<Path>>(url: &Url, dst: P) -> Result<(), Box<dyn Error>> {
    if dst.as_ref().exists() {
        return Ok(());
    }

    let resp = reqwest::get(url.as_ref()).await?;

    if !resp.status().is_success() {
        return Err(format!("failed to download file from {url}").into());
    }

    let mut file = File::create(dst)?;
    let content = resp.bytes().await?;
    io::copy(&mut content.as_ref(), &mut file)?;

    Ok(())
}
