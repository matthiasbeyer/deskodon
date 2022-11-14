use futures::FutureExt;
use megalodon::Megalodon;
use tracing::{Instrument, trace_span, info_span};

#[derive(Debug, Clone)]
pub struct Auth {
    pub client_id: String,
    pub client_secret: String,
    pub url: url::Url,
}

#[tracing::instrument]
pub async fn generate_auth(instance: String) -> Result<Auth, String> {
    let client = megalodon::mastodon::Mastodon::new(instance, None, None);

    let options = megalodon::megalodon::AppInputOptions {
        redirect_uris: None,
        scopes: Some(
            [
                "read".to_string(),
                "write".to_string(),
                "follow".to_string(),
            ]
            .to_vec(),
        ),
        website: None,
    };

    tracing::trace!("Starting app registration");
    let reg = client
        .register_app("deskodon".to_string(), &options)
        .map(|app_data| {
            app_data.map_err(|e| e.to_string()).and_then(|app_data| {
                Ok(Auth {
                    client_id: app_data.client_id,
                    client_secret: app_data.client_secret,
                    url: url::Url::parse(&app_data.url.unwrap()).map_err(|e| e.to_string())?,
                })
            })
        })
        .instrument(trace_span!("app registration"))
        .await;
    tracing::trace!(?reg, "App registration done");
    reg
}

#[derive(Clone, Debug)]
pub struct AccessToken(String);

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
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
