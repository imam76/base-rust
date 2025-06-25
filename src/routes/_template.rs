// src/routes/_template.rs
// 🔥 ROUTE TEMPLATE - Copy paste untuk route baru!

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::template; // 👈 Ganti sesuai handler

// ========================
// TEMPLATE ROUTES
// ========================

/// Template CRUD routes
/// Base path: /api/v1/templates
pub fn template_routes() -> Router {
    Router::new()
        // Collection routes
        .route("/", get(template::get_all).post(template::create))
        // Individual resource routes
        .route(
            "/:id",
            get(template::get_by_id)
                .put(template::update)
                .delete(template::delete),
        )
        // Custom routes (optional)
        .route("/:id/activate", post(template::activate))
        .route("/:id/deactivate", post(template::deactivate))
        .route("/search", get(template::search))
}

// ========================
// HOW TO USE:
// ========================
// 1. Copy this file to new route file
// 2. Replace "template" with your feature name
// 3. Update handler imports
// 4. Add route to main api_v1_routes() in mod.rs:
//    .nest("/templates", template_routes())
// ========================
