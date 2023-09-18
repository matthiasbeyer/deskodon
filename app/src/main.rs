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
    let app_task =
        crate::application::Application::run_with_xdg(xdg, event_receiver, command_sender);

    let gui_task = tokio::task::spawn_blocking(|| {
        let gui = deskodon_frontend::Gui::new(event_sender, command_receiver);

        gui.run().map_err(crate::error::Error::Gui)
    });

    let (_, _) = tokio::join!(gui_task, app_task);
    Ok(())
}
