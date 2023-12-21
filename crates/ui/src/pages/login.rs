use crate::routes::Route;
use common::Credentials;
use eyre::{eyre, Result};
use tracing::{error, info};
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

async fn post_login() -> Result<String> {
    let document = web_sys::window()
        .expect("failed to access window")
        .document()
        .expect("failed to get document");

    let username = document
        .get_element_by_id("username")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    let password = document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();

    let login_post_data = Credentials { username, password };
    info!("before response");

    let response = gloo::net::http::Request::post("/api/login")
        .json(&login_post_data)?
        .send()
        .await?;
    match response.status() {
        200 | 202 => Ok(response.text().await?),
        x => {
            error!("authorization error code: {x:?}");
            Err(eyre!(response.text().await?))
        }
    }
}

#[function_component(Login)]
pub(crate) fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let fetch_post_login =
        yew_hooks::use_async(async move { post_login().await.map_err(|e| format!("{e:?}")) });
    let click_handle = fetch_post_login.clone();

    let onclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        click_handle.run();
    });

    html! {
        <div class={classes!("flex", "center", "full")}>
            <form class={classes!("flex","login")}>
                  <input id={"username"} class={classes!("login-input")} type={"email"} placeholder={"email"}/>
                  <input id={"password"} class={classes!("login-input")} type={"password"} placeholder={"password"}/>
                  <input  class={classes!("login-input", "full")} value={"submit"} type={"submit"} onclick={onclick}/>

                    {
                        if fetch_post_login.loading {
                            html! { "Loading" }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if let Some(data) = &fetch_post_login.data {
                            navigator.push(&Route::Home);
                            html! {
                                <p>{format!("{}", data)}</p>
                            }

                        } else {
                            html! {}
                        }
                    }
                    {
                        if let Some(_error) = &fetch_post_login.error {
                            html! {
                                <p>
                                { format!("authentication failed, please check your username and password") }
                                </p>
                            }
                        } else {
                            html! {}
                        }
                    }




            </form>
        </div>
    }
}
