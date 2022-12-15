use crate::error::Error;

use deskodon_types::auth::Auth;
use deskodon_types::access_token::AccessToken;

use futures::FutureExt;
use megalodon::Megalodon;
use tracing::{info_span, Instrument};

pub async fn generate_auth(instance: url::Url) -> Result<Auth, Error> {
    let client = megalodon::mastodon::Mastodon::new(instance.to_string(), None, None);

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
            app_data.map_err(Error::from).and_then(|app_data| {
                Ok(Auth {
                    client_id: app_data.client_id,
                    client_secret: app_data.client_secret,
                    url: url::Url::parse(&app_data.url.unwrap()).map_err(Error::from)?,
                })
            })
        })
        .instrument(info_span!("app registration"))
        .await;
    tracing::trace!(?reg, "App registration done");
    reg
}

#[tracing::instrument(skip_all)]
pub async fn fetch_access_token(
    instance: url::Url,
    client_id: String,
    client_secret: String,
    auth_token: String,
) -> Result<AccessToken, String> {
    let client = megalodon::mastodon::Mastodon::new(instance.to_string(), None, None);

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
        .map(|td| AccessToken::from(td.access_token))
}
