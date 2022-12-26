use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};

use deskodon_types::auth::Auth;

use crate::{message::Message, view::login::Login};

#[derive(Debug)]
pub enum App {
    Login {
        input_ref: NodeRef,
        err: Option<String>,
    },
    Auth {
        input_ref: NodeRef,
        auth: Auth,
    },
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::Login {
            input_ref: NodeRef::default(),
            err: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!("App::update(): {self:?}, msg => {msg:?}");

        match msg {
            Message::Authenticate => match self {
                App::Login { input_ref, .. } => {
                    log::debug!("Login view, logging into instance...");
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        let val = input.value();
                        log::info!("Logging into: {}", val);

                        ctx.link().send_future(async move {
                            log::debug!("Calling generate_auth({val})");
                            let url = match url::Url::parse(&val).map_err(|e| e.to_string()) {
                                Ok(url) => url,
                                Err(e) => return Message::InstanceUrlInvalid(val, e),
                            };

                            match crate::tauri::call_generate_auth(url).await {
                                Ok(auth) => {
                                    log::error!("generate_auth() success");
                                    Message::AuthSuccess(auth)
                                }
                                Err(err) => {
                                    log::error!(
                                        "Error deserializing reply from generate_auth(): '{}'",
                                        err
                                    );
                                    Message::AuthErr(err.to_string())
                                }
                            }
                        });
                    } else {
                        log::error!("Cannot cast {input_ref:?} to HtmlInputElement");
                    }
                    true
                }
                _ => {
                    log::error!("Not in Login view");
                    true
                }
            },

            Message::InstanceUrlInvalid(_url, error) => {
                let input_ref = NodeRef::default();
                *self = App::Login {
                    input_ref,
                    err: Some(error),
                };
                true
            }

            Message::AuthSuccess(auth) => {
                log::info!("Auth: Success!");
                *self = App::Auth {
                    input_ref: NodeRef::default(),
                    auth,
                };
                true
            }

            Message::AuthErr(err) => {
                log::info!("Auth: failed: {err}");
                *self = App::Login {
                    input_ref: NodeRef::default(),
                    err: Some(err),
                };
                true
            }

            Message::Login => {
                log::info!("Login()");
                true
            }

            Message::OpenBrowser(url) => {
                log::info!("Opening browser at {}", url);
                ctx.link().send_future(async move {
                    crate::tauri::call_open_browser(url).await
                })
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("view()");

        match self {
            App::Login { input_ref, err } => {
                log::info!("view(): Login");

                let onclick_login = ctx.link().callback(move |_| {
                    web_sys::console::log_1(&"Authenticate clicked".into());
                    Message::Authenticate
                });

                html! {
                    <Login {input_ref} {onclick_login} error={err.clone()}/>
                }
            }

            App::Auth { input_ref, auth } => {
                let onclick_login = ctx.link().callback(move |_| {
                    web_sys::console::log_1(&"Login clicked".into());
                    Message::Login
                });

                let open_browser = ctx.link().callback(move |url: url::Url| {
                    web_sys::console::log_1(format!("Open browser: {}" url).into());
                    Message::OpenBrowser(url)
                });

                html! {
                    <crate::view::auth::Auth {input_ref} {onclick_login} auth={auth.clone()}/>
                }
            }
        }
    }
}
