use crate::app::toot::Toot;

use iced::Element;
use iced::widget::{text, Column};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::Container,
    widget::Row,
    Length,
};

use super::message::Message;

#[derive(Debug)]
pub struct TootColumn {
    name: String,
    items: Vec<Toot>,
}

impl TootColumn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
        }
    }

    pub fn update(&mut self, items: Vec<Toot>) {
        self.items = items;
    }

    #[tracing::instrument]
    pub fn view(&self) -> iced::Element<Message> {
        tracing::trace!("{} column with {} elements", self.name, self.items.len());

        let col_name = text(self.name.to_string());
        let header = Row::new().spacing(20).push(col_name);

        let content = Column::with_children({
            self.items
                .iter()
                .map(Toot::clone)
                .map(|t| Element::new(t))
                .collect()
        });

        let column = Column::new()
            .spacing(20)
            .push(header)
            .push(content);

        Container::new(column)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}
