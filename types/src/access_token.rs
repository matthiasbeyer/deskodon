#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccessToken(String);

impl From<String> for AccessToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
