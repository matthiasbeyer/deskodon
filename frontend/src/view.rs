use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::model::Model;

pub fn view(model: &Model) -> Node<Message> {
    match model {
        Model::Initialized => {
            div!["Hello World"]
        }
        Model::Unauthorized {
            mastodon_url,
            error: _,
        } => {
            div![
                textarea![
                    attrs! {
                        At::Rows => 1,
                        At::Placeholder => "https://mastodon.social",
                    },
                    mastodon_url,
                    input_ev(Ev::Input, Message::MastodonUrlInput),
                ],
                button!["Authorize", ev(Ev::Click, |_| Message::Register),],
            ]
        }

        Model::LoggingIn => {
            div!["Logging in"]
        }
        Model::LoadingConfigFailed(errtext) => {
            div![
                "loading config failed",
                p! {
                    errtext
                },
            ]
        }
        Model::Home => {
            div!["Home"]
        }
        Model::WaitingForAuthCode { code } => {
            div![
                textarea![
                    attrs! {
                        At::Rows => 1,
                    },
                    code,
                    input_ev(Ev::Input, Message::MastodonAuthCodeInput),
                ],
                button!["Login", ev(Ev::Click, |_| Message::Authorize),],
            ]
        }
    }
}
