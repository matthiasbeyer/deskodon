use seed::prelude::*;
use seed::*;

use crate::message::Message;

pub fn view_button(name: &str, handler: EventHandler<Message>) -> Node<Message> {
    button![name, C!["button"], handler]
}
