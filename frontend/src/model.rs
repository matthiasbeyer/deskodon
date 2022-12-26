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

pub fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::ConfigFileFound(path) => match model {
            Model::Initialized => {
                orders.perform_cmd(async {
                    crate::tauri::call_load_mastodon(path)
                        .await
                        .map(|_| Message::LoggedIn)
                        .map_err(|te| te.to_string())
                        .map_err(ErrorMessage::LoadingFailed)
                        .unwrap_either_message()
                });

                *model = Model::LoggingIn;
            }
            _ => {
                // TODO
            }
        },
        Message::Error(ErrorMessage::ConfigFileNotFound(s)) => {
            *model = Model::Unauthorized {
                mastodon_url: String::new(),
                error: Some(s),
            };
        }
        Message::Register => {
            match model {
                Model::Unauthorized { mastodon_url, .. } => {
                    let url = match url::Url::parse(&mastodon_url) {
                        Ok(url) => url,
                        Err(e) => {
                            *model = Model::Unauthorized {
                                mastodon_url: mastodon_url.to_string(),
                                error: Some(e.to_string()),
                            };

                            return // early
                        }
                    };
                    orders.perform_cmd(async {
                        crate::tauri::call_register(url)
                            .await
                            .map(|_| Message::RegistrationStarted)
                            .map_err(|te| te.to_string())
                            .map_err(ErrorMessage::RegistrationFailed)
                            .unwrap_either_message()
                    });
                }
                _ => {
                }
            }
        }
        Message::RegistrationStarted => {
            *model = Model::WaitingForAuthCode {
                code: String::new(),
            };
        }
        Message::Authorize => {
            match model {
                Model::WaitingForAuthCode { code } => {
                    let code = code.to_string();
                    orders.perform_cmd(async {
                        crate::tauri::call_finalize_registration(AuthorizationCode::from(code))
                            .await
                            .map(|_| Message::LoggedIn)
                            .map_err(|te| te.to_string())
                            .map_err(ErrorMessage::LoginFailed)
                            .unwrap_either_message()
                    });

                }
                _ => {
                }
            }
        }
        Message::LoggedIn => {
            *model = Model::Home;
        }

        Message::MastodonUrlInput(text) => {
            match model {
                Model::Unauthorized { mastodon_url, .. } => {
                    *mastodon_url = text;
                }
                _ => {
                }
            }
        }
        Message::MastodonAuthCodeInput(newcode) => {
            match model {
                Model::WaitingForAuthCode { code } => *code = newcode,
                _ => {
                }
            }
        }
        Message::Error(ErrorMessage::LoadingFailed(s)) => {
            *model = Model::LoadingConfigFailed(s);
        }
        Message::Error(ErrorMessage::RegistrationFailed(s)) => {
            *model = Model::Unauthorized {
                mastodon_url: String::new(),
                error: Some(s)
            };
        }
        Message::Error(ErrorMessage::LoginFailed(s)) => {
            *model = Model::Unauthorized {
                mastodon_url: String::new(),
                error: Some(s)
            };
        }
    }
}
