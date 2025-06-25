# Routes Documentation

## 📁 Structure Overview

```
src/routes/
├── mod.rs           # Main routes configuration
├── _template.rs     # Template for new routes
└── [feature].rs     # Individual feature routes
```

## 🛣️ Available Endpoints

### Health Check Routes
```
GET  /api/v1/health      # Basic health check with version info
GET  /api/v1/health/ready # Readiness probe (Kubernetes)
GET  /api/v1/health/live  # Liveness probe (Kubernetes)
```

### Account Subclassifications
```
GET    /api/v1/account-subclassifications     # Get all
POST   /api/v1/account-subclassifications     # Create new
GET    /api/v1/account-subclassifications/:id # Get by ID
PUT    /api/v1/account-subclassifications/:id # Update
DELETE /api/v1/account-subclassifications/:id # Soft delete
```

## 🚀 Adding New Routes

### Step 1: Copy Template
```bash
cp src/routes/_template.rs src/routes/product.rs
```

### Step 2: Update Template
```rust
// src/routes/product.rs
use crate::handlers::product; // 👈 Update import

pub fn product_routes() -> Router {
    Router::new()
        .route("/", get(product::get_all).post(product::create))
        .route("/:id", get(product::get_by_id).put(product::update).delete(product::delete))
}
```

### Step 3: Add to Main Routes
```rust
// src/routes/mod.rs
pub fn api_v1_routes() -> Router {
    Router::new()
        .nest("/account-subclassifications", account_subclassification_routes())
        .nest("/products", product_routes()) // 👈 Add here
        .nest("/health", health_routes())
}
```

## 📋 Route Conventions

### RESTful Patterns
- `GET /resources` - List all resources
- `POST /resources` - Create new resource
- `GET /resources/:id` - Get specific resource
- `PUT /resources/:id` - Update resource
- `DELETE /resources/:id` - Delete resource

### Custom Actions
- `POST /resources/:id/activate` - Custom action
- `GET /resources/search` - Search endpoint

### Health Check Standards
- `/health` - Basic health check
- `/health/ready` - Kubernetes readiness probe
- `/health/live` - Kubernetes liveness probe

## 🔧 Best Practices

1. **Consistent Naming**: Use kebab-case for URLs
2. **Version Control**: All routes under `/api/v1/`
3. **Documentation**: Comment each route group
4. **Error Handling**: Use consistent error responses
5. **Validation**: Validate all inputs
6. **Logging**: Log all route access

## 📊 Response Format

### Success Response
```json
{
  "success": true,
  "data": {...},
  "message": "Optional message",
  "request_id": "uuid",
  "timestamp": "2025-06-25T10:30:00Z"
}
```

### Error Response
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error message",
    "validation_errors": [...]
  },
  "request_id": "uuid",
  "timestamp": "2025-06-25T10:30:00Z"
}
```
