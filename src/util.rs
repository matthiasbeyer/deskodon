use miette::IntoDiagnostic;
use miette::Error;
use miette::WrapErr;

#[tracing::instrument]
pub async fn open_url(url: url::Url) -> Result<(), Error> {
    tracing::trace!(?url, "Opening url");
    open::that(url.as_ref())
        .into_diagnostic()
        .with_context(|| format!("Opening URL {}", url))
        .map_err(Error::from)
}
