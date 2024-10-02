use image::{GenericImageView, ImageReader};
use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_void;
use std::path::{Path, PathBuf};
use std::{env, vec};
use tauri::image::Image;
use tauri::{AppHandle, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use window_vibrancy::apply_mica;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoA, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

use xpic::{bing, spotlight};

fn get_cache_dir() -> PathBuf {
    env::var("LocalAppData").map_or(PathBuf::from(".cache"), |local_app_data| {
        PathBuf::from(local_app_data).join("Xpic").join(".cache")
    })
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
        if let Err(err) = SystemParametersInfoA(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_.as_ptr() as *mut c_void),
            SPIF_UPDATEINIFILE,
        ) {
            eprintln!("failed to set {} as desktop wallpaper: {}", path, err);
        }
    }
}

#[tauri::command]
async fn show_path_in_file_manager(path: String) {
    showfile::show_path_in_file_manager(path)
}

fn load_image(path: impl AsRef<Path>) -> Result<Image<'static>, Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let (width, height) = img.dimensions();

    Ok(Image::new_owned(img.into_rgba8().into_raw(), width, height))
}

fn write_image_to_clipboard(app_handle: AppHandle, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    app_handle.clipboard().write_image(&(load_image(path)?))?;

    Ok(())
}

#[tauri::command]
async fn copy_image(app_handle: AppHandle, path: String) -> Result<(), String> {
    write_image_to_clipboard(app_handle, path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                apply_mica(&window, Some(true))?;
            }

            window.set_decorations(true)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_wallpapers,
            update_wallpapers,
            set_as_desktop_wallpaper,
            show_path_in_file_manager,
            copy_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
