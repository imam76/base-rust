// src/routes/account_subclassification.rs
use axum::{Router, routing::get};

use crate::handlers::account_subclassification;

/// Routes for account subclassification endpoints
///
/// Provides RESTful endpoints for managing account subclassifications:
/// - GET /account-subclassifications - List all account subclassifications
/// - GET /account-subclassifications/:id - Get a specific account subclassification by ID
/// - POST /account-subclassifications - Create a new account subclassification
/// - PUT /account-subclassifications/:id - Update an existing account subclassification
/// - DELETE /account-subclassifications/:id - Delete an account subclassification
pub fn routes() -> Router {
    Router::new()
        .route(
            "/",
            get(account_subclassification::get_all).post(account_subclassification::create),
        )
        .route(
            "/{id}",
            get(account_subclassification::get_by_id)
                .put(account_subclassification::update)
                .delete(account_subclassification::delete),
        )
}
