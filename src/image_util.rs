use image::io::{Reader as ImageReader, Reader};
use image::{DynamicImage, ImageFormat};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn new_image_reader<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>, Box<dyn Error>> {
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
pub fn open_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, Box<dyn Error>> {
    match new_image_reader(path)?.decode() {
        Ok(img) => Ok(img),
        Err(e) => Err(format!("failed to decode image: {}", e).into()),
    }
}

/// Returns image format.
pub fn get_image_format<P: AsRef<Path>>(path: P) -> Result<ImageFormat, Box<dyn Error>> {
    match new_image_reader(path) {
        Ok(reader) => match reader.format() {
            Some(format) => Ok(format),
            None => Err("failed to get image format".into()),
        },
        Err(e) => Err(format!("failed to get image format: {}", e).into()),
    }
}

/// Copies image from src to dst.
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
    set_extension: bool,
) -> Result<(), Box<dyn Error>> {
    let mut dst = PathBuf::from(dst.as_ref());

    if dst.is_dir() {
        let filename = match src.as_ref().file_name() {
            Some(filename) => filename,
            None => return Err("failed to get filename".into()),
        };

        dst = dst.join(filename);
    }

    if set_extension {
        dst.set_extension(
            get_image_format(src.as_ref())?
                .extensions_str()
                .first()
                .unwrap(),
        );
    }

    fs::copy(src, dst)?;
    Ok(())
}
