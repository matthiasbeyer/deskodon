mod app;
mod config;
mod mastodon;
mod util;

fn main() -> Result<(), miette::Error> {
    tracing_subscriber::fmt::init();
    if let Err(e) = crate::app::boot() {
        match e {
            iced::Error::GraphicsCreationFailed(e) => {
                eprintln!("{:?}", e);
                miette::bail!(e)
            }
            iced::Error::ExecutorCreationFailed(e) => miette::bail!(e),
            iced::Error::WindowCreationFailed(e) => miette::bail!(e),
        }
    }
    Ok(())
}
