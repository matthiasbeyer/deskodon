use std::sync::Arc;

use miette::IntoDiagnostic;
use tokio::sync::Mutex;

mod config;
mod ui;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), miette::Error> {
    let xdg = xdg::BaseDirectories::with_prefix("deskodon")
        .into_diagnostic()?;

    let config = Config::load_xdg(&xdg)
        .await?
        .unwrap_or_else(|| Config::empty());

    let config = Arc::new(Mutex::new(config));

    crate::ui::boot(config)
}
