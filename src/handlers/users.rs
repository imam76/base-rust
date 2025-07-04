use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{AppError, models::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    name: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Uuid,
    username: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseUsers {
    users: Vec<User>,
}

pub async fn get_all_users(
    Query(params): Query<Hello>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let record = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email
        FROM users
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    let name = params.name.clone().unwrap_or_else(|| "World".to_string());
    let response_body = json!({
        "message": format!("Hello, {}!", name),
        "name": name,
        "users": record,
    });
    Ok((StatusCode::OK, Json(response_body)).into_response())
}
