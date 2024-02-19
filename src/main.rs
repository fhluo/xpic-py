use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod bing;
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
    /// List all wallpapers if no flags are specified
    List {
        /// List Windows Spotlight wallpapers
        #[arg(long)]
        spotlight: bool,

        /// List Bing wallpapers
        #[arg(long)]
        bing: bool,
    },
    /// Save all wallpapers if no flags are specified
    Save {
        /// The directory where wallpapers are saved
        dir: PathBuf,

        /// Save Windows Spotlight wallpapers
        #[arg(long)]
        spotlight: bool,

        /// Save Bing wallpapers
        #[arg(long)]
        bing: bool,
    },
}

fn list_wallpapers(spotlight: bool, bing: bool) {
    let all = !spotlight && !bing;

    if all || spotlight {
        spotlight::get_images()
            .unwrap()
            .into_iter()
            .for_each(|path| println!("{}", path.display()));
    }

    if all || bing {}
}

fn save_wallpapers(dir: &PathBuf, spotlight: bool, bing: bool) {
    let all = !spotlight && !bing;

    if all || spotlight {
        spotlight::copy_images_to(dir, true).unwrap()
    }
    if all || bing {}
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::List { spotlight, bing } => list_wallpapers(*spotlight, *bing),
            Commands::Save {
                dir,
                spotlight,
                bing,
            } => save_wallpapers(dir, *spotlight, *bing),
        }
    }
}
