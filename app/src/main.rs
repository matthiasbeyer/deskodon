mod application;
mod configuration;
mod error;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let xdg = xdg::BaseDirectories::with_prefix("deskodon")?;

    let (command_sender, command_receiver) = tokio::sync::mpsc::channel(100);
    let (event_sender, event_receiver) = tokio::sync::mpsc::channel(100);
    let app = crate::application::Application::load_from_xdg(xdg).await?;

    let gui_task = tokio::task::spawn_blocking(|| {
        let gui = deskodon_frontend::Gui::new(event_sender, command_receiver);

        gui.run().map_err(crate::error::Error::Gui)
    });

    let app_task = app.run(event_receiver, command_sender);

    let (_, _) = tokio::join!(gui_task, app_task);
    Ok(())
}
