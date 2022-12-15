use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub struct Login;

#[derive(Properties, PartialEq)]
pub struct LoginProbs {
    pub input_ref: NodeRef,
    pub onclick_login: Callback<()>,

    pub error: Option<String>,
}

impl Component for Login {
    type Message = ();
    type Properties = LoginProbs;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::debug!("Login::update()");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.props().onclick_login.clone();
        let onclick_login = move |_| {
            cb.emit(());
        };

        html! {
            <div class="columns">
                <div class="column is-half is-offset-one-quarter">
                    <div class="box">
                        <div class="field is-horizontal">
                          <div class="field-label is-normal">
                            <label class="label">{ "Username" }</label>
                          </div>
                          <div class="field-body">
                            <div class="field">
                              <p class="control">
                                <input ref={ctx.props().input_ref.clone()} class="input" type="text" />
                              </p>
                            </div>
                          </div>
                        </div>

                        <div class="field is-horizontal">
                          <div class="field-label"> </div>
                          <div class="field-body">
                            <div class="field">
                              <div class="control">
                                <button class="button is-primary" onclick={onclick_login}>
                                  { "Login" }
                                </button>
                              </div>
                            </div>
                          </div>
                        </div>
                    </div>

                    if let Some(err) = ctx.props().error.as_ref() {
                        <article class="message is-danger">
                          <div class="message-header">
                            <p>{ "Error" }</p>
                            <button class="delete" aria-label="delete"></button>
                          </div>
                          <div class="message-body">
                            { err }
                          </div>
                        </article>
                    }
                </div>
            </div>
        }
    }
}

