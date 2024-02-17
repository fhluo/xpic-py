use std::path::PathBuf;

mod image_util;
mod spotlight;

fn main() {
    spotlight::get_images()
        .unwrap()
        .into_iter()
        .for_each(|path| println!("{}", path.display()));

    spotlight::copy_images_to(&PathBuf::from("./temp/images"), true).unwrap();
}
