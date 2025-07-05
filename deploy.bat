@echo off
REM Fly.io deployment script for Rust Axum API (Windows)

echo 🚀 Starting Fly.io deployment for Rust Base API...

REM Check if flyctl is installed
flyctl version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ flyctl is not installed. Please install it first:
    echo    https://fly.io/docs/hands-on/install-flyctl/
    exit /b 1
)

REM Check if logged in to Fly.io
flyctl auth whoami >nul 2>&1
if %errorlevel% neq 0 (
    echo 🔐 Please log in to Fly.io first:
    flyctl auth login
)

REM Set app name
set APP_NAME=rust-base-api

REM Check if app exists
flyctl apps list | findstr "%APP_NAME%" >nul 2>&1
if %errorlevel% neq 0 (
    echo 📱 Creating new Fly.io app: %APP_NAME%
    flyctl apps create "%APP_NAME%" --org personal
) else (
    echo 📱 App %APP_NAME% already exists
)

REM Set secrets if .env file exists
if exist ".env" (
    echo 🔑 Setting environment secrets...
    echo Please set your secrets manually using:
    echo flyctl secrets set DATABASE_URL=postgresql://... --app %APP_NAME%
    echo flyctl secrets set JWT_SECRET=your-secret-key --app %APP_NAME%
) else (
    echo ⚠️  No .env file found. Make sure to set DATABASE_URL and other secrets manually:
    echo    flyctl secrets set DATABASE_URL=postgresql://... --app %APP_NAME%
    echo    flyctl secrets set JWT_SECRET=your-secret-key --app %APP_NAME%
)

REM Setup PostgreSQL database
echo 🗄️  Setting up PostgreSQL database...
flyctl postgres list | findstr "%APP_NAME%-db" >nul 2>&1
if %errorlevel% neq 0 (
    echo Creating new PostgreSQL database: %APP_NAME%-db
    flyctl postgres create --name "%APP_NAME%-db" --region sin --vm-size shared-cpu-1x --volume-size 1
    
    echo Attaching database to app...
    flyctl postgres attach "%APP_NAME%-db" --app "%APP_NAME%"
) else (
    echo Database %APP_NAME%-db already exists
)

REM Build and deploy
echo 🏗️  Building and deploying application...
flyctl deploy --app "%APP_NAME%"

REM Show deployment status
echo ✅ Deployment completed!
echo.
echo 🌐 Your app is available at: https://%APP_NAME%.fly.dev
echo 📊 Check status: flyctl status --app %APP_NAME%
echo 📝 View logs: flyctl logs --app %APP_NAME%
echo 🔧 Open dashboard: flyctl dashboard --app %APP_NAME%
echo.
echo 🔗 Useful commands:
echo    flyctl ssh console --app %APP_NAME%  # SSH into the app
echo    flyctl proxy 5432 -a %APP_NAME%-db   # Connect to database locally
echo    flyctl secrets list --app %APP_NAME% # View secrets

pause
