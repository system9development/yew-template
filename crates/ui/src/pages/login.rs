use crate::pages::login;
use crate::routes::Route;

use super::BACKEND_URL;
use common::Credentials;
use eyre::Result;
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

    // let response = client
    //     .post(format!("{}/login", BACKEND_URL))
    //     .header("Content-Type", "application/json")
    //     .body(serde_json::to_string_pretty(&login_post_data)?)
    //     .send()
    //     .await?;
    // let cookies = response.clone().cookies();

    let response = gloo::net::http::Request::post("/api/login")
        .json(&login_post_data)?
        .send()
        .await?;
    Ok(response.text().await?)
}

#[function_component(Login)]
pub(crate) fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let fetch_post_login = yew_hooks::use_async(async move {
        post_login().await.map_err(|e| {
            tracing::error!("authentication failed...{e:?}");
            "unexpected error".to_string()
        })
    });
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
                  <input  class={classes!("login-input")} value={"submit"} type={"submit"} onclick={onclick}/>

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
                        if let Some(error) = &fetch_post_login.error {
                            html! { error }
                        } else {
                            html! {}
                        }
                    }




            </form>
        </div>
    }
}
