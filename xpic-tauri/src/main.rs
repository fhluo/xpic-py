#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, vec};
use std::path::{PathBuf};
use xpic::{bing, spotlight};

fn get_cache_dir() -> PathBuf {
    PathBuf::from(env::var("LocalAppData").unwrap()).join("Xpic\\Cache")
}

async fn cache_images() {
    let dir = get_cache_dir();

    futures::future::join_all(vec![
        {
            let dir = dir.to_owned();
            tokio::spawn(async move {
                if let Err(e) = spotlight::copy_images_to(&dir) {
                    eprintln!(
                        "failed to copy Windows Spotlight wallpapers to {}: {}",
                        dir.display(),
                        e
                    );
                }
            })
        },
        {
            let dir = dir.to_owned();
            tokio::spawn(async move {
                if let Err(e) = bing::copy_images_to(&dir).await {
                    eprintln!("failed to copy Bing wallpapers to {}:{}", dir.display(), e);
                }
            })
        },
    ]).await;
}

fn get_cached_images() -> Vec<PathBuf> {
    let pattern = get_cache_dir().join("*.*");

    glob::glob(pattern.to_string_lossy().as_ref())
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}


#[tauri::command]
async fn get_wallpapers() -> Vec<String> {
    cache_images().await;
    
    get_cached_images()
        .into_iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<_>>()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_wallpapers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
