#[derive(Debug)]
pub enum Event {
    GuiBooted,

    Login {
        instance: String,
    },

    OpenInBrowser {
        url: String,
    },
}
