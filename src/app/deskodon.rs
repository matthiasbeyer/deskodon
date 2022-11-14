use iced::{
    widget::{text, text_input, Column, Container},
    Application, Length, Theme,
};

use miette::Error;

use crate::app::message::Message;
use crate::config::Config;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Deskodon {
    ConfigLoading,

    ConfigLoaded { config: Result<Config, Error> },

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
            iced::Command::perform(crate::config::load(), Message::ConfigLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("deskodon")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        use crate::app::message::Message::*;

        match message {
            ConfigLoaded(result) => {
                *self = Deskodon::ConfigLoaded { config: result };
                iced::Command::none()
            }
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
                if let Ok(config) = config {
                    if let Some((username, instance_url)) = config.get_login() {
                        let text = text(format!("Welcome @{}@{}", username, instance_url));

                        let content = Column::new().spacing(20).push(text);

                        Container::new(content)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x()
                            .center_y()
                            .into()
                    } else {
                        let text = text("Enter credentials");
                        let username = text_input("Username", "", |s: String| -> Message {
                            Message::UsernameInputChanged(s)
                        });

                        let instance = text_input("mastodon.social", "", |s: String| -> Message {
                            Message::InstanceInputChanged(s)
                        });

                        let content = Column::new()
                            .spacing(20)
                            .push(text)
                            .push(username)
                            .push(instance);

                        Container::new(content)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x()
                            .center_y()
                            .into()
                    }
                } else {
                    let text = text("Config loading failed");

                    let content = Column::new().spacing(20).push(text);

                    Container::new(content)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                        .into()
                }
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
