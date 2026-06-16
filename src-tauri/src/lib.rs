mod commands;
mod config;
mod error;

use tauri::Manager;
use tracing_subscriber::EnvFilter;

use crate::commands::open_url::open_url;
use crate::commands::settings::{
    settings_clear_credentials, settings_get, settings_path, settings_set,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with_target(false)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("Simple Podcast Studio");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_url,
            settings_get,
            settings_set,
            settings_clear_credentials,
            settings_path,
        ])
        .run(tauri::generate_context!())
        .expect("error al iniciar Simple Podcast Studio");
}
