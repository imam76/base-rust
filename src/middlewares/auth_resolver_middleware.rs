use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;
use tracing::info;

use crate::{utils::constants::AUTH_TOKEN, AppError};

pub async fn start(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    info!("-> MIDDLEWARE_API - api {}", req.uri().path());
    // Extract the user_id from cookies
    let auth_tokken = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string());

    // If the auth token is not present, return an error
    if auth_tokken.is_none() {
        info!("Auth token not found in cookies");
        return Err(AppError::UnAuthorized);
    }

    info!("Auth Token => {:?}", auth_tokken);
    Ok(next.run(req).await)
}
