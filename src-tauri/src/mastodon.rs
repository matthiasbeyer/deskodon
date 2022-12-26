use std::path::PathBuf;
use std::sync::Arc;

use crate::error::Error;

use deskodon_types::authorization_code::AuthorizationCode;
use mastodon_async::mastodon::Mastodon;
use mastodon_async::registration::Registered;
use mastodon_async::Registration;

use tokio::sync::RwLock;

const USER_AGENT: &str = "deskodon";

pub struct MastodonState(Arc<RwLock<Inner>>);

enum Inner {
    Empty,
    Registering { registration: Registered },

    Mastodon(Mastodon),
}

impl Default for MastodonState {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Inner::Empty)))
    }
}

impl MastodonState {
    pub async fn state_file(&self) -> Result<Option<PathBuf>, Error> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("deskodon")?;

        match xdg_dirs.find_config_file("deskodon.toml") {
            Some(config_file) => Ok(Some(config_file)),
            None => Ok(None),
        }
    }

    pub async fn load_from_file(&self, config_path: PathBuf) -> Result<(), Error> {
        let config_data: mastodon_async::data::Data = {
            let file = tokio::fs::read_to_string(&config_path).await?;
            toml::from_str(file.as_ref())?
        };

        {
            let mut inner = self.0.write().await;
            *inner = Inner::Mastodon(Mastodon::from(config_data));
        }

        Ok(())
    }

    pub async fn register(&self, instance_url: url::Url) -> Result<(), Error> {
        let registration = Registration::new(instance_url)
            .client_name(USER_AGENT)
            .build()
            .await?;

        {
            let mut inner = self.0.write().await;
            *inner = Inner::Registering { registration };
        }

        Ok(())
    }

    pub async fn finalize_registration(&self, code: AuthorizationCode) -> Result<(), Error> {
        let mut inner = self.0.write().await;
        if let Inner::Registering { registration } = &*inner {
            let mastodon = registration.complete(code.as_ref()).await?;
            *inner = Inner::Mastodon(mastodon);
        }

        Ok(())
    }
}
