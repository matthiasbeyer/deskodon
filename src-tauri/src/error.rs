#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    Megalodon(#[from] mastodon_async::errors::Error),

    #[error(transparent)]
    XdgBaseDirs(#[from] xdg::BaseDirectoriesError),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
}

impl From<Error> for deskodon_types::error::Error {
    fn from(e: Error) -> Self {
        deskodon_types::error::Error::Str(e.to_string())
    }
}
