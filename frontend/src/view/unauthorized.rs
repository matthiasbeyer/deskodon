use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::view::button_primary;
use crate::view::input_labeled;
use crate::view::notification_error;

pub fn view_unauthorized(mastodon_url: &str, error: Option<&str>) -> Node<Message> {
    let mut html = div![
        // Hack to display the unauthorized field in the middle of the screen
        attrs! { At::Style => "padding-top: 40%" },
        div![
            C!["box", "is-vcentered"],
            input_labeled(
                "Instance Url",
                Some("https://mastodon.social"),
                mastodon_url,
                input_ev(Ev::Input, Message::MastodonUrlInput)
            ),
            button_primary("Authorize", ev(Ev::Click, |_| Message::Register)),
        ],
    ];

    if let Some(err) = error.map(notification_error) {
        html.add_child(err);
    }

    html
}
