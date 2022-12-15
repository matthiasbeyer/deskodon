#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    Megalodon(#[from] megalodon::error::Error),
}

impl From<Error> for deskodon_types::error::Error {
    fn from(e: Error) -> Self {
        deskodon_types::error::Error::Str(e.to_string())
    }
}
