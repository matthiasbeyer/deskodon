use seed::prelude::*;
use seed::*;

use crate::message::Message;

pub fn view_input(
    placeholder: Option<&'static str>,
    state: &str,
    handler: EventHandler<Message>,
) -> Node<Message> {
    input![
        C!["input"],
        attrs! {
            At::Rows => 1,
            At::Placeholder => placeholder.unwrap_or(""),
        },
        state,
        handler,
    ]
}

pub fn view_input_labeled(
    label: &'static str,
    placeholder: Option<&'static str>,
    state: &str,
    handler: EventHandler<Message>,
) -> Node<Message> {
    div![
        C!["field"],
        label![C!["label"], label],
        input![
            C!["input"],
            attrs! {At::Rows => 1, At::Placeholder => placeholder.unwrap_or("")},
            state,
            handler
        ]
    ]
}
