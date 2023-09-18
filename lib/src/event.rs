#[derive(Debug)]
pub enum Event {
    GuiBooted,

    Login {
        instance: String,
    }
}
