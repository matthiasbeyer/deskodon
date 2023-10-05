use std::path::PathBuf;

use tokio::io::AsyncWriteExt;

use crate::error::ApplicationError;

pub struct Configuration {
    path: PathBuf,
    config: Config,
}

impl Configuration {
    pub async fn load_from_path(
        path: PathBuf,
        gui: deskodon_frontend::GuiHandle,
    ) -> Result<Self, ApplicationError> {
        if path.exists() {
            tracing::debug!(path = %path.display(), "Configuration path exists");
            gui.notify_loading_config();
        } else {
            tracing::debug!(path = %path.display(), "Configuration path does not exist, creating");
            gui.notify_creating_default_config();
            let _ = tokio::fs::create_dir_all(path.parent().unwrap()) // TODO
                .await
                .map_err(|error| ApplicationError::CreatingConfigDir {
                    error,
                    path: path.to_path_buf(),
                })?;

            let _ = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&path)
                .await
                .map_err(|source| ApplicationError::WritingConfig {
                    path: path.to_path_buf(),
                    source,
                })?;
        }

        tracing::debug!(path = %path.display(), "Reading Configuration");
        tokio::fs::read_to_string(&path)
            .await
            .map_err(ApplicationError::ReadingConfig)
            .and_then(|text| {
                tracing::debug!(?text, "Parsing configuration");
                toml::from_str(&text).map_err(ApplicationError::ParsingConfig)
            })
            .map(|config| {
                tracing::debug!("Configuration instantiated");
                Configuration { path, config }
            })
    }

    pub async fn save(&self) -> Result<(), ApplicationError> {
        tracing::debug!(path = %self.path.display(), "Saving Configuration");
        let config = toml::to_string(&self.config).map_err(ApplicationError::SerializingConfig)?;

        tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|source| ApplicationError::OpenConfigFile {
                path: self.path.to_path_buf(),
                source,
            })?
            .write_all(config.as_bytes())
            .await
            .map_err(|source| ApplicationError::WritingConfig {
                path: self.path.to_path_buf(),
                source,
            })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {}
