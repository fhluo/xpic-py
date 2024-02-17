use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod image_util;
mod spotlight;


#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List wallpapers
    List {},
    /// Save wallpapers
    Save {
        /// The directory where wallpapers are saved
        dir: PathBuf
    },
}

fn main() {
    let cli = CLI::parse();


    if let Some(command) = &cli.command {
        match command {
            Commands::List {} => spotlight::get_images()
                .unwrap()
                .into_iter()
                .for_each(|path| println!("{}", path.display())),
            Commands::Save { dir } => {
                spotlight::copy_images_to(dir, true).unwrap();
            }
        }
    }
}
