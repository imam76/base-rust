use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:5001")?;

    hc.do_get("/api/v1/hi").await?.print().await?;
    hc.do_get("/version").await?.print().await?;

    let req_login = hc.do_post(
        "/api/auth",
        json!({
            "email": "yqwhjahsdjhuuushdajshdjh@mailinator.com",
            "password": "123123"
        }),
    );
    req_login.await?.print().await?;

    let get_contacts = hc
        .do_get("/api/v1/contacts?search_fields=first_name,last_name&search_value=john")
        .await?;
    get_contacts.print().await?;

    // let req_create_contact = hc.do_post(
    //     "/api/v1/contacts",
    //     json!({
    //         "first_name": "John",
    //         "last_name": "Doe",
    //         "email": "john.doe@example.com",
    //         "phone": "+1-555-0123",
    //         "mobile": "+1-555-0124",
    //         "company": "Tech Solutions Inc",
    //         "address_line1": "123 Main Street",
    //         "address_line2": "Suite 100",
    //         "city": "New York",
    //         "state": "NY",
    //         "postal_code": "10001",
    //         "country": "United States",
    //         "billing_address_line1": "123 Main Street",
    //         "billing_address_line2": "Suite 100",
    //         "billing_city": "New York",
    //         "billing_state": "NY",
    //         "billing_postal_code": "10001",
    //         "billing_country": "United States",
    //         "delivery_address_line1": "456 Oak Avenue",
    //         "delivery_address_line2": "Floor 2",
    //         "delivery_city": "Brooklyn",
    //         "delivery_state": "NY",
    //         "delivery_postal_code": "11201",
    //         "delivery_country": "United States",
    //         "is_customer": true,
    //         "is_employee": false,
    //         "is_supplier": false,
    //         "is_active": true
    //     }),
    // );
    // req_create_contact.await?.print().await?;

    Ok(())
}
