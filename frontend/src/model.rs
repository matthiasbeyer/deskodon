use std::path::PathBuf;

use deskodon_types::authorization_code::AuthorizationCode;
use mastodon_async_entities::status::Status;
use seed::{prelude::Orders, Url};

use crate::message::{ErrorMessage, Message, UnwrapEitherMessage};

pub enum Model {
    Initialized,
    LoggingIn,

    LoadingConfigFailed(String),

    Home {
        errors: Vec<String>,
        current_statuses: Vec<Status>,
    },

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
                *self = Model::Home {
                    errors: vec![],
                    current_statuses: vec![],
                };
                perform_safe_login(orders);
                perform_get_current_statuses(orders);
            }
            Message::BrowserOpenSuccess => {
                // ignored for now
            }
            Message::LoginSafed => {
                log::info!("Login saved");
            }

            Message::CurrentStatuses(statuses) => {
                if let Model::Home {
                    current_statuses, ..
                } = self
                {
                    *current_statuses = statuses;
                }
            }

            Message::Like(status_id) => {
                log::info!("Liking {:?}", status_id);
            }

            Message::Retoot(status_id) => {
                log::info!("Retoot {:?}", status_id);
            }

            Message::Reply(status_id) => {
                log::info!("Reply {:?}", status_id);
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
            Message::Error(ErrorMessage::BrowserOpenFailed(s)) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(s),
                };
            }
            Message::Error(ErrorMessage::LoginSafeFailed(s)) => {
                *self = Model::Unauthorized {
                    mastodon_url: String::new(),
                    error: Some(s),
                };
            }
            Message::Error(ErrorMessage::GettingStatusesFailed(s)) => {
                if let Model::Home { errors, .. } = self {
                    errors.push(s);
                }
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

fn perform_open_browser(orders: &mut impl Orders<Message>, url: url::Url) {
    orders.perform_cmd(async {
        crate::tauri::call_open_browser(url)
            .await
            .map(|()| Message::BrowserOpenSuccess)
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::BrowserOpenFailed)
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

fn perform_safe_login(orders: &mut impl Orders<Message>) {
    orders.perform_cmd(async {
        crate::tauri::call_save_login()
            .await
            .map(|_| Message::LoginSafed)
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::LoginSafeFailed)
            .unwrap_either_message()
    });
}

fn perform_get_current_statuses(orders: &mut impl Orders<Message>) {
    orders.perform_cmd(async {
        crate::tauri::call_get_current_statuses()
            .await
            .map(Message::CurrentStatuses)
            .map_err(|te| te.to_string())
            .map_err(ErrorMessage::GettingStatusesFailed)
            .unwrap_either_message()
    });
}
