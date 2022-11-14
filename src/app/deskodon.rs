use iced::{
    widget::{text, text_input, Column, Container, button, Button},
    Application, Length, Theme,
};

use crate::app::message::Message;
use crate::config::Config;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Deskodon {
    ConfigLoading,

    ConfigLoaded { config: Config },

    ConfigLoadingFailed { err: String },

    LoginScreen {},

    DefaultView {},
}

impl Application for Deskodon {
    type Executor = iced::executor::Default; // tokio
    type Message = Message;
    type Flags = String;
    type Theme = Theme;

    fn new(_name: String) -> (Self, iced::Command<Self::Message>) {
        (
            Self::LoginScreen {},
            iced::Command::perform(crate::config::load(), |result| match result {
                Ok(config) => Message::ConfigLoaded(config),
                Err(e) => Message::ConfigLoadingFailed(e.to_string())
            }),
        )
    }

    fn title(&self) -> String {
        String::from("deskodon")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        use crate::app::message::Message::*;

        match message {
            ConfigLoaded(config) => {
                *self = Deskodon::ConfigLoaded { config };
                iced::Command::none()
            },
            ConfigLoadingFailed(err) => {
                *self = Deskodon::ConfigLoadingFailed { err };
                iced::Command::none()
            },
            InstanceInputChanged(s) => {
                if let Deskodon::ConfigLoaded { config } = self {
                    config.set_instance(s);
                }
                iced::Command::none()
            },
            UsernameInputChanged(s) => {
                if let Deskodon::ConfigLoaded { config } = self {
                    config.set_username(s);
                }
                iced::Command::none()
            },
            LoginButtonPressed => {
                if let Deskodon::ConfigLoaded { config } = self {
                    let username = config.username().unwrap();
                    let instance = config.instance().unwrap();

                    iced::Command::perform(crate::mastodon::login(username.to_string(), instance.to_string()), |result| match result {
                        Ok(logged_in) => Message::LoggedIn(logged_in),
                        Err(e) => Message::LoginFailed(e.to_string()),
                    })
                } else {
                    iced::Command::none()
                }
            },
            LoggedIn(_) => iced::Command::none(),
            LoginFailed(_) => iced::Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        match self {
            Deskodon::ConfigLoading => {
                let text = text("Welcome");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::ConfigLoaded { config } => {
                let text = text("Enter credentials");
                let username = config.username().unwrap_or_default();
                let instance = config.instance().unwrap_or_default();

                let username = text_input("Username", &username, |s: String| -> Message {
                    Message::UsernameInputChanged(s)
                });

                let instance = text_input("mastodon.social", &instance, |s: String| -> Message {
                    Message::InstanceInputChanged(s)
                });

                let login = Button::new("Login").on_press(Message::LoginButtonPressed);

                let content = Column::new()
                    .spacing(20)
                    .push(text)
                    .push(username)
                    .push(instance)
                    .push(login);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::ConfigLoadingFailed { err } => {
                let text = text("Failed to load configuration");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::LoginScreen { .. } => {
                let text = text("Login screen");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::DefaultView { .. } => {
                let text = text("Default view");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}
