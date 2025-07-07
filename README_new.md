# Rust Base Application

A REST API backend built with Rust using Axum framework and PostgreSQL database.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- PostgreSQL 12+
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`

### Setup
1. **Clone and install dependencies**
```bash
git clone <repository-url>
cd rust-base
cargo build
```

2. **Environment setup**
Create `.env` file:
```env
DATABASE_URL=postgresql://username:password@localhost/rust_base_db
JWT_SECRET=your-super-secret-jwt-key-here
PORT=3000
RUST_LOG=info
```

3. **Database setup**
```bash
# Create database
createdb rust_base_db

# Run migrations
sqlx migrate run
```

4. **Run application**
```bash
# Development (with auto-reload)
cargo watch -x run

# Production
cargo run --release
```

App runs at: `http://localhost:3000`

## 📡 API Endpoints

### Authentication
- `POST /api/auth` - Login
- `POST /api/auth/logout` - Logout
- `POST /api/auth/refresh` - Refresh token
- `GET /api/auth/me` - Get current user info

### Users
- `GET /api/v1/users` - List users (with pagination, search, filter)
- `GET /api/v1/users/{id}` - Get user by ID
- `POST /api/v1/users` - Create user
- `PUT /api/v1/users/{id}` - Update user
- `DELETE /api/v1/users/{id}` - Delete user

### Contacts
- `GET /api/v1/contacts` - List contacts
- `GET /api/v1/contacts/{id}` - Get contact by ID
- `POST /api/v1/contacts` - Create contact
- `PUT /api/v1/contacts/{id}` - Update contact
- `DELETE /api/v1/contacts/{id}` - Delete contact

### Query Parameters
```bash
# Pagination
GET /api/v1/users?page=1&perPage=10

# Search
GET /api/v1/users?search=john

# Filter (JSON)
GET /api/v1/users?filter={"is_active":true}

# Sort
GET /api/v1/users?sortBy=created_at&sortOrder=desc
```

## 🐳 Docker

```bash
# Build and run with Docker Compose
docker-compose up --build

# Or build manually
docker build -t rust-base .
docker run -p 3000:3000 -e DATABASE_URL="..." -e JWT_SECRET="..." rust-base
```

## 🚀 Deployment

### Fly.io (Recommended)
```bash
# Quick deploy
./deploy.sh

# Manual
fly auth login
fly apps create rust-base
fly secrets set DATABASE_URL="..." JWT_SECRET="..."
fly deploy
```

## 🛠️ Development

### Project Structure
```
src/
├── main.rs              # Entry point
├── models/              # Database models
├── handlers/            # API handlers
├── routes/              # Route definitions
├── utils/               # Utilities (JWT, CRUD, Query Builder)
└── middlewares/         # Auth middleware
migrations/              # Database migrations
```

### Testing
```bash
# Run tests
cargo test

# Test API endpoints
./test_api.sh
```

### Features
- ✅ JWT Authentication
- ✅ CRUD Operations with Generic Service
- ✅ Pagination, Search, Filter, Sort
- ✅ Database Migrations
- ✅ Docker Support
- ✅ Production Ready

## 🔧 Troubleshooting

**Database connection issues:**
```bash
# Check PostgreSQL is running
sudo systemctl start postgresql

# Test connection
psql -h localhost -U username -d rust_base_db
```

**Migration issues:**
```bash
# Check migration status
sqlx migrate info

# Reset if needed
sqlx database drop && sqlx database create && sqlx migrate run
```

**Port conflicts:**
```bash
# Change PORT in .env file
PORT=3001
```

---

**Ready to build amazing APIs with Rust! 🦀**
