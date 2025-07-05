#!/bin/bash
# Fly.io deployment script for Rust Axum API

set -e

echo "🚀 Starting Fly.io deployment for Rust Base API..."

# Check if flyctl is installed
if ! command -v flyctl &> /dev/null; then
    echo "❌ flyctl is not installed. Please install it first:"
    echo "   https://fly.io/docs/hands-on/install-flyctl/"
    exit 1
fi

# Check if logged in to Fly.io
if ! flyctl auth whoami &> /dev/null; then
    echo "🔐 Please log in to Fly.io first:"
    flyctl auth login
fi

# Check if app exists
APP_NAME="rust-base-api"
if ! flyctl apps list | grep -q "$APP_NAME"; then
    echo "📱 Creating new Fly.io app: $APP_NAME"
    flyctl apps create "$APP_NAME" --org personal
else
    echo "📱 App $APP_NAME already exists"
fi

# Set secrets (if .env file exists)
if [ -f ".env" ]; then
    echo "🔑 Setting environment secrets..."
    
    # Read .env file and set secrets
    while IFS='=' read -r key value; do
        # Skip comments and empty lines
        if [[ ! "$key" =~ ^#.*$ ]] && [[ -n "$key" ]]; then
            # Remove quotes from value if present
            value=$(echo "$value" | sed 's/^["'\'']//' | sed 's/["'\'']$//')
            echo "Setting secret: $key"
            flyctl secrets set "$key=$value" --app "$APP_NAME"
        fi
    done < .env
else
    echo "⚠️  No .env file found. Make sure to set DATABASE_URL and other secrets manually:"
    echo "   flyctl secrets set DATABASE_URL=postgresql://... --app $APP_NAME"
fi

# Check if PostgreSQL database is needed
echo "🗄️  Setting up PostgreSQL database..."
if ! flyctl postgres list | grep -q "$APP_NAME-db"; then
    echo "Creating new PostgreSQL database: $APP_NAME-db"
    flyctl postgres create --name "$APP_NAME-db" --region sin --vm-size shared-cpu-1x --volume-size 1
    
    echo "Attaching database to app..."
    flyctl postgres attach "$APP_NAME-db" --app "$APP_NAME"
else
    echo "Database $APP_NAME-db already exists"
fi

# Build and deploy
echo "🏗️  Building and deploying application..."
flyctl deploy --app "$APP_NAME"

# Show deployment status
echo "✅ Deployment completed!"
echo ""
echo "🌐 Your app is available at: https://$APP_NAME.fly.dev"
echo "📊 Check status: flyctl status --app $APP_NAME"
echo "📝 View logs: flyctl logs --app $APP_NAME"
echo "🔧 Open dashboard: flyctl dashboard --app $APP_NAME"
echo ""
echo "🔗 Useful commands:"
echo "   flyctl ssh console --app $APP_NAME  # SSH into the app"
echo "   flyctl proxy 5432 -a $APP_NAME-db   # Connect to database locally"
echo "   flyctl secrets list --app $APP_NAME # View secrets"
