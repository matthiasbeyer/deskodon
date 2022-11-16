use iced::Application;

mod column;
mod deskodon;
mod message;
mod toot;

use self::deskodon::Deskodon;

pub fn boot() -> Result<(), iced::Error> {
    let settings = iced::Settings {
        window: iced::window::Settings {
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            ..iced::window::Settings::default()
        },
        flags: String::from("deskodon"),
        exit_on_close_request: true,
        ..iced::Settings::default()
    };

    Deskodon::run(settings)
}

