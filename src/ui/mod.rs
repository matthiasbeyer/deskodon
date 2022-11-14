use std::sync::Arc;

use iced::Application;
use miette::IntoDiagnostic;
use tokio::sync::Mutex;

use crate::config::Config;

mod app;
mod message;

use crate::ui::app::Deskodon;

pub fn boot(config: Arc<Mutex<Config>>) -> Result<(), miette::Error> {
    let settings = iced::Settings {
        window: iced::window::Settings {
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            ..iced::window::Settings::default()
        },
        flags: String::from("deskodon"),
        exit_on_close_request: true,
        ..iced::Settings::default()
    };

    Deskodon::run(settings).into_diagnostic().map_err(miette::Error::from)
}

