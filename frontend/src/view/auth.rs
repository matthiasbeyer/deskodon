use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub struct Auth;

#[derive(Properties, PartialEq)]
pub struct AuthProbs {
    pub input_ref: NodeRef,
    pub onclick_login: Callback<()>,

    pub auth: deskodon_types::auth::Auth,
}

impl Component for Auth {
    type Message = ();
    type Properties = AuthProbs;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::debug!("Auth::update()");
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
                            <label class="label">{ "Auth Token" }</label>
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
                </div>
            </div>
        }
    }
}


