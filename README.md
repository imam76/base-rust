# Rust Base Application

Aplikasi API backend berbasis Rust menggunakan Axum framework dengan database PostgreSQL.

## 📋 Daftar Isi
- [Persyaratan Sistem](#persyaratan-sistem)
- [Instalasi](#instalasi)
- [Konfigurasi Database](#konfigurasi-database)
- [Menjalankan Aplikasi](#menjalankan-aplikasi)
- [API Endpoints](#api-endpoints)
- [🚀 Production Deployment](#-production-deployment)
- [🐳 Docker Support](#-docker-support)
- [Cara Penggunaan](#cara-penggunaan)
- [Troubleshooting](#troubleshooting)

## 🔧 Persyaratan Sistem

Sebelum menjalankan aplikasi, pastikan sistem Anda memiliki:

- **Rust** (versi 1.70 atau lebih baru)
- **PostgreSQL** (versi 12 atau lebih baru)
- **SQLx CLI** untuk menjalankan migrasi database

### Instalasi Rust
```bash
# Install Rust menggunakan rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Instalasi SQLx CLI
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Instalasi PostgreSQL
- **Windows**: Download dari [postgresql.org](https://www.postgresql.org/download/windows/)
- **MacOS**: `brew install postgresql`
- **Ubuntu/Debian**: `sudo apt-get install postgresql postgresql-contrib`

## 🚀 Instalasi

1. **Clone repository**
```bash
git clone <repository-url>
cd rust-base
```

2. **Install dependencies**
```bash
cargo build
```


3. **Setup environment variables**
Buat file `.env` di root project:
```env
DATABASE_URL=postgresql://username:password@localhost/rust_base_db
PORT=5001
RUST_LOG=info
```

## Dev (REPL)

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the tests.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```


## 🗄️ Konfigurasi Database

1. **Buat database PostgreSQL**
```sql
CREATE DATABASE rust_base_db;
```

2. **Jalankan migrasi database**
```bash
# Jalankan semua migrasi
sqlx migrate run

# Atau jika ingin melihat status migrasi
sqlx migrate info
```

3. **Verifikasi tabel yang dibuat**
```sql
-- Tabel yang akan dibuat:
-- users
-- contacts
-- term_of_payments
-- currencies
-- attachments
-- account_classifications
-- contact_classifications
-- account_subclassifications
```

## ▶️ Menjalankan Aplikasi

### Mode Development
```bash
# Jalankan dengan auto-reload
cargo watch -x run

# Atau jalankan biasa
cargo run
```

### Mode Production
```bash
# Build release version
cargo build --release

# Jalankan executable
./target/release/rust-base
```

Aplikasi akan berjalan di: `http://127.0.0.1:5001`

## 🔌 API Endpoints

### Base Endpoints
- `GET /` - Root endpoint (Hello World)
- `GET /hi` - Test endpoint

### User Management
- `GET /users` - Ambil semua users
- `GET /api/v1/users` - Ambil semua users (API v1)

### Account Subclassification
- `GET /api/v1/account_subclassifications` - Ambil semua account subclassifications
- `POST /api/v1/account_subclassifications` - Buat account subclassification baru

## 📝 Cara Penggunaan

### 1. Test Koneksi Aplikasi
```bash
# Test apakah aplikasi berjalan
curl http://localhost:5001/

# Response: "Hello, world!"
```

### 2. Ambil Data Users
```bash
# GET request untuk mengambil semua users
curl http://localhost:5001/api/v1/users

# Response example:
# [
#   {
#     "id": "550e8400-e29b-41d4-a716-446655440000",
#     "username": "zombie",
#     "email": "test@mailinator.com",
#     "password_hash": "$2y$10$..."
#   }
# ]
```

### 3. Buat Account Subclassification Baru
```bash
# POST request dengan JSON data
curl -X POST http://localhost:5001/api/v1/account_subclassifications \
  -H "Content-Type: application/json" \
  -d '{
    "code": "ASC001",
    "name": "Current Assets",
    "alias_name": "CA",
    "cash_flow_type": "operating",
    "ratio_type": "current",
    "is_variable_cost": false,
    "account_classification_id": "550e8400-e29b-41d4-a716-446655440000",
    "parent_id": null,
    "is_parent": true,
    "is_active": true
  }'
```

### 4. Ambil Account Subclassifications
```bash
# GET request untuk mengambil semua account subclassifications
curl http://localhost:5001/api/v1/account_subclassifications

# Response example:
# [
#   {
#     "id": "123e4567-e89b-12d3-a456-426614174000",
#     "code": "ASC001",
#     "name": "Current Assets",
#     "alias_name": "CA",
#     "cash_flow_type": "operating",
#     "ratio_type": "current",
#     "is_variable_cost": false,
#     "is_parent": true,
#     "account_classification_id": "550e8400-e29b-41d4-a716-446655440000",
#     "parent_id": null,
#     "is_active": true,
#     "created_by": "550e8400-e29b-41d4-a716-446655440000",
#     "updated_by": "550e8400-e29b-41d4-a716-446655440000",
#     "created_at": "2025-06-24T10:30:00Z",
#     "updated_at": "2025-06-24T10:30:00Z"
#   }
# ]
```

## 🛠️ Struktur Project

```
rust-base/
├── src/
│   ├── main.rs              # Entry point aplikasi
│   ├── models/
│   │   └── mod.rs           # Database models
│   ├── handlers/
│   │   └── account_subclassification.rs  # API handlers
│   └── routes/
│       └── mod.rs           # Route definitions
├── migrations/              # Database migrations
│   ├── 20250622014122_users.sql
│   ├── 20250622174346_contacts.sql
│   └── ...
├── Cargo.toml              # Dependencies
├── .env                    # Environment variables
└── README.md               # Dokumentasi ini
```

## 📊 Database Schema

### Users Table
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Account Subclassifications Table
```sql
CREATE TABLE account_subclassifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    alias_name VARCHAR(50),
    cash_flow_type VARCHAR(50) NOT NULL CHECK (cash_flow_type IN ('operating', 'investing', 'financing')),
    ratio_type VARCHAR(50) NOT NULL,
    is_variable_cost BOOLEAN DEFAULT FALSE,
    is_parent BOOLEAN DEFAULT FALSE,
    account_classification_id UUID NOT NULL,
    parent_id UUID NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID NOT NULL REFERENCES users(id)
);
```

## 🏗️ Generic CRUD System

This application features a robust, extensible, and reusable backend architecture with a generic query builder and CRUD service that supports:

### Features
- ✅ **Pagination**: `page`, `perPage` parameters
- ✅ **Search**: Full-text search across specified fields
- ✅ **Filtering**: JSON-based filtering with type-safe parameter binding
- ✅ **Sorting**: Configurable sorting by allowed fields
- ✅ **Field Selection**: Include/exclude fields (framework ready)
- ✅ **SQL Injection Protection**: All queries use prepared statements
- ✅ **Authentication**: Integrated with auth middleware
- ✅ **Error Handling**: Comprehensive error types and responses

### Architecture Components

#### 1. Query Builder (`src/utils/query_builder.rs`)
```rust
// Example usage
let query_builder = QueryBuilder::new("users")
    .search_fields(vec!["name", "email"])
    .filterable_fields(vec!["status", "role"])
    .sortable_fields(vec!["name", "email", "created_at"]);

let (query, args, page, per_page) = query_builder.build_query(&params);
```

#### 2. CRUD Service (`src/utils/crud_service.rs`)
```rust
// Generic CRUD operations
let users = CrudService::get_list(pool, &query_builder, &params).await?;
let user = CrudService::get_by_id(pool, &uuid, "users").await?;
let new_user = CrudService::create(pool, "users", &data, &auth).await?;
let updated = CrudService::update(pool, "users", &uuid, &data, &auth).await?;
CrudService::delete(pool, "users", &uuid).await?;
```

#### 3. Handler Pattern (`src/handlers/users.rs`, `src/handlers/contacts.rs`)
Each resource handler follows a consistent pattern:
- **GET** `/resource` - List with pagination, search, filter, sort
- **GET** `/resource/{id}` - Get single item by ID
- **POST** `/resource` - Create new item (requires auth)
- **PUT** `/resource/{id}` - Update item (requires auth)
- **DELETE** `/resource/{id}` - Delete item (requires auth)

### Adding New Resources

To add a new resource (e.g., "products"):

1. **Create the model** (`src/models/product.rs`):
```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub category_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: f64,
    pub category_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub category_id: Option<Uuid>,
}
```

2. **Create the handler** (`src/handlers/products.rs`):
```rust
use axum:{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

use crate:{
    errors::{AppError, Result},
    middlewares::auth_resolver_middleware::UserContext,
    models::product::{Product, CreateProductRequest, UpdateProductRequest},
    utils::{crud_service::CrudService, query_builder::QueryBuilder, query_builder::QueryParams},
    AppState,
};

pub async fn get_products(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<serde_json::Value>> {
    let query_builder = QueryBuilder::new("products")
        .search_fields(vec!["name"])
        .filterable_fields(vec!["category_id"])
        .sortable_fields(vec!["name", "price", "created_at"]);

    let result = CrudService::get_list(&state.pool, &query_builder, &params).await?;
    Ok(Json(result))
}

pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>> {
    let product = CrudService::get_by_id(&state.pool, &id, "products").await?;
    Ok(Json(product))
}

pub async fn create_product(
    State(state): State<AppState>,
    auth: UserContext,
    Json(data): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>)> {
    let product = CrudService::create(&state.pool, "products", &data, &auth).await?;
    Ok((StatusCode::CREATED, Json(product)))
}

pub async fn update_product(
    State(state): State<AppState>,
    auth: UserContext,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateProductRequest>,
) -> Result<Json<Product>> {
    let product = CrudService::update(&state.pool, "products", &id, &data, &auth).await?;
    Ok(Json(product))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    CrudService::delete(&state.pool, "products", &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

3. **Add routes** (`src/routes/main.rs`):
```rust
use crate::handlers::products:{
    create_product, delete_product, get_product_by_id, get_products, update_product,
};

// Add to the router
.route("/products", get(get_products).post(create_product))
.route(
    "/products/{id}",
    get(get_product_by_id).put(update_product).delete(delete_product),
)
```

4. **Update module exports** (`src/models/mod.rs`, `src/handlers/mod.rs`).

### API Query Examples

```bash
# Pagination
GET /users?page=1&perPage=10

# Search
GET /users?search=john

# Filtering (JSON)
GET /users?filter={"status":"active","role":"admin"}

# Sorting
GET /users?sortBy=created_at&sortOrder=desc

# Combined
GET /users?page=1&perPage=5&search=admin&sortBy=email&sortOrder=asc&filter={"status":"active"}
```

### Testing the API

Use the provided test script:
```bash
# Make the script executable
chmod +x test_api.sh

# Run tests
./test_api.sh
```

Or test manually with curl:
```bash
# Health check
curl http://127.0.0.1:5001/health

# List users with pagination
curl "http://127.0.0.1:5001/users?page=1&perPage=10"

# Search users
curl "http://127.0.0.1:5001/users?search=admin"
```

### Security Features
- **SQL Injection Protection**: All database queries use prepared statements
- **Authentication Middleware**: Protects write operations
- **Input Validation**: Type-safe request/response models
- **Error Handling**: Secure error responses without data leakage

### Performance Features
- **Connection Pooling**: PostgreSQL connection pool with SQLx
- **Prepared Statements**: Efficient query execution
- **Pagination**: Memory-efficient data retrieval
- **Optimized Queries**: Minimal data transfer with field selection

This architecture provides a solid foundation for building scalable REST APIs with consistent patterns, comprehensive functionality, and robust security.

## 🚀 Production Deployment

Untuk menjalankan aplikasi di lingkungan production, ikuti langkah-langkah berikut:

1. **Build aplikasi untuk release**
```bash
cargo build --release
```

2. **Jalankan migrasi database**
```bash
sqlx migrate run
```

3. **Jalankan aplikasi**
```bash
./target/release/rust-base
```

4. **Akses aplikasi di browser**
Buka `http://your-server-ip:5001`

## 🚀 Production Deployment

### Quick Setup Scripts

**Development Environment:**
```bash
# Linux/macOS
chmod +x setup-dev.sh
./setup-dev.sh

# Windows
setup-dev.bat
```

### Fly.io Deployment (Recommended)

Deploy to Fly.io with PostgreSQL database:

```bash
# Quick deployment
chmod +x deploy.sh
./deploy.sh

# Or Windows
deploy.bat
```

**Manual Fly.io Setup:**
```bash
# 1. Install Fly.io CLI
curl -L https://fly.io/install.sh | sh

# 2. Login to Fly.io
flyctl auth login

# 3. Create app and database
flyctl apps create rust-base-api
flyctl postgres create --name rust-base-api-db
flyctl postgres attach rust-base-api-db --app rust-base-api

# 4. Set secrets
flyctl secrets set JWT_SECRET="your-secret-key" --app rust-base-api

# 5. Deploy
flyctl deploy --app rust-base-api
```

**📖 Complete deployment guide:** [FLY_DEPLOYMENT.md](FLY_DEPLOYMENT.md)

### GitHub Actions CI/CD

Automated deployment on push to main branch:
- ✅ Tests and linting
- ✅ Security audit
- ✅ Auto-deploy to Fly.io
- ✅ Health checks

See `.github/workflows/deploy.yml` for configuration.

### Environment Variables (Production)

```bash
# Required
DATABASE_URL=postgresql://...
JWT_SECRET=your-super-secret-key

# Optional
HOST=0.0.0.0
PORT=3000
RUST_LOG=info
```

Use `.env.production.template` as a starting point.

## 🐳 Docker Support

### Multi-stage Production Build

Optimized Docker image (~50MB) with security best practices:

```bash
# Build image
docker build -t rust-base-api .

# Run container
docker run -p 3000:3000 \
  -e DATABASE_URL="postgresql://..." \
  -e JWT_SECRET="your-secret" \
  rust-base-api
```

### Build Docker Image
```bash
docker build -t rust-base .
```

### Jalankan Docker Container
```bash
docker run -d -p 5001:5001 --name rust-base-app rust-base
```

### Menghentikan dan Menghapus Container
```bash
docker stop rust-base-app
docker rm rust-base-app
```

### Mengakses Docker Container
```bash
docker exec -it rust-base-app /bin/sh
```

Untuk informasi lebih lanjut tentang Docker, kunjungi [dokumen resmi Docker](https://docs.docker.com/).

## 🔧 Troubleshooting

### Error: Database Connection Failed
```bash
# Pastikan PostgreSQL berjalan
sudo systemctl start postgresql  # Linux
brew services start postgresql   # MacOS

# Cek koneksi database
psql -h localhost -U username -d rust_base_db
```

### Error: Migration Failed
```bash
# Reset migrasi jika diperlukan
sqlx migrate revert

# Jalankan ulang migrasi
sqlx migrate run
```

### Error: Port Already in Use
```bash
# Ubah PORT di file .env
PORT=5001

# Atau kill process yang menggunakan port
lsof -ti:5001 | xargs kill -9  # MacOS/Linux
netstat -ano | findstr :5001   # Windows
```

### Error: Compilation Failed
```bash
# Update dependencies
cargo update

# Clean build
cargo clean && cargo build
```

## 📚 Dependencies Utama

- **axum**: Web framework untuk Rust
- **sqlx**: Database toolkit asinkron
- **tokio**: Runtime asinkron
- **serde**: Serialization/deserialization
- **uuid**: UUID generation
- **chrono**: Date/time handling
- **tracing**: Logging framework

## 🤝 Kontribusi

1. Fork repository
2. Buat branch feature (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buat Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

**Selamat menggunakan aplikasi Rust Base! 🚀**

Jika ada pertanyaan atau masalah, silakan buat issue di repository ini.