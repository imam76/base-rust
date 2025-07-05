# 🚀 Fly.io Deployment Guide

Complete guide for deploying the Rust Axum API to Fly.io with PostgreSQL database.

## 📋 Prerequisites

1. **Install Fly.io CLI**
   ```bash
   # Windows (PowerShell)
   iwr https://fly.io/install.ps1 -useb | iex
   
   # macOS
   brew install flyctl
   
   # Linux
   curl -L https://fly.io/install.sh | sh
   ```

2. **Login to Fly.io**
   ```bash
   flyctl auth login
   ```

3. **Verify Installation**
   ```bash
   flyctl version
   flyctl auth whoami
   ```

## 🛠️ Configuration Files

### **Dockerfile**
Multi-stage Docker build optimized for:
- ✅ Small image size (~50MB)
- ✅ Security (non-root user)
- ✅ Fast builds (cached dependencies)
- ✅ Health checks
- ✅ Alpine Linux base

### **fly.toml**
Fly.io configuration with:
- ✅ Auto-scaling (0-3 machines)
- ✅ Health checks on `/health`
- ✅ HTTPS force redirect
- ✅ Migration on deploy
- ✅ Singapore region (sin)

### **.dockerignore**
Optimized build context excluding:
- ✅ Development files
- ✅ Git history
- ✅ Logs and temporary files
- ✅ IDE configurations

## 🗄️ Database Setup

### **Option 1: Fly.io PostgreSQL (Recommended)**
```bash
# Create PostgreSQL database
flyctl postgres create --name rust-base-api-db --region sin

# Attach to your app
flyctl postgres attach rust-base-api-db --app rust-base-api
```

### **Option 2: External Database**
```bash
# Set DATABASE_URL secret
flyctl secrets set DATABASE_URL="postgresql://user:pass@host:5432/dbname" --app rust-base-api
```

## 🔑 Environment Configuration

### **Required Secrets**
```bash
flyctl secrets set JWT_SECRET="your-super-secret-jwt-key" --app rust-base-api
flyctl secrets set DATABASE_URL="postgresql://..." --app rust-base-api
```

### **Optional Environment Variables**
```bash
flyctl secrets set RUST_LOG="info" --app rust-base-api
flyctl secrets set HOST="0.0.0.0" --app rust-base-api
flyctl secrets set PORT="3000" --app rust-base-api
```

## 🚀 Deployment Methods

### **Method 1: Automated Script**
```bash
# Linux/macOS
chmod +x deploy.sh
./deploy.sh

# Windows
deploy.bat
```

### **Method 2: Manual Steps**
```bash
# 1. Create app
flyctl apps create rust-base-api --org personal

# 2. Set secrets
flyctl secrets set DATABASE_URL="postgresql://..." --app rust-base-api
flyctl secrets set JWT_SECRET="your-secret-key" --app rust-base-api

# 3. Deploy
flyctl deploy --app rust-base-api
```

### **Method 3: GitHub Actions (CI/CD)**
Create `.github/workflows/deploy.yml`:
```yaml
name: Deploy to Fly.io

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: superfly/flyctl-actions/setup-flyctl@master
      - run: flyctl deploy --remote-only
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
```

## 📊 Post-Deployment

### **Check Deployment Status**
```bash
flyctl status --app rust-base-api
flyctl logs --app rust-base-api
```

### **Test API Endpoints**
```bash
# Health check
curl https://rust-base-api.fly.dev/health

# Version info
curl https://rust-base-api.fly.dev/version

# Login test
curl -X POST https://rust-base-api.fly.dev/api/auth \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password"}'
```

### **Database Management**
```bash
# Connect to database locally
flyctl proxy 5432 -a rust-base-api-db

# SSH into app
flyctl ssh console --app rust-base-api

# Run migrations manually
flyctl ssh console --app rust-base-api -C "sqlx migrate run"
```

## 🔧 Useful Commands

### **App Management**
```bash
# View app info
flyctl info --app rust-base-api

# Scale app
flyctl scale count 2 --app rust-base-api

# View secrets
flyctl secrets list --app rust-base-api

# Update secret
flyctl secrets set KEY=VALUE --app rust-base-api
```

### **Monitoring**
```bash
# Real-time logs
flyctl logs --app rust-base-api -f

# Metrics dashboard
flyctl dashboard --app rust-base-api

# SSH into machine
flyctl ssh console --app rust-base-api
```

### **Database Operations**
```bash
# Database console
flyctl postgres connect -a rust-base-api-db

# Database proxy
flyctl proxy 5432 -a rust-base-api-db

# Backup database
flyctl postgres backup list -a rust-base-api-db
```

## 🛡️ Security Best Practices

### **Secrets Management**
- ✅ Use `flyctl secrets` for sensitive data
- ✅ Never commit secrets to git
- ✅ Rotate JWT secrets regularly
- ✅ Use strong database passwords

### **Network Security**
- ✅ HTTPS enforced by default
- ✅ Health checks configured
- ✅ Non-root Docker user
- ✅ Minimal attack surface

### **Database Security**
- ✅ Connection encryption
- ✅ Network isolation
- ✅ Regular backups
- ✅ Access logging

## 📈 Performance Optimization

### **App Configuration**
```toml
# fly.toml optimizations
[vm]
  cpu_kind = "shared"
  cpus = 1
  memory_mb = 512

[scaling]
  min_machines_running = 0
  max_machines_running = 3
```

### **Database Optimization**
```bash
# Monitor database performance
flyctl postgres list -a rust-base-api-db

# Scale database if needed
flyctl postgres update --vm-size dedicated-cpu-1x -a rust-base-api-db
```

## 🚨 Troubleshooting

### **Common Issues**

1. **Build Failures**
   ```bash
   # Check build logs
   flyctl logs --app rust-base-api
   
   # Build locally first
   docker build -t rust-base .
   docker run -p 3000:3000 rust-base
   ```

2. **Database Connection Issues**
   ```bash
   # Verify DATABASE_URL
   flyctl secrets list --app rust-base-api
   
   # Test connection
   flyctl ssh console --app rust-base-api -C "nc -zv db-host 5432"
   ```

3. **Migration Failures**
   ```bash
   # Run migrations manually
   flyctl ssh console --app rust-base-api
   cd /app && sqlx migrate run
   ```

4. **App Not Starting**
   ```bash
   # Check logs for errors
   flyctl logs --app rust-base-api -f
   
   # Verify health check
   curl https://rust-base-api.fly.dev/health
   ```

### **Debug Commands**
```bash
# SSH into running app
flyctl ssh console --app rust-base-api

# Check process status
flyctl ssh console --app rust-base-api -C "ps aux"

# Check disk usage
flyctl ssh console --app rust-base-api -C "df -h"

# Check network connectivity
flyctl ssh console --app rust-base-api -C "netstat -tlnp"
```

## 📝 Additional Resources

- [Fly.io Rust Guide](https://fly.io/docs/languages-and-frameworks/rust/)
- [Fly.io PostgreSQL](https://fly.io/docs/postgres/)
- [Fly.io Networking](https://fly.io/docs/networking/)
- [Fly.io Monitoring](https://fly.io/docs/monitoring/)

---

**🎉 Your Rust Axum API is now ready for production on Fly.io!**

Live API: `https://rust-base-api.fly.dev`
