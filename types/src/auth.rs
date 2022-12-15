#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Auth {
    pub client_id: String,
    pub client_secret: String,
    pub url: url::Url,
}
