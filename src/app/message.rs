#[derive(Clone, Debug)]
pub enum Message {
    ConfigLoaded(Result<crate::config::Config, miette::Error>),

    InstanceInputChanged(String),
    UsernameInputChanged(String),
}
