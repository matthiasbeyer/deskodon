#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use deskodon_types::login::LoginHandle;

#[tauri::command]
pub async fn login(name: &str) -> Result<LoginHandle, String> {
    Ok(LoginHandle::new(name.to_string()))
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
