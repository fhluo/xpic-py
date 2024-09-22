use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use xpic::{bing, spotlight};

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

fn list_spotlight_wallpapers() {
    if let Err(e) = spotlight::get_images().map(|images| {
        images.into_iter().for_each(|path| println!("{}", path.display()))
    }) {
        eprintln!("failed to get Windows Spotlight wallpapers: {e}")
    }
}

async fn list_bing_wallpapers() {
    if let Err(e) = bing::get_images().await.map(|images| {
        images.into_iter().for_each(|u| println!("{u}"))
    }) {
        eprintln!("failed to get Bing wallpapers: {e}");
    }
}

async fn list_wallpapers(spotlight: bool, bing: bool) {
    let all = !(spotlight || bing);

    let tasks = vec![
        if all || spotlight {
            Some(tokio::spawn(async { list_spotlight_wallpapers() }))
        } else {
            None
        },
        if all || bing {
            Some(tokio::spawn(list_bing_wallpapers()))
        } else {
            None
        },
    ]
        .into_iter()
        .filter_map(|handle| handle);

    futures::future::join_all(tasks).await;
}

fn save_spotlight_wallpapers<P: AsRef<Path>>(dir: P) {
    if let Err(e) = spotlight::copy_images_to(dir.as_ref()) {
        eprintln!(
            "failed to copy Windows Spotlight wallpapers to {}:{}",
            dir.as_ref().display(),
            e
        );
    }
}

async fn save_bing_wallpapers<P: AsRef<Path>>(dir: P) {
    if let Err(e) = bing::copy_images_to(&dir).await {
        eprintln!(
            "failed to copy Bing wallpapers to {}:{}",
            dir.as_ref().display(),
            e
        );
    }
}

async fn save_wallpapers(dir: &PathBuf, spotlight: bool, bing: bool) {
    let all = !(spotlight || bing);

    let tasks = vec![
        if all || spotlight {
            let dir = dir.clone();
            Some(tokio::spawn(async { save_spotlight_wallpapers(dir) }))
        } else {
            None
        },
        if all || bing {
            let dir = dir.clone();
            Some(tokio::spawn(save_bing_wallpapers(dir)))
        } else {
            None
        },
    ]
        .into_iter()
        .filter_map(|handle| handle);

    futures::future::join_all(tasks).await;
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::List { spotlight, bing } => list_wallpapers(*spotlight, *bing).await,
            Commands::Save {
                dir,
                spotlight,
                bing,
            } => save_wallpapers(dir, *spotlight, *bing).await,
        }
    }
}
