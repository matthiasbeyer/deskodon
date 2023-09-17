use crate::configuration::Configuration;
use crate::error::Error;
use crate::state::State;

use deskodon_lib::CommandSender;
use deskodon_lib::EventReceiver;
use tokio::sync::Mutex;

pub struct Application {
    app_state: Mutex<AppState>,
}

impl Application {
    pub async fn load_from_xdg(xdg: xdg::BaseDirectories) -> Result<Self, crate::error::Error> {
        let (config, state) = tokio::try_join!(
            crate::configuration::Configuration::load_from_path(xdg.get_config_file("config.toml")),
            crate::state::State::load_from_path(xdg.get_state_file("state.toml")),
        )?;

        let app_state = Mutex::new(AppState { config, state });
        Ok(Application { app_state })
    }

    pub async fn run(
        &self,
        mut event_receiver: EventReceiver,
        mut command_sender: CommandSender,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

struct AppState {
    #[allow(unused)]
    config: Configuration,
    state: State,
}
