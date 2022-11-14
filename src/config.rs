use miette::Context;
use miette::Error;
use miette::IntoDiagnostic;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    username: Option<String>,
    instance: Option<url::Url>,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            username: None,
            instance: None,
        }
    }

    /// Returns None if login credentials are not complete
    pub fn get_login(&self) -> Option<(&str, &url::Url)> {
        self.username
            .as_ref()
            .and_then(|name| self.instance.as_ref().map(|url| (name.as_ref(), url)))
    }

    /// Load the configuration if it exists
    pub async fn load_xdg(xdg: &xdg::BaseDirectories) -> Result<Option<Self>, Error> {
        let config_path = xdg
            .place_config_file("config.toml")
            .into_diagnostic()
            .context("Creating configuration file")?;

        if !config_path.exists() {
            return Ok(None);
        }

        let config = tokio::fs::read_to_string(config_path)
            .await
            .into_diagnostic()
            .context("Reading configuration file")?;

        toml::from_str(&config)
            .into_diagnostic()
            .context("Parsing configuration file")
            .map_err(Error::from)
            .map(Some)
    }
}

pub async fn load() -> Result<Config, miette::Error> {
    let xdg = xdg::BaseDirectories::with_prefix("deskodon").into_diagnostic()?;

    let config = Config::load_xdg(&xdg)
        .await?
        .unwrap_or_else(|| Config::empty());

    Ok(config)
}
