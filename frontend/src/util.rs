#[derive(Clone, Debug)]
pub struct StatusId(String);

impl From<String> for StatusId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for StatusId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
