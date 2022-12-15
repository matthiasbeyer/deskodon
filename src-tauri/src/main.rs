#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod browser;
use deskodon_types::login::LoginHandle;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crate::browser::open_browser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
