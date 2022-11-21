use std::sync::Arc;

use iced::{widget::{Text, Row, Column}, Element};
use megalodon::entities::status::Status;

use super::message::Message;

#[derive(Clone, Debug)]
pub struct Toot {
    status: Arc<Status>,
}

impl From<Status> for Toot {
    fn from(status: Status) -> Self {
        Self {
            status: Arc::new(status),
        }
    }
}

impl Toot {
    pub fn view(&self) -> Element<Message> {
        let header = Row::new()
            .spacing(20)
            .push(Text::new(self.status.account.username.to_string()))
            .push(Text::new(
                self.status
                    .reblog
                    .is_some()
                    .then(|| "Reblog")
                    .unwrap_or("Original"),
            ))
            .push(Text::new(self.status.created_at.to_string()));

        let content = iced_native::widget::Row::new()
            .spacing(20)
            .push(Text::new(self.status.content.to_string()));

        let replies_count = Text::new(self.status.replies_count.to_string());
        let reblogs_count = Text::new(self.status.reblogs_count.to_string());

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

