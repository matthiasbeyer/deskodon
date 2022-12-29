use std::path::PathBuf;
use std::sync::Arc;

use crate::error::Error;

use deskodon_types::authorization_code::AuthorizationCode;
use mastodon_async::entities::status::Status;
use mastodon_async::mastodon::Mastodon;
use mastodon_async::page::Page;
use mastodon_async::registration::Registered;
use mastodon_async::Registration;

use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

const USER_AGENT: &str = "deskodon";

pub struct MastodonState(Arc<RwLock<Inner>>);

enum Inner {
    Empty,
    Registering {
        registration: Registered,
    },

    Mastodon {
        mastodon: Mastodon,
        current_page: Page<Status>,
    },
}

impl Default for MastodonState {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Inner::Empty)))
    }
}

const DESKODON_CONFIG_FILE_NAME: &str = "deskodon.toml";

impl MastodonState {
    fn xdg_base_dir() -> Result<xdg::BaseDirectories, Error> {
        xdg::BaseDirectories::with_prefix("deskodon").map_err(Error::from)
    }

    fn find_config_file(base_dirs: &xdg::BaseDirectories) -> Option<PathBuf> {
        base_dirs.find_config_file(DESKODON_CONFIG_FILE_NAME)
    }

    fn create_config_file(base_dirs: &xdg::BaseDirectories) -> Result<PathBuf, Error> {
        if let Some(path) = Self::find_config_file(base_dirs) {
            Ok(path)
        } else {
            base_dirs
                .place_config_file(DESKODON_CONFIG_FILE_NAME)
                .map_err(Error::from)
        }
    }

    pub async fn state_file(&self) -> Result<Option<PathBuf>, Error> {
        let xdg_dirs = Self::xdg_base_dir()?;

        match Self::find_config_file(&xdg_dirs) {
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
            let mastodon = Mastodon::from(config_data);
            let current_page = mastodon.get_home_timeline().await?;
            *inner = Inner::Mastodon {
                mastodon,
                current_page,
            };
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
            let current_page = mastodon.get_home_timeline().await?;
            *inner = Inner::Mastodon {
                mastodon,
                current_page,
            };
        }

        Ok(())
    }

    pub async fn save_login(&self) -> Result<(), Error> {
        let config_file_path =
            Self::xdg_base_dir().and_then(|dir| Self::create_config_file(&dir))?;
        log::debug!("Saving login to {}", config_file_path.display());

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(config_file_path)
            .await?;

        let inner = self.0.read().await;
        if let Inner::Mastodon { mastodon, .. } = &*inner {
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

    pub async fn get_current_statuses(&self) -> Result<Vec<Status>, Error> {
        let inner = self.0.read().await;
        if let Inner::Mastodon { current_page, .. } = &*inner {
            Ok(current_page.initial_items.clone())
        } else {
            Err(Error::NotAuthenticated {
                action_desc: "Getting current statuses",
            })
        }
    }
}
