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
        log::info!("init()");
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
        log::info!("update(msg: {:?})", msg);
        match msg {
            Message::ConfigFileFound(path) => {
                if let Model::Initialized = self {
                    perform_call_load_mastodon(orders, path);
                    *self = Model::LoggingIn;
                }
            }
            Message::Error(ErrorMessage::ConfigFileNotFound(s)) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(s),
                };
            }
            Message::Register => {
                if let Model::Unauthorized { mastodon_url, .. } = self {
                    let url = match url::Url::parse(mastodon_url) {
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
            }
            Message::RegistrationStarted(url) => {
                perform_open_browser(orders, url);
                *self = Model::WaitingForAuthCode {
                    code: String::new(),
                };
            }
            Message::Authorize => {
                if let Model::WaitingForAuthCode { code } = self {
                    let code = AuthorizationCode::from(code.to_string());
                    perform_call_finalize_registration(orders, code);
                }
            }
            Message::LoggedIn => {
                *self = Model::Home;
            }

            Message::MastodonUrlInput(text) => {
                if let Model::Unauthorized { mastodon_url, .. } = self {
                    *mastodon_url = text;
                }
            }
            Message::MastodonAuthCodeInput(newcode) => {
                if let Model::WaitingForAuthCode { code } = self {
                    *code = newcode;
                }
            }
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
            Message::Error(ErrorMessage::FailedToParseUrl { url, error }) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(format!("{}: {}", error, url)),
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
    orders.perform_cmd(async move {
        crate::tauri::call_register(url.clone())
            .await
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::RegistrationFailed)
            .and_then(|s| {
                let url = url::Url::parse(&s).map_err(|e| ErrorMessage::FailedToParseUrl {
                    url: s,
                    error: e.to_string(),
                })?;
                Ok(Message::RegistrationStarted(url))
            })
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
