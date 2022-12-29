#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_log::{LogTarget, LoggerBuilder};

mod browser;
mod commands;
mod error;
mod mastodon;

fn main() {
    let app_state = crate::mastodon::MastodonState::default();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(
            LoggerBuilder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            crate::browser::open_browser,
            crate::commands::configuration_file_path,
            crate::commands::finalize_registration,
            crate::commands::load_mastodon,
            crate::commands::register,
            crate::commands::save_login,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
