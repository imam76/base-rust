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
        Some(token) => match parse_token(token) {
            Ok(user_id) => {
                req.extensions_mut().insert(AuthenticatedUser::new(user_id));
                Ok(next.run(req).await)
            }
            Err(e) => {
                info!("Invalid token found, removing cookie");
                cookies.remove(Cookie::build(AUTH_TOKEN).build());
                Err(e)
            }
        },
        None => {
            info!("No auth token found");
            Err(AppError::UnAuthorized)
        }
    }
}

fn parse_token(token: String) -> Result<Uuid, AppError> {
    if token.is_empty() {
        return Err(AppError::UnAuthorized);
    }

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 4 {
        return Err(AppError::UnAuthorized);
    }

    let user_id_str = parts[1];

    Uuid::parse_str(user_id_str).map_err(|_| AppError::UnAuthorized)
}
