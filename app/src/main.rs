mod application;
mod configuration;
mod error;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let (event_sender, event_receiver) = tokio::sync::mpsc::channel(100);

    let gui = deskodon_frontend::Gui::new(event_sender);
    let gui_handle = gui.handle();
    let app_task = crate::application::run(gui_handle, event_receiver);
    let gui_res = gui.run().map_err(crate::error::Error::Gui);
    let app_res = app_task.join();

    match (app_res, gui_res) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(error), _) => {
            tracing::error!(?error);
            Ok(())
        }
        (_, Err(error)) => {
            tracing::error!(?error);
            Ok(())
        }
    }
}
