use std::path::PathBuf;

use tokio::io::AsyncWriteExt;

use crate::error::ApplicationError;

pub struct State {
    path: PathBuf,
    state_inner: StateInner,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum StateInner {
    None,

    WaitingForAuthorizationUrl {
        url: url::Url,
    }
}

impl State {
    pub async fn load_from_path(
        path: PathBuf,
        gui: deskodon_frontend::GuiHandle
    ) -> Result<Self, ApplicationError> {
        if path.exists() {
            tracing::debug!(path = %path.display(), "State file exists");
            let text = tokio::fs::read_to_string(&path)
                .await
                .map_err(|error| ApplicationError::ReadingState {
                    error,
                    path: path.to_path_buf(),
                })?;

            tracing::debug!(?text, "State file read");
            toml::from_str(&text)
                .map_err(ApplicationError::ParsingState)
                .map(|state_inner| State { path, state_inner })
        } else {
            tracing::debug!(path = %path.display(), "State file does not exist");
            let state_dir = path.parent().ok_or_else(|| ApplicationError::FindingStateDirName {
                path: path.to_path_buf(),
            })?;

            let _ = tokio::fs::create_dir_all(state_dir)
                .await
                .map_err(|error| ApplicationError::CreatingStateDir {
                    error,
                    path: path.to_path_buf(),
                })?;

            let _ = tokio::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .append(false)
                .create(true)
                .open(&path)
                .await
                .map_err(|source| ApplicationError::OpenStateFile {
                    path: path.to_path_buf(),
                    source,
                })?;

            Ok(State {
                path,
                state_inner: StateInner::None,
            })
        }
    }

    pub async fn save(&self) -> Result<(), ApplicationError> {
        tracing::debug!(path = %self.path.display(), "Saving State file");
        let ser = toml::to_string(&self.state_inner).map_err(ApplicationError::SerializingState)?;

        tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .append(false)
            .create(true)
            .open(&self.path)
            .await
            .map_err(|source| ApplicationError::OpenStateFile {
                path: self.path.to_path_buf(),
                source,
            })?
            .write_all(ser.as_bytes())
            .await
            .map_err(|source| ApplicationError::WritingState {
                path: self.path.to_path_buf(),
                source,
            })
    }

    pub fn set_to_waiting_for_auth(&mut self, url: url::Url) {
        self.state_inner = StateInner::WaitingForAuthorizationUrl { url };
    }
}
