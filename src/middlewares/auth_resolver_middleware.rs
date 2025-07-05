use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tower_cookies::{Cookie, Cookies};
use tracing::info;
use uuid::Uuid;

use crate::{models::AuthenticatedUser, utils::constants::AUTH_TOKEN, AppError};

pub async fn start(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    info!("AUTH MIDDLEWARE - {}", req.uri().path());

    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string());

    match auth_token {
        Some(token) => {
            info!("Found auth token: {}", token);
            match parse_token(token) {
                Ok(user_id) => {
                    info!("Successfully parsed user_id: {}", user_id);
                    req.extensions_mut().insert(AuthenticatedUser::new(user_id));
                    Ok(next.run(req).await)
                }
                Err(e) => {
                    info!("Invalid token found, removing cookie");
                    cookies.remove(Cookie::build(AUTH_TOKEN).build());
                    Err(e)
                }
            }
        }
        None => {
            info!("No auth token found");
            Err(AppError::UnAuthorized)
        }
    }
}

fn parse_token(token: String) -> Result<Uuid, AppError> {
    info!("Parsing token: {}", token);

    if token.is_empty() {
        info!("Token is empty");
        return Err(AppError::UnAuthorized);
    }

    // Token format: "user-{uuid}.exp.sign"
    let parts: Vec<&str> = token.split('.').collect();
    info!("Token parts: {:?} (count: {})", parts, parts.len());

    if parts.len() != 3 {
        info!("Expected 3 parts, got {}", parts.len());
        return Err(AppError::UnAuthorized);
    }

    // Extract user ID from the first part: "user-{uuid}"
    let user_part = parts[0];
    info!("User part: {}", user_part);

    if !user_part.starts_with("user-") {
        info!("User part doesn't start with 'user-'");
        return Err(AppError::UnAuthorized);
    }

    let user_id_str = &user_part[5..]; // Remove "user-" prefix
    info!("Extracted user_id_str: {}", user_id_str);

    match Uuid::parse_str(user_id_str) {
        Ok(uuid) => {
            info!("Successfully parsed UUID: {}", uuid);
            Ok(uuid)
        }
        Err(e) => {
            info!("Failed to parse UUID: {}", e);
            Err(AppError::UnAuthorized)
        }
    }
}
