use std::path::PathBuf;

use crate::mastodon::MastodonState;

use deskodon_types::error::Error;

#[tauri::command]
pub async fn configuration_file_path(state: tauri::State<'_, MastodonState>) -> Result<Option<PathBuf>, Error> {
    state.inner().state_file().await.map_err(Error::from)
}

#[tauri::command]
pub async fn load_mastodon(state: tauri::State<'_, MastodonState>, config_file: PathBuf) -> Result<(), Error> {
    state.inner().load_from_file(config_file).await.map_err(Error::from)
}
