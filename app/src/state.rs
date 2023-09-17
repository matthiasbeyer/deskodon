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
        tokio::fs::read_to_string(&path)
            .await
            .map_err(Error::ReadingState)
            .and_then(|text| toml::from_str(&text).map_err(Error::ParsingState))
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
