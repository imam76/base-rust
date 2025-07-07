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
    utils::{constants::AUTH_TOKEN, JwtService},
    AppError,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub use_cookie: Option<bool>, // Optional: specify if you want cookie-based auth
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
        handle_successful_login(&cookies, &user, email, body.use_cookie).await
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
    use_cookie: Option<bool>,
) -> Result<Response, AppError> {
    info!("Password is valid for user: {}", email);

    // Generate JWT token
    let jwt_token = JwtService::generate_token(user.id, user.username.clone(), user.email.clone())?;

    if use_cookie.unwrap_or(true) {
        // Default to using cookies
        // Set JWT as HTTP-only cookie for security
        let mut cookie = Cookie::new(AUTH_TOKEN, jwt_token.clone());
        cookie.set_http_only(true);
        cookie.set_secure(false); // Set to true in production with HTTPS
        cookie.set_path("/");

        // Set cookie max age to match JWT expiry (24 hours)
        cookie.set_max_age(tower_cookies::cookie::time::Duration::hours(24));

        cookies.add(cookie);
    }

    let response_body = json!({
        "success": true,
        "message": "Login successful",
        "token": jwt_token, // Also return token in response for API clients
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

pub async fn refresh_token(
    cookies: Cookies,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    info!("-> HANDLER - POST /auth/refresh");

    // Get current token from cookie
    let current_token = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string())
        .ok_or(AppError::UnAuthorized)?;

    // Validate current token
    let claims = JwtService::validate_token(&current_token)?;
    let _user_id = claims.user_id()?; // Validate user_id format

    // Get fresh user data from database
    let user = get_user_by_email(&state, &claims.email).await?;

    // Check if user is still active
    if !user.is_active {
        info!("Refresh attempt for inactive user: {}", claims.email);
        return Err(AppError::UnAuthorized);
    }

    // Generate new JWT token
    let new_jwt_token =
        JwtService::generate_token(user.id, user.username.clone(), user.email.clone())?;

    // Set new JWT as HTTP-only cookie
    let mut cookie = Cookie::new(AUTH_TOKEN, new_jwt_token.clone());
    cookie.set_http_only(true);
    cookie.set_secure(false); // Set to true in production with HTTPS
    cookie.set_path("/");

    // Set cookie max age to match JWT expiry (24 hours)
    cookie.set_max_age(tower_cookies::cookie::time::Duration::hours(24));

    cookies.add(cookie);

    let response_body = json!({
        "success": true,
        "message": "Token refreshed successfully",
        "token": new_jwt_token,
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

pub async fn me(
    headers: axum::http::HeaderMap,
    cookies: Cookies,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    info!("-> HANDLER - GET /auth/me");

    // Try to get token from Authorization header first, then from cookie
    let token = if let Some(auth_header) = headers.get("authorization") {
        let auth_str = auth_header.to_str().map_err(|_| AppError::UnAuthorized)?;

        if auth_str.starts_with("Bearer ") {
            Some(auth_str.strip_prefix("Bearer ").unwrap().to_string())
        } else {
            None
        }
    } else if let Some(cookie) = cookies.get(AUTH_TOKEN) {
        Some(cookie.value().to_string())
    } else {
        None
    };

    let token = token.ok_or(AppError::UnAuthorized)?;

    // Validate JWT token
    let claims = JwtService::validate_token(&token)?;

    // Get user from database to check if still active
    let user_id = claims.user_id()?;
    let user = get_user_by_id(&state, user_id).await?;

    // Check if user is still active
    if !user.is_active {
        info!("User {} is no longer active", user.email);
        return Err(AppError::UnAuthorized);
    }

    // Return user info (without sensitive data)
    let response_body = json!({
        "status": "success",
        "data": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "is_verified": user.is_verified,
            "is_active": user.is_active,
            "created_at": user.created_at,
            "updated_at": user.updated_at
        }
    });

    Ok((StatusCode::OK, Json(response_body)).into_response())
}

// Helper function to get user by ID
async fn get_user_by_id(state: &AppState, user_id: uuid::Uuid) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, first_name, last_name, 
               is_active, is_verified, last_login_at, created_at, created_by, updated_at, updated_by
        FROM users 
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error when fetching user by ID: {}", e);
        AppError::InternalServerError("Database error".to_string())
    })?
    .ok_or(AppError::UnAuthorized)
}
