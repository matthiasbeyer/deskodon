use yew::Html;
use yew::function_component;
use yew::html;
use yew_router::BrowserRouter;
use yew_router::Switch;

use crate::route::Route;
use crate::view::registration::Registration;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Registration => html! {
            <Registration />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

