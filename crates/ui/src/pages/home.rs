use super::BACKEND_URL;
use crate::routes::Route;
use common::Credentials;
use eyre::Result;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

pub(crate) async fn fetch_login() -> Result<String> {
    // let cookie_store = {
    //     let file = std::fs::File::open("cookies.json")
    //         .map(std::io::BufReader::new)
    //         .unwrap();
    //     // use re-exported version of `CookieStore` for crate compatibility
    //     reqwest_cookie_store::CookieStore::load_json(file).unwrap()
    // };
    // let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    // let cookie_store = std::sync::Arc::new(cookie_store);
    // {
    //     // Examine initial contents
    //     println!("initial load");
    //     let store = cookie_store.lock().unwrap();
    //     for c in store.iter_any() {
    //         println!("{:?}", c);
    //     }
    // }
    // let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    // Ok(client
    //     .get(format!("{}/protected", BACKEND_URL))
    //     // .header("", value)
    //     .send()
    //     .await?
    //     .text()
    //     .await?)

    let response = gloo::net::http::Request::get("/api/protected")
        .send()
        .await?;
    Ok(response.text().await?)
}

#[function_component(Home)]
pub(crate) fn home() -> Html {
    let state = use_async_with_options(
        async move {
            fetch_login().await.map_err(|e| {
                tracing::error!("authentication failed...{e:?}");
                "unexpected error".to_string()
            })
        },
        UseAsyncOptions::enable_auto(),
    );

    let navigator = use_navigator().unwrap();

    html! {
        <div>
            {
                if state.loading {
                    html! { "Loading" }
                } else {
                    html! {}
                }
            }
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
                if let Some(error) = &state.error {
                    navigator.push(&Route::Login);

                    html! { error }
                } else {
                    html! {}
                }
            }

        </div>
    }
}

// Your main code here

#[cfg(test)]
mod tests {
    use super::*; // Import all the necessary elements from the outer module
    use tokio;
    #[tokio::test]
    async fn test_my_function() -> Result<()> {
        // let result = my_function(); // Replace with your function
        // assert!(result.is_expected()); // Replace with your condition
        // You can add more assertions here
        println!("in test");
        let login = fetch_login().await?;
        println!("fetch_login: {:?}", login);

        Ok(())
    }

    // You can add more test functions here
}
