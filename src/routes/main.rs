use axum::{Router, routing::get};

use crate::{
    handlers::{root::get_all, users::get_all_users},
    routes::auth::auth_route,
};

pub async fn route() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/hello", get(get_all_users))
        .merge(auth_route().await)
}
