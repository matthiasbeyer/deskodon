use megalodon::Megalodon;
use tracing::{Instrument, info_span};

#[derive(Clone, Debug)]
pub struct AccessToken(String);

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for AccessToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}


#[tracing::instrument(skip_all)]
pub async fn fetch_access_token(
    instance: String,
    client_id: String,
    client_secret: String,
    auth_token: String,
) -> Result<AccessToken, String> {
    let client = megalodon::mastodon::Mastodon::new(instance, None, None);

    client
        .fetch_access_token(
            client_id,
            client_secret,
            auth_token,
            megalodon::default::NO_REDIRECT.to_string(),
        )
        .instrument(info_span!("Fetching access token"))
        .await
        .map_err(|e| e.to_string())
        .map(|td| AccessToken(td.access_token))
}
