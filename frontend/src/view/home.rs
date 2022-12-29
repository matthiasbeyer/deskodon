use mastodon_async::entities::prelude::Account;
use mastodon_async::entities::status::Status;
use seed::prelude::*;
use seed::*;

use crate::message::Message;
use crate::util::StatusId;
use crate::view::button;

pub fn view_home(statuses: &[Status]) -> Node<Message> {
    div![view_status_list(statuses)]
}

fn view_status_list(statuses: &[Status]) -> Node<Message> {
    let mut html = div![];
    for status in statuses {
        html.add_child(view_status(status));
    }
    html
}

fn view_status(status: &'_ Status) -> Node<Message> {
    let status_id_1 = StatusId::from(status.id.clone());
    let status_id_2 = status_id_1.clone();
    let status_id_3 = status_id_2.clone();

    div![
        C!["columns"],
        div![C!["column", "is-2"], view_account(&status.account),],
        div![C!["column", "is-8", "content"], p![status.content.clone()],],
        div![
            C!["column", "is-2"],
            button("Like", ev(Ev::Click, |_| Message::Like(status_id_1))),
            button("Retoot", ev(Ev::Click, |_| Message::Retoot(status_id_2))),
            button("Reply", ev(Ev::Click, |_| Message::Reply(status_id_3))),
        ]
    ]
}

fn view_account(account: &Account) -> Node<Message> {
    div![
        p![account.username.clone()],
        div![
            C!["columns"],
            p![C!["column", "is-4"], account.followers_count],
            p![C!["column", "is-4"], account.following_count],
            p![C!["column", "is-4"], account.statuses_count],
        ]
    ]
}
