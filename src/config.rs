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

    pub fn is_fully_initialized(&self) -> bool {
        self.username.is_none() || self.instance.is_none()
    }

    /// Load the configuration if it exists
    pub async fn load_xdg(xdg: &xdg::BaseDirectories) -> Result<Option<Self>, Error> {
        let config_path = xdg.place_config_file("config.toml")
            .into_diagnostic()
            .context("Creating configuration file")?;

        if !config_path.exists() {
            return Ok(None)
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
