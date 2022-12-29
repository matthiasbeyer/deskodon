use seed::prelude::*;
use seed::*;

use crate::message::Message;

pub fn view_notification_error(err: &str) -> Node<Message> {
    div![C!["notification", "is-danger"], err]
}
