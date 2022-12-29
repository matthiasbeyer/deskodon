use std::path::PathBuf;
use std::sync::Arc;

use crate::error::Error;

use deskodon_types::authorization_code::AuthorizationCode;
use mastodon_async::mastodon::Mastodon;
use mastodon_async::registration::Registered;
use mastodon_async::Registration;

use tokio::io::AsyncWriteExt;
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

    pub async fn register(&self, instance_url: url::Url) -> Result<String, Error> {
        let registration = Registration::new(instance_url)
            .client_name(USER_AGENT)
            .build()
            .await?;

        let authentication_url = registration.authorize_url()?;

        {
            let mut inner = self.0.write().await;
            *inner = Inner::Registering { registration };
        }

        Ok(authentication_url)
    }

    pub async fn finalize_registration(&self, code: AuthorizationCode) -> Result<(), Error> {
        let mut inner = self.0.write().await;
        if let Inner::Registering { registration } = &*inner {
            let mastodon = match registration.complete(code.as_ref()).await {
                Err(e) => {
                    log::error!("Failed to finalize registration: {:?}", e);
                    return Err(Error::from(e));
                }
                Ok(m) => m,
            };
            *inner = Inner::Mastodon(mastodon);
        }

        Ok(())
    }

    pub async fn save_login(&self) -> Result<(), Error> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("deskodon")?;
        let xdg_profile_file_path = xdg_dirs.place_data_file("account.toml")?;
        log::debug!("Saving login to {}", xdg_profile_file_path.display());

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .create_new(true)
            .append(false)
            .open(xdg_profile_file_path)
            .await?;

        let inner = self.0.read().await;
        if let Inner::Mastodon(mastodon) = &*inner {
            let data_toml = toml::to_string(&mastodon.data)?;
            file.write_all(data_toml.as_bytes()).await?;
            log::debug!("Profile state written");
            file.sync_all().await?;
            log::debug!("Profile state syned to disk");
            Ok(())
        } else {
            log::error!("Cannot save profile state: Not authenticated");
            Err(Error::NotAuthenticated {
                action_desc: "Saving login",
            })
        }
    }
}
