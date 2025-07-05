use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::verify;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{
    models::{AppState, User},
    utils::constants::AUTH_TOKEN,
    AppError,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    cookies: Cookies,
    State(state): State<AppState>,
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Response, AppError> {
    info!("-> HANDLER - POST /auth/login");

    let Json(body) = body.map_err(|_| AppError::BadRequest("Invalid JSON".to_string()))?;
    let email = body.email.trim();

    info!("Login attempt for email: {}", email);

    // Fetch user from database
    let user = get_user_by_email(&state, email).await?;

    // Check if user is active
    if !user.is_active {
        info!("Login attempt for inactive user: {}", email);
        return Err(AppError::LoginFailed);
    }

    // Verify password
    let is_valid = verify(&body.password, &user.password_hash)
        .map_err(|e| AppError::UnhandledError(e.to_string()))?;

    if is_valid {
        handle_successful_login(&cookies, &user, email).await
    } else {
        handle_failed_login(&cookies, email).await
    }
}

async fn get_user_by_email(state: &AppState, email: &str) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, first_name, last_name, 
               is_active, is_verified, last_login_at, created_at, created_by, updated_at, updated_by
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        info!("Database error: {}", e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        info!("User not found with email: {}", email);
        AppError::LoginFailed
    })
}

async fn handle_successful_login(
    cookies: &Cookies,
    user: &User,
    email: &str,
) -> Result<Response, AppError> {
    info!("Password is valid for user: {}", email);

    let auth_token = format!("user-{}.exp.sign", user.id);
    cookies.add(Cookie::new(AUTH_TOKEN, auth_token));

    let response_body = json!({
        "success": true,
        "message": "Login successful",
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "is_verified": user.is_verified
        }
    });

    Ok((StatusCode::OK, Json(response_body)).into_response())
}

async fn handle_failed_login(cookies: &Cookies, email: &str) -> Result<Response, AppError> {
    info!("Invalid password for user: {}", email);
    cookies.remove(Cookie::from(AUTH_TOKEN));
    Err(AppError::LoginFailed)
}

pub async fn logout(cookies: Cookies) -> Result<Response, AppError> {
    info!("-> HANDLER - POST /auth/logout");

    cookies.remove(Cookie::from(AUTH_TOKEN));

    let response_body = json!({
        "message": "Logout successful"
    });

    Ok((StatusCode::OK, Json(response_body)).into_response())
}
