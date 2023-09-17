use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error in GUI")]
    Gui(#[source] deskodon_frontend::error::Error),

    #[error("Failed to read configuration")]
    ReadingConfig(#[source] std::io::Error),

    #[error("Failed to parse configuration")]
    ParsingConfig(#[source] toml::de::Error),

    #[error("Failed to serialize config")]
    SerializingConfig(#[source] toml::ser::Error),

    #[error("Opening config file {}", .path.display())]
    OpenConfigFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Writing to config file {}", .path.display())]
    WritingConfig {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read state")]
    ReadingState(#[source] std::io::Error),

    #[error("Failed to parse state")]
    ParsingState(#[source] toml::de::Error),

    #[error("Failed to serialize state")]
    SerializingState(#[source] toml::ser::Error),

    #[error("Opening state file {}", .path.display())]
    OpenStateFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Writing to state file {}", .path.display())]
    WritingState {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to join application tasks")]
    Join(#[source] tokio::task::JoinError),

}
