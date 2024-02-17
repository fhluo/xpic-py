use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use image::io::{Reader as ImageReader, Reader};
use image::{DynamicImage, ImageFormat};

fn new_image_reader(path: &PathBuf) -> Result<Reader<BufReader<File>>, Box<dyn Error>> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("failed to open file: {}", e).into());
        }
    };

    match ImageReader::new(BufReader::new(file)).with_guessed_format() {
        Ok(reader) => Ok(reader),
        Err(e) => Err(format!("failed to read image: {}", e).into()),
    }
}

/// Opens image with guessed format.
pub fn open_image(path: &PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    match new_image_reader(path)?.decode() {
        Ok(img) => Ok(img),
        Err(e) => Err(format!("failed to decode image: {}", e).into()),
    }
}

/// Returns image format.
pub fn get_image_format(path: &PathBuf) -> Option<ImageFormat> {
    match new_image_reader(path) {
        Ok(reader) => reader.format(),
        Err(_) => None,
    }
}
