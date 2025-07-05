use axum::{routing::post, Router};

use crate::{
    handlers::auth::{login, logout, refresh_token},
    models::AppState,
};

pub async fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/refresh", post(refresh_token))
}
