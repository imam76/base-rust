use axum::{routing::post, Router};

use crate::{handlers::auth::get_auth, models::AppState};

pub async fn routes() -> Router<AppState> {
    Router::new().route("/api/auth", post(get_auth))
}
