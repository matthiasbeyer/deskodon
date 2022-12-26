use std::path::PathBuf;

use deskodon_types::authorization_code::AuthorizationCode;
use seed::{prelude::Orders, Url};

use crate::message::{ErrorMessage, Message, UnwrapEitherMessage};

pub enum Model {
    Initialized,
    LoggingIn,

    LoadingConfigFailed(String),

    Home,

    Unauthorized {
        mastodon_url: String,
        error: Option<String>,
    },
    WaitingForAuthCode {
        code: String,
    },
}

impl Model {
    pub fn init(_: Url, orders: &mut impl Orders<Message>) -> Model {
        orders.perform_cmd(async {
            crate::tauri::call_configuration_file_path()
                .await
                .map(Message::ConfigFileFound)
                .map_err(|te| te.to_string())
                .map_err(ErrorMessage::ConfigFileNotFound)
                .unwrap_either_message()
        });

        Model::Initialized
    }

    pub fn update(&mut self, msg: Message, orders: &mut impl Orders<Message>) {
        match msg {
            Message::ConfigFileFound(path) => match self {
                Model::Initialized => {
                    perform_call_load_mastodon(orders, path);
                    *self = Model::LoggingIn;
                }
                _ => {
                    // TODO
                }
            },
            Message::Error(ErrorMessage::ConfigFileNotFound(s)) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(s),
                };
            }
            Message::Register => {
                match self {
                    Model::Unauthorized { mastodon_url, .. } => {
                        let url = match url::Url::parse(&mastodon_url) {
                            Ok(url) => url,
                            Err(e) => {
                                *self = Model::Unauthorized {
                                    mastodon_url: mastodon_url.to_string(),
                                    error: Some(e.to_string()),
                                };

                                return; // early
                            }
                        };
                        perform_call_register(orders, url);
                    }
                    _ => {}
                }
            }
            Message::RegistrationStarted => {
                *self = Model::WaitingForAuthCode {
                    code: String::new(),
                };
            }
            Message::Authorize => match self {
                Model::WaitingForAuthCode { code } => {
                    let code = AuthorizationCode::from(code.to_string());
                    perform_call_finalize_registration(orders, code);
                }
                _ => {}
            },
            Message::LoggedIn => {
                *self = Model::Home;
            }

            Message::MastodonUrlInput(text) => match self {
                Model::Unauthorized { mastodon_url, .. } => {
                    *mastodon_url = text;
                }
                _ => {}
            },
            Message::MastodonAuthCodeInput(newcode) => match self {
                Model::WaitingForAuthCode { code } => *code = newcode,
                _ => {}
            },
            Message::Error(ErrorMessage::LoadingFailed(s)) => {
                *self = Model::LoadingConfigFailed(s);
            }
            Message::Error(ErrorMessage::RegistrationFailed(s)) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(s),
                };
            }
            Message::Error(ErrorMessage::LoginFailed(s)) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(s),
                };
            }
        }
    }
}

fn perform_call_load_mastodon(orders: &mut impl Orders<Message>, path: PathBuf) {
    orders.perform_cmd(async {
        crate::tauri::call_load_mastodon(path)
            .await
            .map(|_| Message::LoggedIn)
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::LoadingFailed)
            .unwrap_either_message()
    });
}

fn perform_call_register(orders: &mut impl Orders<Message>, url: url::Url) {
    orders.perform_cmd(async {
        crate::tauri::call_register(url)
            .await
            .map(|_| Message::RegistrationStarted)
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::RegistrationFailed)
            .unwrap_either_message()
    });
}

fn perform_call_finalize_registration(orders: &mut impl Orders<Message>, code: AuthorizationCode) {
    orders.perform_cmd(async {
        crate::tauri::call_finalize_registration(code)
            .await
            .map(|_| Message::LoggedIn)
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::LoginFailed)
            .unwrap_either_message()
    });
}
