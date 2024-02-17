mod spotlight;

fn main() {
    spotlight::get_images()
        .unwrap()
        .into_iter()
        .for_each(|path|
            println!("{}", path.into_os_string().to_str().unwrap())
        );
}
