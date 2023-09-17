use std::path::PathBuf;

use tokio::io::AsyncWriteExt;

use crate::error::Error;

pub struct State {
    path: PathBuf,
    state_inner: StateInner,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct StateInner {}

impl State {
    pub async fn load_from_path(path: PathBuf) -> Result<Self, Error> {
        let text = match tokio::fs::read_to_string(&path).await {
            Err(error) => {
                if let std::io::ErrorKind::NotFound = error.kind() {
                    let state_dir = path.parent().ok_or_else(|| Error::FindingStateDirName {
                        path: path.to_path_buf(),
                    })?;

                    let _ = tokio::fs::create_dir_all(state_dir)
                        .await
                        .map_err(|error| Error::CreatingStateDir {
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
                        .map_err(|source| Error::OpenStateFile {
                            path: path.to_path_buf(),
                            source,
                        })?;

                    String::new()
                } else {
                    tracing::error!(?error, "Cannot handle error");
                    return Err(Error::ReadingState(error));
                }
            }
            Ok(text) => text,
        };

        toml::from_str(&text)
            .map_err(Error::ParsingState)
            .map(|state_inner| State { path, state_inner })
    }

    pub async fn save(&self) -> Result<(), Error> {
        let ser = toml::to_string(&self.state_inner).map_err(Error::SerializingState)?;

        tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .append(false)
            .create(true)
            .open(&self.path)
            .await
            .map_err(|source| Error::OpenStateFile {
                path: self.path.to_path_buf(),
                source,
            })?
            .write_all(ser.as_bytes())
            .await
            .map_err(|source| Error::WritingState {
                path: self.path.to_path_buf(),
                source,
            })
    }
}
