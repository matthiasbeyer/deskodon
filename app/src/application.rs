use crate::configuration::Configuration;
use crate::error::ApplicationError;
use crate::error::Error;
use crate::state::State;

use deskodon_lib::EventReceiver;
use tokio::sync::Mutex;

pub fn run(
    gui: deskodon_frontend::GuiHandle,
    event_receiver: EventReceiver,
) -> std::thread::JoinHandle<Result<(), crate::error::Error>> {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().map_err(ApplicationError::AsyncRuntimeCreation)?;
        rt.block_on(async move {
            tracing_subscriber::fmt::init();
            let xdg =
                xdg::BaseDirectories::with_prefix("deskodon").map_err(ApplicationError::Xdg)?;

            let app = Application::new(xdg, gui, event_receiver).await?;
            app.run().await
        })
        .map_err(Error::Application)
    })
}

pub struct Application {
    app_state: Mutex<AppState>,
    gui: deskodon_frontend::GuiHandle,
    event_receiver: EventReceiver,
}

impl Application {
    pub async fn new(
        xdg: xdg::BaseDirectories,
        gui: deskodon_frontend::GuiHandle,
        event_receiver: EventReceiver,
    ) -> Result<Self, ApplicationError> {
        let (config, state) = tokio::try_join!(
            crate::configuration::Configuration::load_from_path(
                xdg.get_config_file("config.toml"),
                gui.clone(),
            ),

            crate::state::State::load_from_path(xdg.get_state_file("state.toml"), gui.clone()),
        )?;

        Ok(Application {
            app_state: Mutex::new(AppState { config, state }),
            gui,
            event_receiver,
        })
    }

    pub async fn run(mut self) -> Result<(), ApplicationError> {
        while let Some(event) = self.event_receiver.recv().await {
            tracing::info!(?event, "Received event");
        }

        Ok(())
    }
}

struct AppState {
    #[allow(unused)]
    config: Configuration,
    state: State,
}
