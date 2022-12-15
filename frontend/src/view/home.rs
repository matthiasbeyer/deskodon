use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub struct Home;

#[derive(Properties, PartialEq)]
pub struct HomeProbs {}

impl Component for Home {
    type Message = ();
    type Properties = HomeProbs;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::debug!("Home::update()");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="container">

                    <div class="columns">
                        <div class="column is-3">
                        </div>
                        <div class="column is-9">
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

