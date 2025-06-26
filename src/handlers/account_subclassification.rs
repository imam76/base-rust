// src/handlers/account_subclassification.rs
use axum::{Extension, extract::Path};
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

// GET All
pub async fn get_all(Extension(_pool): Extension<Pool<Postgres>>) {
    info!("Fetching all account subclassifications");
}

pub async fn get_by_id(Extension(_pool): Extension<Pool<Postgres>>, Path(id): Path<Uuid>) {
    info!("Creating account subclassification: {}", id);
}

// POST
pub async fn create(
    Extension(_pool): Extension<Pool<Postgres>>,
    // Json(mut request): Json<CreateAccountSubclassification>,
) {
    let create = "Example account subclassification creation data".to_string();
    info!("Creating account subclassification: {}", create);
}

// PUT
pub async fn update(
    Extension(_pool): Extension<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    // Json(request): Json<UpdateAccountSubclassification>,
) {
    info!("Updating account subclassification: {}", id);
}

// DELETE
pub async fn delete(Extension(_pool): Extension<Pool<Postgres>>, Path(id): Path<Uuid>) {
    info!("Deleting account subclassification: {}", id);
}
