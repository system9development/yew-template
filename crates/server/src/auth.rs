// use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::users::AuthSession;
use common::Credentials;
use eyre::Result;
use sqlx::PgPool;

pub async fn router() -> Result<Router<()>> {
    let db = PgPool::connect("postgresql://root:toor@localhost:5432/db").await?;
    Ok(Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
        .route("/signup", post(self::post::signup))
        .layer(CorsLayer::very_permissive())
        .with_state(db))
}

mod post {
    use axum::{extract::State, Json};
    use sqlx::PgPool;

    use super::*;
    use crate::users::User;

    use password_auth::generate_hash;

    pub async fn login(
        mut auth_session: AuthSession,
        Json(creds): Json<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if let Err(e) = auth_session.login(&user).await {
            tracing::error!("auth error: {e:?}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        (StatusCode::ACCEPTED, user.username).into_response()
    }

    pub async fn signup(
        State(pool): State<PgPool>,
        Json(creds): Json<Credentials>,
    ) -> impl IntoResponse {
        match sqlx::query_as("select * from users where username = $1")
            .bind(creds.username.clone())
            .fetch_optional(&pool)
            .await
        {
            Ok(user) => {
                let user: Option<User> = user;
                match user {
                    Some(user) => (
                        StatusCode::BAD_REQUEST,
                        format!("{} already exists", user.username),
                    )
                        .into_response(),
                    None => {
                        tracing::info!("creating new user {}", creds.username.clone());
                        if let Err(e) = sqlx::query(
                            "insert into users (username, password) values ($1, $2) returning id",
                        )
                        .bind(creds.username)
                        .bind(generate_hash(creds.password))
                        .execute(&pool)
                        .await
                        {
                            tracing::error!("failed to create user...{:?}", e);
                            (StatusCode::BAD_REQUEST, "failed to create user").into_response()
                        } else {
                            (StatusCode::OK, "success").into_response()
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("database error while fetching username : {:?}", e);
                (StatusCode::BAD_REQUEST, "unexpected error").into_response()
            }
        }
    }
}

mod get {

    use super::*;

    pub async fn login(auth_session: AuthSession) -> impl IntoResponse {
        if let Some(user) = auth_session.user {
            tracing::info!("logging in {:?}", user.username.clone());
            user.username.into_response()
        } else {
            tracing::error!(
                "failed to login user with session info: {:?}",
                auth_session.clone()
            );
            (StatusCode::UNAUTHORIZED, "authentication failed").into_response()
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout() {
            Ok(_) => Redirect::to("/").into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "failed to logout").into_response(),
        }
    }
}
