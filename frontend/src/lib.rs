slint::include_modules!();

use deskodon_lib::EventSender;
use deskodon_lib::CommandReceiver;

pub mod error;

pub struct Gui {
    gui: GuiMain,
    event_sender: EventSender,
    command_recv: CommandReceiver,
}

impl Gui {
    pub fn new(event_sender: EventSender, command_recv: CommandReceiver) -> Self {
        Gui {
            event_sender,
            command_recv,
            gui: GuiMain::new().unwrap(),
        }
    }

    pub fn run(&self) -> Result<(), crate::error::Error> {
        self.gui.run().map_err(crate::error::Error::from)
    }
}
