mod application;
mod configuration;
mod error;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_sender, event_receiver) = tokio::sync::mpsc::channel(100);

    let gui = deskodon_frontend::Gui::new(event_sender);
    let gui_handle = gui.handle();
    let gui_res = gui.run().map_err(crate::error::Error::Gui);
    let app_task = crate::application::run(gui_handle, event_receiver);
    let app_res = app_task.join();

    match (app_res, gui_res) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(error), _) => todo!(),
        (_, Err(error)) => todo!(),
    }
}
