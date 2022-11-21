use std::sync::Arc;

use iced::{widget::{Text, Row, Column}, Element};
use megalodon::entities::status::Status;

use super::message::Message;

#[derive(Clone, Debug)]
pub struct Toot {
    status: Arc<Status>,
    content: scraper::Html,
}

impl From<Status> for Toot {
    fn from(status: Status) -> Self {
        Self {
            content: {
                if let Some(reblog) = status.reblog.as_ref() {
                    scraper::Html::parse_fragment(&reblog.content)
                } else {
                    scraper::Html::parse_fragment(&status.content)
                }
            },
            status: Arc::new(status),
        }
    }
}

impl Toot {
    pub fn view(&self) -> Element<Message> {
        let header = Row::new()
            .spacing(20)
            .push(Text::new(self.status.account.username.to_string()).size(12))
            .push(Text::new(
                self.status
                    .reblog
                    .is_some()
                    .then(|| "Reblog")
                    .unwrap_or("Original"),
            ).size(12))
            .push(Text::new(self.status.created_at.to_string()).size(12));

        let content = {
            let elms = self.content.tree.values().filter_map(|node| {
                match node {
                    scraper::Node::Document => {
                        tracing::trace!("Encountered element type Document");
                        None
                    },
                    scraper::Node::Fragment => {
                        tracing::trace!("Encountered element type Fragment");
                        None
                    }
                    scraper::Node::Doctype(dt) => {
                        tracing::trace!("Encountered element type Doctype {:?}", dt);
                        None
                    },
                    scraper::Node::Comment(cm) => {
                        tracing::trace!("Encountered element type Comment {:?}", cm);
                        None
                    },
                    scraper::Node::Text(t) => {
                        tracing::trace!("Encountered element type Text {:?}", t);
                        Some(Text::new(t.to_string()).size(12))
                    }
                    scraper::Node::Element(el) => {
                        tracing::trace!("Encountered element type Element {:?}", el);
                        None
                    },
                    scraper::Node::ProcessingInstruction(pi) => {
                        tracing::trace!("Encountered element type ProcessingInstruction {:?}", pi);
                        None
                    },
                }
            })
            .map(Element::from)
            .collect();

            iced_native::widget::Column::with_children(elms).spacing(20)
        };

        let replies_count = Text::new(self.status.replies_count.to_string()).size(12);
        let reblogs_count = Text::new(self.status.reblogs_count.to_string()).size(12);

        let footer = Row::new()
            .spacing(20)
            .push(replies_count)
            .push(reblogs_count);

        Column::new()
            .spacing(20)
            .push(header)
            .push(content)
            .push(footer)
            .into()
    }
}

