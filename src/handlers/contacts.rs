use axum::{
    extract::{Path, Query, State},
    response::Response,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    models::{
        AppState, AuthenticatedUser, Contact, ContactResponse, CreateContactRequest,
        UpdateContactRequest,
    },
    utils::{CrudService, PaginatedResponse, QueryParams},
    AppError,
};

const TABLE: &str = "contacts";
const SELECT_FIELDS: &[&str] = &[
    "id",
    "first_name",
    "last_name",
    "email",
    "phone",
    "mobile",
    "company",
    "address_line1",
    "address_line2",
    "city",
    "state",
    "postal_code",
    "country",
    "billing_address_line1",
    "billing_address_line2",
    "billing_city",
    "billing_state",
    "billing_postal_code",
    "billing_country",
    "delivery_address_line1",
    "delivery_address_line2",
    "delivery_city",
    "delivery_state",
    "delivery_postal_code",
    "delivery_country",
    "is_customer",
    "is_employee",
    "is_supplier",
    "is_active",
    "created_at",
    "created_by",
    "updated_at",
    "updated_by",
];
const SEARCHABLE_FIELDS: &[&str] = &[
    "first_name",
    "last_name",
    "email",
    "phone",
    "mobile",
    "company",
    "city",
    "state",
    "country",
];
const FILTERABLE_FIELDS: &[&str] = &[
    "is_customer",
    "is_employee",
    "is_supplier",
    "is_active",
    "city",
    "state",
    "country",
];
const SORTABLE_FIELDS: &[&str] = &[
    "first_name",
    "last_name",
    "email",
    "company",
    "created_at",
    "updated_at",
];
const JOINS: &[&str] = &[];

// GET /api/v1/contacts
pub async fn get_contacts(
    query: Query<QueryParams>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Json<PaginatedResponse<ContactResponse>>, AppError> {
    let includes = vec![
        (
            "created_user",
            "LEFT JOIN users created_user ON contacts.created_by = created_user.id",
            vec![
                "created_user.id as created_user_id",
                "created_user.first_name as created_user_name",
            ],
        ),
        (
            "updated_user",
            "LEFT JOIN users updated_user ON contacts.updated_by = updated_user.id",
            vec![
                "updated_user.id as updated_user_id",
                "updated_user.first_name as updated_user_name",
            ],
        ),
    ];

    let Json(response_data) = CrudService::get_list_with_includes::<Contact>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        SEARCHABLE_FIELDS.to_vec(),
        FILTERABLE_FIELDS.to_vec(),
        SORTABLE_FIELDS.to_vec(),
        JOINS.to_vec(),
        includes,
        query,
        state,
        "/api/v1/contacts",
        Some(auth),
    )
    .await?;

    // Convert Contact to ContactResponse
    let converted_data: Vec<ContactResponse> = response_data
        .results
        .into_iter()
        .map(|contact| contact.into())
        .collect();

    Ok(Json(PaginatedResponse {
        count: response_data.count,
        page_context: response_data.page_context,
        links: response_data.links,
        results: converted_data,
    }))
}

// GET /api/v1/contacts/:id
pub async fn get_contact_by_id(
    id: Path<Uuid>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Json<ContactResponse>, AppError> {
    let Json(contact) = CrudService::get_by_id::<Contact>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        JOINS.to_vec(),
        id,
        state,
        Some(auth),
    )
    .await?;

    Ok(Json(contact.into()))
}

