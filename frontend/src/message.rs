#[derive(Debug)]
pub enum Message {
    StartLoggingIn,
    LoginSuccess(String),
    LoginFailed(String),
}

