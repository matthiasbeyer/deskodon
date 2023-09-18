use crate::configuration::Configuration;
use crate::error::Error;
use crate::state::State;

use deskodon_lib::CommandSender;
use deskodon_lib::EventReceiver;
use tokio::sync::Mutex;

pub struct Application {
    app_state: Mutex<AppState>,
    event_receiver: EventReceiver,
    command_sender: CommandSender,
}

impl Application {
    pub async fn run_with_xdg(
        xdg: xdg::BaseDirectories,
        event_receiver: EventReceiver,
        command_sender: CommandSender,
    ) -> Result<(), crate::error::Error> {
        let (config, state) = tokio::try_join!(
            crate::configuration::Configuration::load_from_path(
                xdg.get_config_file("config.toml"),
                &command_sender
            ),
            crate::state::State::load_from_path(xdg.get_state_file("state.toml"), &command_sender),
        )?;

        Application {
            app_state: Mutex::new(AppState { config, state }),
            event_receiver,
            command_sender,
        }
        .run()
        .await
    }

    pub async fn run(mut self) -> Result<(), Error> {
        unimplemented!()
    }
}

struct AppState {
    #[allow(unused)]
    config: Configuration,
    state: State,
}
