use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::model::Model;

mod unauthorized;
mod waiting_for_authcode;

pub fn view(model: &Model) -> Node<Message> {
    match model {
        Model::Initialized => {
            div!["Hello World"]
        }
        Model::Unauthorized {
            mastodon_url,
            error: _,
        } => self::unauthorized::view_unauthorized(mastodon_url),

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
            self::waiting_for_authcode::view_waiting_for_authcode(code)
        }
    }
}
