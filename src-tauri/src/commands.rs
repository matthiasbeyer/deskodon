use std::path::PathBuf;

use crate::mastodon::MastodonState;

use deskodon_types::authorization_code::AuthorizationCode;
use deskodon_types::error::Error;
use mastodon_async::entities::status::Status;

trait LogResult<T> {
    fn log_result(self) -> Result<T, Error>;
}

impl<T> LogResult<T> for Result<T, Error>
where
    T: std::fmt::Debug,
{
    fn log_result(self) -> Result<T, Error> {
        #[cfg(debug_assertions)]
        match self {
            Ok(ok) => {
                log::debug!("Ok({:?})", ok);
                Ok(ok)
            }
            Err(e) => {
                log::error!("Err({:?}", e);
                Err(e)
            }
        }
        #[cfg(not(debug_assertions))]
        self
    }
}

#[tauri::command]
pub async fn configuration_file_path(
    state: tauri::State<'_, MastodonState>,
) -> Result<Option<PathBuf>, Error> {
    state
        .inner()
        .state_file()
        .await
        .map_err(Error::from)
        .log_result()
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
        .log_result()
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
        .log_result()
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
        .log_result()
}

#[tauri::command]
pub async fn save_login(state: tauri::State<'_, MastodonState>) -> Result<(), Error> {
    state
        .inner()
        .save_login()
        .await
        .map_err(Error::from)
        .log_result()
}

#[tauri::command]
pub async fn get_current_statuses(
    state: tauri::State<'_, MastodonState>,
) -> Result<Vec<Status>, Error> {
    state
        .inner()
        .get_current_statuses()
        .await
        .map_err(Error::from)
        .log_result()
}
