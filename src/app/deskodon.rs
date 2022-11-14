use iced::{widget::{Column, Container, text}, Length, Application, Theme};

use crate::app::message::Message;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Deskodon {
    LoginScreen {
    },

    DefaultView {
    },
}

impl Application for Deskodon {
    type Executor = iced::executor::Default; // tokio
    type Message = Message;
    type Flags = String;
    type Theme = Theme;

    fn new(_name: String) -> (Self, iced::Command<Self::Message>) {
        (Self::LoginScreen {}, iced::Command::perform(crate::config::load(), Message::ConfigLoaded))
    }

    fn title(&self) -> String {
        String::from("deskodon")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        match self {
            Deskodon::LoginScreen { .. } => {
                let text = text("Welcome");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::DefaultView {
                ..
            } => {
                unimplemented!()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        unimplemented!()
    }

}
