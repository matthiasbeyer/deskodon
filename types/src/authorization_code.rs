#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AuthorizationCode(String);

impl AsRef<str> for AuthorizationCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
