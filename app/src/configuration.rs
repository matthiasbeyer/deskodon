use std::path::PathBuf;

use tokio::io::AsyncWriteExt;

use crate::error::Error;

pub struct Configuration {
    path: PathBuf,
    config: Config,
}

impl Configuration {
    pub async fn load_from_path(path: PathBuf) -> Result<Self, Error> {
        tokio::fs::read_to_string(&path)
            .await
            .map_err(Error::ReadingConfig)
            .and_then(|text| toml::from_str(&text).map_err(Error::ParsingConfig))
            .map(|config| Configuration { path, config })
    }

    pub async fn save(&self) -> Result<(), Error> {
        let config = toml::to_string(&self.config).map_err(Error::SerializingConfig)?;

        tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|source| Error::OpenConfigFile {
                path: self.path.to_path_buf(),
                source,
            })?
            .write_all(config.as_bytes())
            .await
            .map_err(|source| Error::WritingConfig {
                path: self.path.to_path_buf(),
                source,
            })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
}
