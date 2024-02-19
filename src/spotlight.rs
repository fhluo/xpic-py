use std::error::Error;
use std::{env, fs};

use image::GenericImageView;
use std::path::{Path, PathBuf};

use crate::util;

/// Returns assets.
pub fn get_assets() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let local_app_data = match env::var("LocalAppData") {
        Ok(path) => PathBuf::from(path),
        Err(e) => return Err(format!("failed to get LocalAppData: {}", e).into()),
    };

    let pattern = local_app_data.join(r"Packages\*ContentDeliveryManager*\LocalState\Assets\*");

    let files = glob::glob(pattern.to_str().unwrap())?
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    Ok(files)
}

/// Returns images(width >= 1920 and height >= 1080).
pub fn get_images() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let assets = match get_assets() {
        Ok(assets) => assets,
        Err(e) => return Err(format!("failed to get assets: {}", e).into()),
    };

    let images = assets
        .into_iter()
        .filter_map(|path| match util::open_image(&path) {
            Ok(img) => {
                if img.dimensions() >= (1920, 1080) {
                    Some(path)
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("failed to open image: {}", e);
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(images)
}

/// Copies images to a specified directory.
pub fn copy_images_to<P: AsRef<Path>>(dst: P) -> Result<(), Box<dyn Error>> {
    if let Err(err) = fs::create_dir_all(&dst) {
        return Err(format!("failed to create {}: {}", dst.as_ref().display(), err).into());
    }

    match get_images() {
        Ok(images) => images.into_iter().for_each(|path| {
            if let Err(err) = util::copy_image(&path, dst.as_ref(), true) {
                eprintln!(
                    "failed to copy image from {} to {}: {}",
                    path.display(),
                    dst.as_ref().display(),
                    err
                )
            }
        }),
        Err(err) => return Err(format!("failed to get images: {}", err).into()),
    }

    Ok(())
}
