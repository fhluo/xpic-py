#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::raw::c_void;
use std::path::PathBuf;
use std::{env, vec};
use std::ffi::CString;
use tauri::Manager;
use window_vibrancy::apply_mica;
use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoA, SystemParametersInfoW, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER};
use xpic::{bing, spotlight};

fn get_cache_dir() -> PathBuf {
    return if let Some(dir) = tauri::api::path::local_data_dir() {
        dir.join("Xpic").join(".cache")
    } else {
        PathBuf::from(".cache")
    };
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
    ])
    .await;
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
    get_cached_images()
        .into_iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<_>>()
}

#[tauri::command]
async fn update_wallpapers() -> Vec<String> {
    cache_images().await;
    get_wallpapers().await
}

#[tauri::command]
async fn set_as_desktop_wallpaper(path: String) {
    let path_ = CString::new(path.to_owned()).unwrap();

    unsafe {
        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
        if let Err(err) = SystemParametersInfoA(SPI_SETDESKWALLPAPER, 0, Some(path_.as_ptr() as *mut c_void), SPIF_UPDATEINIFILE) {
            eprintln!("failed to set {} as desktop wallpaper: {}", path, err);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                apply_mica(&window, Some(true))?;
            }

            window.set_decorations(true)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_wallpapers, update_wallpapers, set_as_desktop_wallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
