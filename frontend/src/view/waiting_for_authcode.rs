use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::view::button;

pub fn view_waiting_for_authcode(code: &str) -> Node<Message> {
    div![
        textarea![
            attrs! {
                At::Rows => 1,
            },
            code,
            input_ev(Ev::Input, Message::MastodonAuthCodeInput),
        ],
        button("Login", ev(Ev::Click, |_| Message::Authorize)),
    ]
}
