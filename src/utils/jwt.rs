use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // Username
    pub email: String,    // Email
    pub iat: i64,         // Issued at
    pub exp: i64,         // Expiry time
    pub iss: String,      // Issuer
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, email: String) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::hours(24); // Token valid for 24 hours

        Self {
            sub: user_id.to_string(),
            username,
            email,
            iat: now.timestamp(),
            exp: expiry.timestamp(),
            iss: "rust-base-api".to_string(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, AppError> {
        Uuid::parse_str(&self.sub).map_err(|_| AppError::UnAuthorized)
    }
}

pub struct JwtService;

impl JwtService {
    fn get_secret() -> String {
        env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-change-this-in-production".to_string())
    }

    pub fn generate_token(
        user_id: Uuid,
        username: String,
        email: String,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, username, email);
        let secret = Self::get_secret();

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("Failed to generate JWT token: {}", e);
            AppError::InternalServerError("Failed to generate authentication token".to_string())
        })
    }

    pub fn validate_token(token: &str) -> Result<Claims, AppError> {
        let secret = Self::get_secret();
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["rust-base-api"]);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| {
            tracing::info!("JWT validation failed: {}", e);
            AppError::UnAuthorized
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token_generation_and_validation() {
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();

        // Generate token
        let token = JwtService::generate_token(user_id, username.clone(), email.clone()).unwrap();
        assert!(!token.is_empty());

        // Validate token
        let claims = JwtService::validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
        assert_eq!(claims.email, email);
        assert_eq!(claims.iss, "rust-base-api");
    }

    #[test]
    fn test_invalid_token() {
        let result = JwtService::validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
