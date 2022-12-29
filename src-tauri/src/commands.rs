use std::path::PathBuf;

use crate::mastodon::MastodonState;

use deskodon_types::authorization_code::AuthorizationCode;
use deskodon_types::error::Error;

#[tauri::command]
pub async fn configuration_file_path(
    state: tauri::State<'_, MastodonState>,
) -> Result<Option<PathBuf>, Error> {
    state.inner().state_file().await.map_err(Error::from)
}

#[tauri::command]
pub async fn load_mastodon(
    state: tauri::State<'_, MastodonState>,
    config_file: PathBuf,
) -> Result<(), Error> {
    state
        .inner()
        .load_from_file(config_file)
        .await
        .map_err(Error::from)
}

#[tauri::command]
pub async fn register(
    state: tauri::State<'_, MastodonState>,
    instance_url: url::Url,
) -> Result<String, Error> {
    state
        .inner()
        .register(instance_url)
        .await
        .map_err(Error::from)
}

#[tauri::command]
pub async fn finalize_registration(
    state: tauri::State<'_, MastodonState>,
    code: AuthorizationCode,
) -> Result<(), Error> {
    state
        .inner()
        .finalize_registration(code)
        .await
        .map_err(Error::from)
}

#[tauri::command]
pub async fn save_login(
    state: tauri::State<'_, MastodonState>,
) -> Result<(), Error> {
    state
        .inner()
        .save_login()
        .await
        .map_err(Error::from)
}
