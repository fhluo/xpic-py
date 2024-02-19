use crate::bing::query;
use clap::{Parser, Subcommand};
use std::error::Error;
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

async fn list_wallpapers(spotlight: bool, bing: bool) -> Result<(), Box<dyn Error>> {
    let all = !(spotlight || bing);

    if all || spotlight {
        spotlight::get_images()?
            .into_iter()
            .for_each(|path| println!("{}", path.display()));
    }

    if all || bing {
        query(0, 8)
            .await?
            .into_iter()
            .for_each(|u| println!("{}", u));
    }

    Ok(())
}

async fn save_wallpapers(dir: &PathBuf, spotlight: bool, bing: bool) -> Result<(), Box<dyn Error>> {
    let all = !(spotlight || bing);

    if all || spotlight {
        spotlight::copy_images_to(dir, true)?;
    }
    if all || bing {}

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = CLI::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::List { spotlight, bing } => list_wallpapers(*spotlight, *bing).await?,
            Commands::Save {
                dir,
                spotlight,
                bing,
            } => save_wallpapers(dir, *spotlight, *bing).await?,
        }
    }

    Ok(())
}
