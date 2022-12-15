use deskodon_types::login::LoginHandle;

#[tauri::command]
pub async fn login(name: &str) -> Result<LoginHandle, String> {
    Ok(LoginHandle::new(name.to_string()))
}
