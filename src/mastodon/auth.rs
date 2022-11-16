use futures::FutureExt;
use megalodon::Megalodon;
use tracing::{Instrument, info_span};

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
        .instrument(info_span!("app registration"))
        .await;
    tracing::trace!(?reg, "App registration done");
    reg
}

