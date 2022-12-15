use crate::state::State;

use deskodon_types::auth::Auth;
use deskodon_types::access_token::AccessToken;
use deskodon_types::login::LoginHandle;

#[tauri::command]
pub async fn login(_state: tauri::State<'_, State>, name: &str) -> Result<LoginHandle, String> {
    Ok(LoginHandle::new(name.to_string()))
}

#[tauri::command]
pub async fn generate_auth(state: tauri::State<'_, State>, instance: String) -> Result<Auth, String> {
    let instance = url::Url::parse(&instance).map_err(|e| e.to_string())?;
    crate::mastodon::generate_auth(instance).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_access_token(state: tauri::State<'_, State>, instance: String, client_id: String, client_secret: String, auth_token: String) -> Result<AccessToken, String> {
    let instance = url::Url::parse(&instance).map_err(|e| e.to_string())?;
    crate::mastodon::fetch_access_token(instance, client_id, client_secret, auth_token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_access_token(_state: tauri::State<'_, State>, _access_token: AccessToken) -> Result<(), String> {
    // TODO
    Ok(())
}
