#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SlintPlatform(#[from] slint::PlatformError),
}
