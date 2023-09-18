slint::include_modules!();

use deskodon_lib::EventSender;
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
        self.gui.on_login(|instance, username| {
            tracing::info!(?instance, ?username, "login() invoked");
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
