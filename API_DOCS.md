# API Documentation

## Account Subclassification Endpoints

Base URL: `/api/v1/account-subclassifications`

### Endpoints

| Method | Endpoint | Description | Handler Function |
|--------|----------|-------------|-----------------|
| GET | `/` | Get all account subclassifications | `get_all` |
| GET | `/:id` | Get account subclassification by ID | `get_by_id` |
| POST | `/` | Create new account subclassification | `create` |
| PUT | `/:id` | Update account subclassification | `update` |
| DELETE | `/:id` | Delete account subclassification | `delete` |

### Example Usage

```bash
# Get all account subclassifications
curl -X GET http://localhost:5000/api/v1/account-subclassifications

# Get specific account subclassification
curl -X GET http://localhost:5000/api/v1/account-subclassifications/{id}

# Create new account subclassification
curl -X POST http://localhost:5000/api/v1/account-subclassifications \
  -H "Content-Type: application/json" \
  -d '{
    "code": "ACC001",
    "name": "Current Assets",
    "cash_flow_type": "operating",
    "is_variable_cost": false,
    "account_classification_id": "uuid-here",
    "is_parent": false,
    "is_active": true
  }'

# Update account subclassification
curl -X PUT http://localhost:5000/api/v1/account-subclassifications/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "code": "ACC001_UPDATED",
    "name": "Updated Current Assets",
    "cash_flow_type": "operating",
    "is_variable_cost": false,
    "account_classification_id": "uuid-here",
    "is_parent": false,
    "is_active": true
  }'

# Delete account subclassification
curl -X DELETE http://localhost:5000/api/v1/account-subclassifications/{id}
```

## Health Check Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Simple health check |
| GET | `/api/v1/health` | API v1 health check |

### Response Format

All API responses follow a standardized format:

**Success Response:**
```json
{
  "success": true,
  "data": { ... },
  "message": "Operation successful"
}
```

**Error Response:**
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validation failed",
    "details": {
      "field": "error description"
    }
  }
}
```

## Adding New Route Modules

1. Create a new file in `src/routes/` (e.g., `user.rs`)
2. Implement your routes using the template in `src/routes/_template.rs`
3. Add the module to `src/routes/mod.rs`:
   ```rust
   pub mod user;
   ```
4. Nest the routes in `api_v1_routes()`:
   ```rust
   .nest("/users", user::routes())
   ```

See `src/routes/README.md` for detailed conventions and best practices.
