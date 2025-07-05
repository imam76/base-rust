#!/bin/bash
# Quick setup script for development environment

set -e

echo "🚀 Setting up Rust Axum API development environment..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

# Check if SQLx CLI is installed
if ! command -v sqlx &> /dev/null; then
    echo "📦 Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Create .env file if it doesn't exist
if [ ! -f ".env" ]; then
    echo "📝 Creating .env file..."
    cat > .env << EOL
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_base_dev
JWT_SECRET=your-development-jwt-secret-key-at-least-32-chars
PORT=3000
HOST=127.0.0.1
RUST_LOG=debug
EOL
    echo "✅ Created .env file. Please update DATABASE_URL with your actual database connection."
fi

# Check if database is running
echo "🗄️  Checking database connection..."
if ! timeout 5 bash -c "</dev/tcp/localhost/5432" 2>/dev/null; then
    echo "⚠️  PostgreSQL is not running on localhost:5432"
    echo "   Please start PostgreSQL or update DATABASE_URL in .env"
    echo "   Docker option: docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:15"
else
    echo "✅ Database connection available"
    
    # Run migrations
    echo "🔄 Running database migrations..."
    source .env
    sqlx migrate run
    echo "✅ Migrations completed"
fi

# Install dependencies and build
echo "📦 Installing dependencies..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "🚀 To start the development server:"
echo "   cargo run"
echo ""
echo "🔧 Useful development commands:"
echo "   cargo watch -x run          # Auto-reload on changes"
echo "   cargo test                  # Run tests"
echo "   cargo clippy               # Lint code"
echo "   cargo fmt                  # Format code"
echo "   sqlx migrate add <name>    # Create new migration"
echo ""
echo "🌐 API will be available at: http://localhost:3000"
echo "📊 Health check: http://localhost:3000/health"
