use mastodon_async::entities::status::Status;
use seed::prelude::*;
use seed::*;

use crate::message::Message;

pub fn view_home(_current_statuses: &[Status]) -> Node<Message> {
    div!["Current statuses"]
}
