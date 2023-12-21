use crate::pages::{home::*, login::*};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Login,
    #[at("/home")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <h1>
                <Home/>
            </h1>
        },
        Route::Login => html! {
            <Login/>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
