use std::path::PathBuf;

#[derive(Debug)]
pub enum Message {
    ConfigFileFound(PathBuf),
    LoggedIn,

    MastodonUrlInput(String),
    Register,
    RegistrationStarted,
    MastodonAuthCodeInput(String),
    Authorize,

    Error(ErrorMessage),
}

#[derive(Debug)]
pub enum ErrorMessage {
    ConfigFileNotFound(String),
    LoadingFailed(String),
    RegistrationFailed(String),
    LoginFailed(String),
}

impl From<ErrorMessage> for Message {
    fn from(e: ErrorMessage) -> Message {
        Message::Error(e)
    }
}

pub trait UnwrapEitherMessage {
    fn unwrap_either_message(self) -> Message;
}

impl UnwrapEitherMessage for Result<Message, Message> {
    fn unwrap_either_message(self) -> Message {
        match self {
            Ok(msg) => msg,
            Err(msg) => msg,
        }
    }
}

impl UnwrapEitherMessage for Result<Message, ErrorMessage> {
    fn unwrap_either_message(self) -> Message {
        match self {
            Ok(msg) => msg,
            Err(msg) => Message::from(msg),
        }
    }
}
