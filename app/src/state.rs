use std::path::PathBuf;

use tokio::io::AsyncWriteExt;

use crate::error::ApplicationError;

pub struct State {
    path: PathBuf,
    state_inner: StateInner,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct StateInner {}

impl State {
    pub async fn load_from_path(
        path: PathBuf,
        gui: deskodon_frontend::GuiHandle
    ) -> Result<Self, ApplicationError> {
        let text = if path.exists() {
            tokio::fs::read_to_string(&path)
                .await
                .map_err(|error| ApplicationError::ReadingState {
                    error,
                    path: path.to_path_buf(),
                })?
        } else {
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

            String::new()
        };

        toml::from_str(&text)
            .map_err(ApplicationError::ParsingState)
            .map(|state_inner| State { path, state_inner })
    }

    pub async fn save(&self) -> Result<(), ApplicationError> {
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
}
