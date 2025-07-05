use axum::{body::Body, extract::Request, http::HeaderMap, middleware::Next, response::Response};
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{
    models::AuthenticatedUser,
    utils::{constants::AUTH_TOKEN, JwtService},
    AppError,
};

pub async fn start(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    info!("AUTH MIDDLEWARE - {}", req.uri().path());

    // Try to get token from cookie first, then from Authorization header
    let token = get_token_from_request(&cookies, req.headers());

    match token {
        Some(jwt_token) => {
            info!("Found JWT token");
            match JwtService::validate_token(&jwt_token) {
                Ok(claims) => {
                    info!("Successfully validated JWT for user: {}", claims.username);
                    let user_id = claims.user_id()?;
                    req.extensions_mut().insert(AuthenticatedUser::new(user_id));
                    Ok(next.run(req).await)
                }
                Err(e) => {
                    info!("Invalid JWT token, removing cookie");
                    cookies.remove(Cookie::build(AUTH_TOKEN).build());
                    Err(e)
                }
            }
        }
        None => {
            info!("No JWT token found");
            Err(AppError::UnAuthorized)
        }
    }
}

fn get_token_from_request(cookies: &Cookies, headers: &HeaderMap) -> Option<String> {
    // First try to get token from cookie
    if let Some(cookie_token) = cookies.get(AUTH_TOKEN) {
        return Some(cookie_token.value().to_string());
    }

    // Then try to get token from Authorization header (Bearer token)
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string()); // Remove "Bearer " prefix
            }
        }
    }

    None
}
