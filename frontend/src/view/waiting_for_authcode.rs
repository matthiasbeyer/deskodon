use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::view::button_primary;
use crate::view::input_labeled;

pub fn view_waiting_for_authcode(code: &str) -> Node<Message> {
    div![
        // Hack to display the unauthorized field in the middle of the screen
        attrs! { At::Style => "padding-top: 40%" },
        div![
            C!["box", "is-vcentered"],
            input_labeled(
                "Authcode",
                None,
                code,
                input_ev(Ev::Input, Message::MastodonAuthCodeInput)
            ),
            button_primary("Login", ev(Ev::Click, |_| Message::Authorize)),
        ]
    ]
}
