use axum::{Router, routing::post};

use crate::{handlers::auth::get_auth, models::AppState};

pub async fn routes() -> Router<AppState> {
    Router::new().route("/auth", post(get_auth))
}
