#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    username: Option<String>,
    instance: Option<url::Url>,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            username: None,
            instance: None,
        }
    }
}
