slint::include_modules!();

use deskodon_lib::{Event, EventSender};
use slint::Weak;

pub mod error;

pub struct Gui {
    gui: GuiMain,
    event_sender: EventSender,
}

impl Gui {
    pub fn new(event_sender: EventSender) -> Self {
        Gui {
            event_sender,
            gui: GuiMain::new().unwrap(),
        }
    }

    pub fn handle(&self) -> GuiHandle {
        GuiHandle {
            gui: self.gui.as_weak(),
        }
    }

    pub fn run(self) -> Result<(), crate::error::Error> {
        self.install_login_callbacks();
        self.gui.run().map_err(crate::error::Error::from)
    }

    fn install_login_callbacks(&self) {
        let event_sender = self.event_sender.clone();
        self.gui.on_login(move |instance, username| {
            tracing::info!(?instance, ?username, "login() invoked");
            let _ = event_sender.blocking_send(Event::Login {
                instance: instance.to_string(),
                username: username.to_string(),
            });
        })
    }
}

#[derive(Clone)]
pub struct GuiHandle {
    gui: Weak<GuiMain>,
}

impl GuiHandle {
    pub fn notify_loading_config(&self) {
        // TODO
    }

    pub fn notify_creating_default_config(&self) {
        // TODO
    }
}
