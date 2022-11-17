use miette::Context;
use miette::Error;
use miette::IntoDiagnostic;
use tokio::io::AsyncWriteExt;
use tracing::Instrument;
use tracing::info_span;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    username: Option<String>,
    instance: Option<String>,
    auth_token: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    access_token: Option<String>,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            username: None,
            instance: None,
            auth_token: None,
            client_id: None,
            client_secret: None,
            access_token: None,
        }
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_ref().map(AsRef::as_ref)
    }

    pub fn set_username(&mut self, s: String) {
        self.username = Some(s)
    }

    pub fn instance(&self) -> Option<&str> {
        self.instance.as_ref().map(AsRef::as_ref)
    }

    pub fn set_instance(&mut self, s: String) {
        self.instance = Some(s)
    }

    pub fn auth_token(&self) -> Option<&str> {
        self.auth_token.as_ref().map(AsRef::as_ref)
    }

    pub fn set_auth_token(&mut self, s: String) {
        self.auth_token = Some(s)
    }

    pub fn client_id(&self) -> Option<&str> {
        self.client_id.as_ref().map(AsRef::as_ref)
    }

    pub fn set_client_id(&mut self, s: String) {
        self.client_id = Some(s);
    }

    pub fn client_secret(&self) -> Option<&str> {
        self.client_secret.as_ref().map(AsRef::as_ref)
    }

    pub fn set_client_secret(&mut self, s: String) {
        self.client_secret = Some(s);
    }

    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_ref().map(AsRef::as_ref)
    }

    pub fn set_access_token(&mut self, s: String) {
        self.access_token = Some(s);
    }

    /// Load the configuration if it exists
    #[tracing::instrument]
    pub async fn load_xdg(xdg: &xdg::BaseDirectories) -> Result<Option<Self>, Error> {
        let config_path = xdg
            .place_config_file("config.toml")
            .into_diagnostic()
            .context("Creating configuration file")?;

        if !config_path.exists() {
            return Ok(None);
        }

        let config = tokio::fs::read_to_string(config_path)
            .instrument(info_span!("Reading config file"))
            .await
            .into_diagnostic()
            .context("Reading configuration file")?;

        toml::from_str(&config)
            .into_diagnostic()
            .context("Parsing configuration file")
            .map_err(Error::from)
            .map(Some)
    }

    #[tracing::instrument]
    async fn save(self, xdg: &xdg::BaseDirectories) -> Result<(), Error> {
        let config_path = xdg
            .place_config_file("config.toml")
            .into_diagnostic()
            .context("Creating configuration file")?;

        let str = toml::to_string_pretty(&self)
            .into_diagnostic()
            .context("Generating configuration file")?;

        tracing::trace!("Serialized");
        let mut config_file = tokio::fs::File::open(config_path)
            .instrument(info_span!("Opening file"))
            .await
            .into_diagnostic()
            .context("Opening configuration file for writing")?;

        tracing::trace!("File opened");
        config_file
            .write_all(str.as_bytes())
            .instrument(info_span!("Writing config file"))
            .await
            .into_diagnostic()
            .context("Writing configuration file")?;

        tracing::trace!("File written");
        config_file
            .sync_all()
            .instrument(info_span!("Flushing config file"))
            .await
            .into_diagnostic()
            .context("Flushing configuration file")
            .map_err(Error::from)
    }
}

#[tracing::instrument]
pub async fn save(config: Config) -> Result<(), miette::Error> {
    let xdg = xdg::BaseDirectories::with_prefix("deskodon").into_diagnostic()?;
    config
        .save(&xdg)
        .instrument(info_span!("Saving configuration"))
        .await
        .map_err(miette::Error::from)
}

#[tracing::instrument]
pub async fn load() -> Result<Config, miette::Error> {
    let xdg = xdg::BaseDirectories::with_prefix("deskodon").into_diagnostic()?;

    let config = Config::load_xdg(&xdg)
        .instrument(info_span!("Loading configuration"))
        .await?
        .unwrap_or_else(|| Config::empty());

    Ok(config)
}
