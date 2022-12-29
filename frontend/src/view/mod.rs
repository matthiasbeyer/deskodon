use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::model::Model;

mod unauthorized;
use self::unauthorized::view_unauthorized as unauthorized;

mod waiting_for_authcode;
use self::waiting_for_authcode::view_waiting_for_authcode as waiting_for_authcode;

mod button;
use self::button::view_button as button;
use self::button::view_button_primary as button_primary;

pub fn view(model: &Model) -> Node<Message> {
    match model {
        Model::Initialized => {
            div!["Hello World"]
        }
        Model::Unauthorized {
            mastodon_url,
            error: _,
        } => unauthorized(mastodon_url),

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
        Model::WaitingForAuthCode { code } => waiting_for_authcode(code),
    }
}