// POST /api/v1/contacts
pub async fn create_contact(
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
    Json(create_data): Json<CreateContactRequest>,
) -> Result<Json<ContactResponse>, AppError> {
    // Create contact data with default values
    let contact_data = serde_json::json!({
        "first_name": create_data.first_name,
        "last_name": create_data.last_name,
        "email": create_data.email,
        "phone": create_data.phone,
        "mobile": create_data.mobile,
        "company": create_data.company,
        "address_line1": create_data.address_line1,
        "address_line2": create_data.address_line2,
        "city": create_data.city,
        "state": create_data.state,
        "postal_code": create_data.postal_code,
        "country": create_data.country.unwrap_or_else(|| "United States".to_string()),
        "billing_address_line1": create_data.billing_address_line1,
        "billing_address_line2": create_data.billing_address_line2,
        "billing_city": create_data.billing_city,
        "billing_state": create_data.billing_state,
        "billing_postal_code": create_data.billing_postal_code,
        "billing_country": create_data.billing_country,
        "delivery_address_line1": create_data.delivery_address_line1,
        "delivery_address_line2": create_data.delivery_address_line2,
        "delivery_city": create_data.delivery_city,
        "delivery_state": create_data.delivery_state,
        "delivery_postal_code": create_data.delivery_postal_code,
        "delivery_country": create_data.delivery_country,
        "is_customer": create_data.is_customer.unwrap_or(false),
        "is_employee": create_data.is_employee.unwrap_or(false),
        "is_supplier": create_data.is_supplier.unwrap_or(false),
        "is_active": true
    });

    let Json(contact) =
        CrudService::create::<Contact, serde_json::Value>(TABLE, Json(contact_data), state, auth)
            .await?;

    Ok(Json(contact.into()))
}

// PUT /api/v1/contacts/:id
pub async fn update_contact(
    id: Path<Uuid>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
    Json(update_data): Json<UpdateContactRequest>,
) -> Result<Json<ContactResponse>, AppError> {
    let Json(contact) = CrudService::update::<Contact, UpdateContactRequest>(
        TABLE,
        id,
        Json(update_data),
        state,
        auth,
    )
    .await?;

    Ok(Json(contact.into()))
}

// DELETE /api/v1/contacts/:id
pub async fn delete_contact(
    id: Path<Uuid>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Response, AppError> {
    CrudService::delete(TABLE, id, state, auth).await
}

// GET /api/v1/contacts/customers
pub async fn get_customers(
    query: Query<QueryParams>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Json<PaginatedResponse<ContactResponse>>, AppError> {
    // Add customer filter to the query
    let mut params = query.0;
    let customer_filter = serde_json::json!({"is_customer": true});
    params.filter = Some(customer_filter.to_string());

    let Json(response_data) = CrudService::get_list::<Contact>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        SEARCHABLE_FIELDS.to_vec(),
        FILTERABLE_FIELDS.to_vec(),
        SORTABLE_FIELDS.to_vec(),
        JOINS.to_vec(),
        Query(params),
        state,
        "/api/v1/contacts/customers",
        Some(auth),
    )
    .await?;

    // Convert Contact to ContactResponse
    let converted_data: Vec<ContactResponse> = response_data
        .results
        .into_iter()
        .map(|contact| contact.into())
        .collect();

    Ok(Json(PaginatedResponse {
        count: response_data.count,
        page_context: response_data.page_context,
        links: response_data.links,
        results: converted_data,
    }))
}

// GET /api/v1/contacts/suppliers
pub async fn get_suppliers(
    query: Query<QueryParams>,
    state: State<AppState>,
    auth: Extension<AuthenticatedUser>,
) -> Result<Json<PaginatedResponse<ContactResponse>>, AppError> {
    // Add supplier filter to the query
    let mut params = query.0;
    let supplier_filter = serde_json::json!({"is_supplier": true});
    params.filter = Some(supplier_filter.to_string());

    let Json(response_data) = CrudService::get_list::<Contact>(
        TABLE,
        SELECT_FIELDS.to_vec(),
        SEARCHABLE_FIELDS.to_vec(),
        FILTERABLE_FIELDS.to_vec(),
        SORTABLE_FIELDS.to_vec(),
        JOINS.to_vec(),
        Query(params),
        state,
        "/api/v1/contacts/suppliers",
        Some(auth),
    )
    .await?;

    // Convert Contact to ContactResponse
    let converted_data: Vec<ContactResponse> = response_data
        .results
        .into_iter()
        .map(|contact| contact.into())
        .collect();

    Ok(Json(PaginatedResponse {
        count: response_data.count,
        page_context: response_data.page_context,
        links: response_data.links,
        results: converted_data,
    }))
}
