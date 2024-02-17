use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use image::{DynamicImage, GenericImageView};
use image::io::Reader as ImageReader;

/// Returns assets.
pub fn get_assets() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let local_app_data = match env::var("LocalAppData") {
        Ok(path) => PathBuf::from(path),
        Err(e) => return Err(format!("failed to get LocalAppData: {}", e).into())
    };

    let pattern = local_app_data
        .join(r"Packages\*ContentDeliveryManager*\LocalState\Assets\*");

    let files = glob::glob(pattern.to_str().unwrap())?
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    Ok(files)
}

/// Opens image with guessed format.
fn open_image(path: &PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("failed to open file: {}", e).into());
        }
    };

    let reader = match ImageReader::new(BufReader::new(file))
        .with_guessed_format() {
        Ok(reader) => reader,
        Err(e) => {
            return Err(format!("failed to read image: {}", e).into());
        }
    };

    let img = match reader.decode() {
        Ok(img) => img,
        Err(e) => {
            return Err(format!("failed to decode image: {}", e).into());
        }
    };

    Ok(img)
}

/// Returns images(width >= 1920 and height >= 1080).
pub fn get_images() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let assets = match get_assets() {
        Ok(assets) => assets,
        Err(e) => return Err(format!("failed to get assets: {}", e).into())
    };

    let images = assets
        .into_iter()
        .filter_map(|path| {
            match open_image(&path) {
                Ok(img) =>
                    if img.dimensions() >= (1920, 1080) {
                        Some(path)
                    } else {
                        None
                    }
                ,
                Err(e) => {
                    eprintln!("failed to open image: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(images)
}
