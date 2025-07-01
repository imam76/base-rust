use axum::{Router, routing::get};

use crate::{handlers::auth::get_auth, models::AppState};

pub async fn routes() -> Router<AppState> {
    Router::new().route("/", get(get_auth))
}
