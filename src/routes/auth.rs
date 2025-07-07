use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::auth::{login, logout, me, refresh_token},
    models::AppState,
};

pub async fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth", post(login))
        .route("/api/auth/me", get(me))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/refresh", post(refresh_token))
}
