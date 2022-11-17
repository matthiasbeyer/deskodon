use iced::{
    widget::{text, text_input, Button, Column, Container},
    Application, Length, Subscription, Theme,
};
use tracing::{info_span, Instrument};

use crate::app::{column::TootColumn, message::Message, toot::Toot};
use crate::config::Config;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Deskodon {
    ConfigLoading,

    ConfigLoaded {
        config: Config,
    },

    ConfigLoadingFailed {
        err: String,
    },

    EnterAuthToken {
        config: Config,
        auth: crate::mastodon::Auth,
    },

    DefaultView {
        mastodon: crate::mastodon::Mastodon,
        column: TootColumn,
    },
}

impl Application for Deskodon {
    type Executor = iced::executor::Default; // tokio
    type Message = Message;
    type Flags = String;
    type Theme = Theme;

    fn new(_name: String) -> (Self, iced::Command<Self::Message>) {
        (
            Self::ConfigLoading {},
            iced::Command::perform(crate::config::load(), |result| match result {
                Ok(config) => Message::ConfigLoaded(config),
                Err(e) => Message::ConfigLoadingFailed(e.to_string()),
            }),
        )
    }

    fn title(&self) -> String {
        String::from("deskodon")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        use crate::app::message::Message::*;

        match message {
            ConfigLoaded(config) => {
                *self = Deskodon::ConfigLoaded { config };
                iced::Command::none()
            }
            ConfigLoadingFailed(err) => {
                *self = Deskodon::ConfigLoadingFailed { err };
                iced::Command::none()
            }
            InstanceInputChanged(s) => {
                if let Deskodon::ConfigLoaded { config } = self {
                    config.set_instance(s);
                }
                iced::Command::none()
            }
            UsernameInputChanged(s) => {
                if let Deskodon::ConfigLoaded { config } = self {
                    config.set_username(s);
                }
                iced::Command::none()
            }
            GenerateAccessTokenButtonPressed => {
                if let Deskodon::ConfigLoaded { config } = self {
                    let instance = config.instance().unwrap();

                    iced::Command::perform(
                        crate::mastodon::generate_auth(instance.to_string()),
                        |result| match result {
                            Ok(auth) => Message::GeneratedAuth(auth),
                            Err(e) => Message::GeneratedAuthFailed(e),
                        },
                    )
                } else {
                    iced::Command::none()
                }
            }
            GeneratedAuth(auth) => {
                if let Deskodon::ConfigLoaded { config } = self {
                    config.set_client_id(auth.client_id.to_string());
                    config.set_client_secret(auth.client_secret.to_string());

                    tracing::trace!(?config, "Dispatch save-config job");
                    let config_clone = config.clone();
                    let save_config =
                        iced::Command::perform(crate::config::save(config_clone), |result| {
                            tracing::trace!(?result, "Saving configuration");
                            match result {
                                Ok(_) => Message::SavingConfigSucceeded,
                                Err(e) => Message::SavingConfigFailed(e.to_string()),
                            }
                        });

                    let open_browser =
                        iced::Command::perform(crate::util::open_url(auth.url.clone()), |result| {
                            tracing::trace!(?result, "Opening url");
                            match result {
                                Ok(_) => Message::UrlOpened,
                                Err(e) => Message::UrlOpenFailed(e.to_string()),
                            }
                        });

                    *self = Deskodon::EnterAuthToken {
                        config: config.clone(),
                        auth,
                    };
                    iced::Command::batch(vec![save_config, open_browser])
                } else {
                    iced::Command::none()
                }
            }
            SavingConfigSucceeded => iced::Command::none(),
            SavingConfigFailed(_) => iced::Command::none(),
            GeneratedAuthFailed(_) => iced::Command::none(),
            AccessTokenInputChanged(s) => {
                if let Deskodon::EnterAuthToken { config, .. } = self {
                    config.set_auth_token(s);
                }

                iced::Command::none()
            }
            LoginButtonPressed(auth, auth_token) => {
                if let Deskodon::EnterAuthToken { config, .. } = self {
                    config.set_auth_token(auth_token.clone());
                    tracing::trace!(?config, "Dispatch save-config job");
                    let config_clone = config.clone();
                    let save_config =
                        iced::Command::perform(crate::config::save(config_clone), |result| {
                            tracing::trace!(?result, "Saving configuration");
                            match result {
                                Ok(_) => Message::SavingConfigSucceeded,
                                Err(e) => Message::SavingConfigFailed(e.to_string()),
                            }
                        });

                    // TODO: No unwrap
                    let instance = config.instance().unwrap().to_string();
                    let client_id = config.client_id().unwrap().to_string();
                    let client_secret = config.client_secret().unwrap().to_string();

                    let fetch_access_token = iced::Command::perform(
                        crate::mastodon::fetch_access_token(
                            instance,
                            client_id,
                            client_secret,
                            auth_token,
                        ),
                        |result| match result {
                            Ok(token) => Message::AccessTokenFetched(auth, token),
                            Err(e) => Message::AccessTokenFetchFailed(e.to_string()),
                        },
                    );

                    iced::Command::batch(vec![save_config, fetch_access_token])
                } else {
                    iced::Command::none()
                }
            }
            UrlOpened => iced::Command::none(),
            UrlOpenFailed(_) => iced::Command::none(),
            AccessTokenFetched(auth, token) => {
                if let Deskodon::EnterAuthToken { config, .. } = self {
                    config.set_auth_token(token.as_ref().to_string());
                    tracing::trace!(?config, "Dispatch save-config job");
                    let config_clone = config.clone();
                    let save_config =
                        iced::Command::perform(crate::config::save(config_clone), |result| {
                            tracing::trace!(?result, "Saving configuration");
                            match result {
                                Ok(_) => Message::SavingConfigSucceeded,
                                Err(e) => Message::SavingConfigFailed(e.to_string()),
                            }
                        });

                    *self = Deskodon::DefaultView {
                        mastodon: crate::mastodon::Mastodon::new(auth.url, token),
                        column: TootColumn::new("Default".to_string()),
                    };

                    save_config
                } else {
                    iced::Command::none()
                }
            }
            AccessTokenFetchFailed(_) => iced::Command::none(),
            TimelineStatuses(statuses) => {
                if let Deskodon::DefaultView { column, .. } = self {
                    column.update(statuses.into_iter().map(Toot::from).collect());
                }
                iced::Command::none()
            }
            GetTimelineFailed(_) => iced::Command::none(),
            LoggedIn => iced::Command::none(),
            LoginFailed(_) => iced::Command::none(),
            None => iced::Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        match self {
            Deskodon::ConfigLoading => {
                let text = text("Welcome");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::ConfigLoaded { config } => {
                let text = text("Enter credentials");
                let instance = config.instance().unwrap_or_default();

                let instance = text_input("mastodon.social", &instance, |s: String| -> Message {
                    Message::InstanceInputChanged(s)
                });

                let login =
                    Button::new("Login").on_press(Message::GenerateAccessTokenButtonPressed);

                let content = Column::new()
                    .spacing(20)
                    .push(text)
                    .push(instance)
                    .push(login);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::ConfigLoadingFailed { err } => {
                let text = text("Failed to load configuration");

                let content = Column::new().spacing(20).push(text);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::EnterAuthToken { config, auth } => {
                let heading = text("Go to URL and generate Token, enter here");

                // unfortunately we need this, as xdg-open in KDE downloads the website as html
                // file and opens that instead of opening the url.
                // Maybe misconfig on my side...
                let url = text_input(
                    auth.url.as_ref(),
                    auth.url.as_ref(),
                    |_: String| -> Message { Message::None },
                )
                .size(12)
                .width(iced::Length::Fill);
                let token = config.auth_token().unwrap_or_default();

                let token_input = text_input("", &token, |s: String| -> Message {
                    Message::AccessTokenInputChanged(s)
                });

                let login = Button::new("Login")
                    .on_press(Message::LoginButtonPressed(auth.clone(), token.to_string()));

                let content = Column::new()
                    .spacing(20)
                    .push(heading)
                    .push(url)
                    .push(token_input)
                    .push(login);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            Deskodon::DefaultView { column, .. } => {
                column.view()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use futures::FutureExt;

        if let Deskodon::DefaultView { mastodon, .. } = self {
            let m = mastodon.clone();

            iced_native::subscription::run(0 /* TODO */, futures::stream::once(async move {
                m.get_home_timeline().map(|res| match res {
                    Ok(status) => Message::TimelineStatuses(status),
                    Err(e) => Message::GetTimelineFailed(e),
                })
                .instrument(info_span!("Getting Home timeline"))
                .await
            }))
        } else {
            Subscription::none()
        }
    }
}
