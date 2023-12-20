use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new()
        .route("/protected", get(self::get::protected))
        .layer(CorsLayer::very_permissive())
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => user.username.into_response(),
            None => (StatusCode::INTERNAL_SERVER_ERROR, "auth failed").into_response(),
        }
    }
}
