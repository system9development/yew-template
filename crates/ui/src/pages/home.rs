use crate::routes::Route;

use eyre::{eyre, Result};
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

pub(crate) async fn check_login() -> Result<String> {
    let response = gloo::net::http::Request::get("/api/protected")
        .send()
        .await?;
    match response.status() {
        200 | 202 => Ok(response.text().await?),
        x => Err(eyre!("{x:?}")),
    }
}

#[function_component(Home)]
pub(crate) fn home() -> Html {
    let state = use_async_with_options(
        async move {
            check_login().await.map_err(|e| {
                tracing::error!("authentication failed...{e:?}");
                "unexpected error".to_string()
            })
        },
        UseAsyncOptions::enable_auto(),
    );

    let navigator = use_navigator().unwrap();

    html! {
        <div class={classes!("border-2")}>
            // TODO: Loading animation
            // {
            //     if state.loading {
            //         html! {}
            //     } else {
            //         html! {}
            //     }
            // }
            {
                if let Some(data) = &state.data {
                    html! {
                        <p>{format!("username: {}", data)}</p>
                     }
                } else {
                    html! {}
                }
            }
            {
                if let Some(_error) = &state.error {
                    navigator.push(&Route::Login);
                    html! { }
                } else {
                    html! {}
                }
            }

        </div>
    }
}
