#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_log::{LogTarget, LoggerBuilder};

mod browser;

fn main() {
    tauri::Builder::default()
        .plugin(
            LoggerBuilder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![crate::browser::open_browser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
