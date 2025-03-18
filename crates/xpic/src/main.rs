use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use xpic::{bing, spotlight};

#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

impl CLI {
    async fn run(self) {
        self.command.run().await;
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Bing wallpapers
    #[command(subcommand)]
    Bing(Bing),
    /// Windows Spotlight wallpapers
    #[command(subcommand)]
    Spotlight(Spotlight),
}

impl Commands {
    async fn run(self) {
        match self {
            Commands::Bing(command) => command.run().await,
            Commands::Spotlight(command) => command.run(),
        }
    }
}

#[derive(Subcommand)]
enum Bing {
    /// List Bing wallpapers
    List {
        /// The number of wallpapers to list
        #[arg(short)]
        number: Option<usize>,
    },
    /// Save wallpapers to a directory
    Save {
        /// The directory where wallpapers are saved
        dir: PathBuf,
    },
}

impl Bing {
    async fn run(self) {
        match self {
            Bing::List { number } => Self::list(number).await,
            Bing::Save { dir } => Self::save(dir).await,
        }
    }

    async fn list(number: Option<usize>) {
        match bing::get_images().await {
            Ok(images) => {
                if let Some(number) = number {
                    for url in images.into_iter().take(number) {
                        println!("{url}");
                    }
                } else {
                    for url in images {
                        println!("{url}");
                    }
                }
            }
            Err(err) => eprintln!("failed to get Bing wallpapers: {err}"),
        }
    }

    async fn save(dir: impl AsRef<Path>) {
        if let Err(err) = bing::copy_images_to(&dir).await {
            eprintln!(
                "failed to copy Bing wallpapers to {}:{}",
                dir.as_ref().display(),
                err
            );
        }
    }
}

#[derive(Subcommand)]
enum Spotlight {
    /// List Windows Spotlight wallpapers
    List {
        /// The number of wallpapers to list
        #[arg(short)]
        number: Option<usize>,
    },
    /// Save wallpapers to a directory
    Save {
        /// The directory where wallpapers are saved
        dir: PathBuf,
    },
}

impl Spotlight {
    fn run(self) {
        match self {
            Spotlight::List { number } => Self::list(number),
            Spotlight::Save { dir } => Self::save(dir),
        }
    }

    fn list(number: Option<usize>) {
        match spotlight::get_images() {
            Ok(images) => {
                if let Some(number) = number {
                    for path in images.into_iter().take(number) {
                        println!("{}", path.display());
                    }
                } else {
                    for path in images {
                        println!("{}", path.display());
                    }
                }
            }
            Err(err) => eprintln!("failed to get Windows Spotlight wallpapers: {err}"),
        }
    }

    fn save(dir: impl AsRef<Path>) {
        if let Err(err) = spotlight::copy_images_to(&dir) {
            eprintln!(
                "failed to copy Windows Spotlight wallpapers to {}:{}",
                dir.as_ref().display(),
                err
            );
        }
    }
}

#[tokio::main]
async fn main() {
    CLI::parse().run().await;
}
