#[derive(Clone, Debug)]
pub enum Message {
    ConfigLoaded(crate::config::Config),
    ConfigLoadingFailed(String),

    InstanceInputChanged(String),
    UsernameInputChanged(String),

    LoginButtonPressed,

    LoggedIn(()),
    LoginFailed(String),
}
