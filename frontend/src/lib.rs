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
        self.install_open_in_browser_callback();
        self.gui.run().map_err(crate::error::Error::from)
    }

    fn install_login_callbacks(&self) {
        let event_sender = self.event_sender.clone();
        self.gui.on_login(move |instance| {
            tracing::info!(?instance, "login() invoked");
            let _ = event_sender.blocking_send(Event::Login {
                instance: instance.to_string(),
            });
            tracing::debug!("event sent");
        })
    }

    fn install_open_in_browser_callback(&self) {
        tracing::debug!("installing open_in_browser() callback");
        let event_sender = self.event_sender.clone();
        self.gui.on_open_url_in_browser(move |url| {
            tracing::info!(?url, "open_url_in_browser() invoked");
            let _ = event_sender.blocking_send(Event::OpenInBrowser {
                url: url.to_string(),
            });
            tracing::debug!("Event sent");
        })
    }
}

#[derive(Clone)]
pub struct GuiHandle {
    gui: Weak<GuiMain>,
}

impl GuiHandle {
    pub fn show_login_page(&self) {
        tracing::debug!("Showing login page");
        self.gui
            .upgrade_in_event_loop(|gui| gui.invoke_show_login_page())
            .unwrap();
    }

    pub fn show_authorization_page(&self, url: url::Url) {
        tracing::debug!(?url, url_display = %url, "Showing authorization page");
        self.gui
            .upgrade_in_event_loop(move |gui| gui.invoke_show_authorization_page(url.to_string().into()))
            .unwrap();
    }

    pub fn show_loading_page(&self) {
        tracing::debug!("Showing loading page");
        self.gui
            .upgrade_in_event_loop(|gui| gui.invoke_show_loading_page())
            .unwrap();
    }

    pub fn notify_loading_config(&self) {
        // TODO
    }

    pub fn notify_creating_default_config(&self) {
        // TODO
    }

    pub fn notify_logging_in(&self) {
        tracing::debug!("Notification received: Logging in");
        self.gui
            .upgrade_in_event_loop(|gui| gui.invoke_notify_login_in_progress())
            .unwrap();
    }

    pub fn notify_login_succeeded(&self) {
        tracing::debug!("Notification received: Logging in");
        self.gui
            .upgrade_in_event_loop(|gui| gui.invoke_notify_login_succeeded())
            .unwrap();
    }

    pub fn notify_login_failed(&self, reason: String) {
    }
}
