use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::view::button;

pub fn view_unauthorized(mastodon_url: &str) -> Node<Message> {
    div![
        textarea![
            attrs! {
                At::Rows => 1,
                At::Placeholder => "https://mastodon.social",
            },
            mastodon_url,
            input_ev(Ev::Input, Message::MastodonUrlInput),
        ],
        button("Authorize", ev(Ev::Click, |_| Message::Register)),
    ]
}
