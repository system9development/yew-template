mod users;

use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode},
    response::{IntoResponse},
    BoxError,
};
use axum_login::{
    login_required,
    tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use sqlx::PgPool;
use time::Duration;
use tower::{Layer, ServiceBuilder};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
mod auth;
mod protected;
use crate::users::Backend;
use eyre::Result;








use tower_http::cors::{CorsLayer};

const STATIC_DIR: &str = "dist/";
pub struct App {
    db: PgPool,
}

impl App {
    pub async fn new() -> Result<Self> {
        let db = PgPool::connect("postgresql://root:toor@localhost:5432/db").await?;
        sqlx::migrate!().run(&db).await?;
        Ok(Self { db })
    }

    pub async fn serve(self) -> Result<()> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_same_site(SameSite::Lax)
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        // Auth service.
        //
        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = Backend::new(self.db);
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: BoxError| async move {
                tracing::error!("auth error: {:?}", e);
                (StatusCode::BAD_REQUEST, "auth error")
            }))
            .layer(AuthManagerLayerBuilder::new(backend, session_layer).build())
            .layer(CorsLayer::very_permissive());

        let app = protected::router()
            .route_layer(login_required!(Backend))
            .merge(auth::router().await?)
            .layer(auth_service)
            .layer(CorsLayer::very_permissive());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        // .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
        //     |_| "axum_login=debug,sqlx=warn,tower_http=info".into(),
        // )))
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "sqlx=warn,server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().pretty())
        .try_init()?;

    tracing::info!("Starting....");

    App::new().await?.serve().await
}
