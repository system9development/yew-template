mod pages;
mod routes;
use routes::*;


use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_wasm::{WASMLayer, WASMLayerConfig};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    tracing_subscriber::registry()
        .with(WASMLayer::new(WASMLayerConfig::default()))
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "ui=debug".into()),
        ))
        .try_init()
        .expect("failed to initialize tracing subscriber...");
    tracing::info!("Starting app..");
    yew::Renderer::<App>::new().render();
}
