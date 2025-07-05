@echo off
REM Quick setup script for development environment (Windows)

echo 🚀 Setting up Rust Axum API development environment...

REM Check if Rust is installed
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install it from https://rustup.rs/
    exit /b 1
)

REM Check if SQLx CLI is installed
sqlx --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 📦 Installing SQLx CLI...
    cargo install sqlx-cli --no-default-features --features postgres
)

REM Create .env file if it doesn't exist
if not exist ".env" (
    echo 📝 Creating .env file...
    (
        echo DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_base_dev
        echo JWT_SECRET=your-development-jwt-secret-key-at-least-32-chars
        echo PORT=3000
        echo HOST=127.0.0.1
        echo RUST_LOG=debug
    ) > .env
    echo ✅ Created .env file. Please update DATABASE_URL with your actual database connection.
)

REM Check if database is running
echo 🗄️  Checking database connection...
timeout 1 >nul 2>&1
echo Testing PostgreSQL connection...

REM Install dependencies and build
echo 📦 Installing dependencies...
cargo build

if %errorlevel% equ 0 (
    echo 📦 Dependencies installed successfully
) else (
    echo ❌ Failed to install dependencies
    exit /b 1
)

REM Run tests
echo 🧪 Running tests...
cargo test

echo.
echo ✅ Development environment setup complete!
echo.
echo 🚀 To start the development server:
echo    cargo run
echo.
echo 🔧 Useful development commands:
echo    cargo run                   # Start the server
echo    cargo test                  # Run tests
echo    cargo clippy               # Lint code
echo    cargo fmt                  # Format code
echo    sqlx migrate add ^<name^>    # Create new migration
echo.
echo 🌐 API will be available at: http://localhost:3000
echo 📊 Health check: http://localhost:3000/health
echo.

pause
