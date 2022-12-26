use yew_router::Routable;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/registration")]
    Registration,

    #[not_found]
    #[at("/404")]
    NotFound,
}
