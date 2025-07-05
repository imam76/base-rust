use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{
        contacts::{
            create_contact, delete_contact, get_contact_by_id, get_contacts, get_customers,
            get_suppliers, update_contact,
        },
        root::get_all,
        users::{create_user, delete_user, get_all_users, get_user_by_id, get_users, update_user},
    },
    models::AppState,
};

pub async fn routes() -> Router<AppState> {
    Router::new()
        // Root route
        .route("/", get(get_all))
        // Users routes
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
        // Contacts routes
        .route("/contacts", get(get_contacts).post(create_contact))
        .route(
            "/contacts/{id}",
            get(get_contact_by_id)
                .put(update_contact)
                .delete(delete_contact),
        )
        .route("/contacts/customers", get(get_customers))
        .route("/contacts/suppliers", get(get_suppliers))
        // Legacy route for backward compatibility
        .route("/hello", get(get_all_users))
}
