use axum::{Router, routing::get};

use crate::{
    handlers::{root::get_all, users::get_all_users},
    models::AppState,
};

pub async fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all))
        .route("/hello", get(get_all_users))
}
