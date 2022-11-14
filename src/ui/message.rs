#[derive(Debug)]
pub enum Message {
    ConfigLoaded(Result<crate::config::Config, miette::Error>),
}
