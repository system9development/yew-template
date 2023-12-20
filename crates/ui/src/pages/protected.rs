use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Protected)]
pub(crate) fn protected() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
