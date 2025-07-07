use axum::{
    extract::{Path, Query, State},
    response::Response,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    models::{
        AppState, AuthenticatedUser, CreateUserRequest, UpdateUserRequest, User, UserResponse,
    },
    utils::{CrudService, PaginatedResponse, QueryParams},
    AppError,
};

const TABLE: &str = "users";
const SELECT_FIELDS: &[&str] = &[
    "id",
    "username",
    "email",
    "first_name",
    "last_name",
    "is_active",
    "is_verified",
    "last_login_at",
    "created_at",
    "updated_at",
];
const SEARCHABLE_FIELDS: &[&str] = &["username", "email", "first_name", "last_name"];
const FILTERABLE_FIELDS: &[&str] = &["is_active", "is_verified"];
const SORTABLE_FIELDS: &[&str] = &[
    "username",
    "email",
    "first_name",
    "last_name",
    "created_at",
    "updated_at",
];
const JOINS: &[&str] = &[];

// GET /api/v1/users
pub async fn get_users(
    query: Query<QueryParams>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Json<PaginatedResponse<UserResponse>>, AppError> {
    let Json(response_data) = CrudService::get_list::<User>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        SEARCHABLE_FIELDS.to_vec(),
        FILTERABLE_FIELDS.to_vec(),
        SORTABLE_FIELDS.to_vec(),
        JOINS.to_vec(),
        query,
        state,
        "/api/v1/users",
        Some(auth),
    )
    .await?;

    // Convert User to UserResponse
    let converted_data: Vec<UserResponse> = response_data
        .results
        .into_iter()
        .map(|user| user.into())
        .collect();

    Ok(Json(PaginatedResponse {
        count: response_data.count,
        page_context: response_data.page_context,
        links: response_data.links,
        results: converted_data,
    }))
}

// GET /api/v1/users/:id
pub async fn get_user_by_id(
    id: Path<Uuid>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Json<UserResponse>, AppError> {
    let Json(user) = CrudService::get_by_id::<User>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        JOINS.to_vec(),
        id,
        state,
        Some(auth),
    )
    .await?;

    Ok(Json(user.into()))
}

// POST /api/v1/users
pub async fn create_user(
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
    Json(create_data): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    // Hash password before creating user
    let password_hash = bcrypt::hash(&create_data.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))?;

    // Create user data with hashed password
    let user_data = serde_json::json!({
        "username": create_data.username,
        "email": create_data.email,
        "password_hash": password_hash,
        "first_name": create_data.first_name,
        "last_name": create_data.last_name,
        "is_active": true,
        "is_verified": false
    });

    let Json(user) =
        CrudService::create::<User, serde_json::Value>(TABLE, Json(user_data), state, auth).await?;

    Ok(Json(user.into()))
}

// PUT /api/v1/users/:id
pub async fn update_user(
    id: Path<Uuid>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
    Json(update_data): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let Json(user) =
        CrudService::update::<User, UpdateUserRequest>(TABLE, id, Json(update_data), state, auth)
            .await?;

    Ok(Json(user.into()))
}

// DELETE /api/v1/users/:id
pub async fn delete_user(
    id: Path<Uuid>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Response, AppError> {
    CrudService::delete(TABLE, id, state, auth).await
}

// For backward compatibility
pub async fn get_all_users(
    query: Query<QueryParams>,
    state: State<AppState>,
) -> Result<Json<PaginatedResponse<UserResponse>>, AppError> {
    let Json(response_data) = CrudService::get_list::<User>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        SEARCHABLE_FIELDS.to_vec(),
        FILTERABLE_FIELDS.to_vec(),
        SORTABLE_FIELDS.to_vec(),
        JOINS.to_vec(),
        query,
        state,
        "/api/v1/users/all",
        None, // No auth required for backward compatibility
    )
    .await?;

    // Convert User to UserResponse
    let converted_data: Vec<UserResponse> = response_data
        .results
        .into_iter()
        .map(|user| user.into())
        .collect();

    Ok(Json(PaginatedResponse {
        count: response_data.count,
        page_context: response_data.page_context,
        links: response_data.links,
        results: converted_data,
    }))
}
