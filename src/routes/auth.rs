use axum::{routing::post, Router};

use crate::{
    handlers::auth::{login, logout},
    models::AppState,
};

pub async fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth", post(login))
        .route("/api/auth/logout", post(logout))
}
