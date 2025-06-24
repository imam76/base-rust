# Rust Base Application

Aplikasi API backend berbasis Rust menggunakan Axum framework dengan database PostgreSQL.

## 📋 Daftar Isi
- [Persyaratan Sistem](#persyaratan-sistem)
- [Instalasi](#instalasi)
- [Konfigurasi Database](#konfigurasi-database)
- [Menjalankan Aplikasi](#menjalankan-aplikasi)
- [API Endpoints](#api-endpoints)
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
PORT=5000
RUST_LOG=info
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

Aplikasi akan berjalan di: `http://127.0.0.1:5000`

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
curl http://localhost:5000/

# Response: "Hello, world!"
```

### 2. Ambil Data Users
```bash
# GET request untuk mengambil semua users
curl http://localhost:5000/api/v1/users

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
curl -X POST http://localhost:5000/api/v1/account_subclassifications \
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
curl http://localhost:5000/api/v1/account_subclassifications

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
lsof -ti:5000 | xargs kill -9  # MacOS/Linux
netstat -ano | findstr :5000   # Windows
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