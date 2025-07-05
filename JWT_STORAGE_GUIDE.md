# 🔐 JWT Token Storage Strategies

## **1. Cookie-Only (Recommended untuk Web Apps) 🍪**

### **Cara Kerja:**
Server otomatis set HTTP-only cookie, client tidak perlu menyimpan token manual.

### **Login Request:**
```javascript
// Web App - Cookie Strategy
fetch('/api/auth', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ 
    email: 'user@example.com', 
    password: 'password',
    use_cookie: true  // Optional, default true
  })
});
```

### **Subsequent Requests:**
```javascript
// Cookie otomatis dikirim browser
fetch('/api/v1/contacts'); // No token handling needed!
```

### **Kelebihan:**
- ✅ **Paling aman** - HTTP-only cookie tidak bisa diakses JavaScript
- ✅ **Otomatis** - browser handle cookie
- ✅ **Zero maintenance** - tidak perlu store/manage token
- ✅ **XSS Protection** - token tidak exposed ke JavaScript

### **Kekurangan:**
- ❌ **CSRF vulnerable** - butuh CSRF protection
- ❌ **Same-origin only** - tidak bisa cross-domain request
- ❌ **Mobile apps sulit** - native mobile apps tidak support cookie

---

## **2. Manual Token Storage (untuk API Clients) 📱**

### **Cara Kerja:**
Client simpan token dari response, kirim manual via Authorization header.

### **Login Request:**
```javascript
// API Client - Manual Token Strategy
const response = await fetch('/api/auth', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ 
    email: 'user@example.com', 
    password: 'password',
    use_cookie: false  // Don't set cookie
  })
});

const { token } = await response.json();
// Store token (localStorage, sessionStorage, secure storage)
localStorage.setItem('auth_token', token);
```

### **Subsequent Requests:**
```javascript
// Get token from storage and send via header
const token = localStorage.getItem('auth_token');

fetch('/api/v1/contacts', {
  headers: {
    'Authorization': `Bearer ${token}`
  }
});
```

### **Storage Options:**

#### **A. localStorage (Persistent)**
```javascript
// Store
localStorage.setItem('auth_token', token);
// Retrieve
const token = localStorage.getItem('auth_token');
// Remove
localStorage.removeItem('auth_token');
```

#### **B. sessionStorage (Session Only)**
```javascript
// Store
sessionStorage.setItem('auth_token', token);
// Retrieve  
const token = sessionStorage.getItem('auth_token');
```

#### **C. Memory Only (Most Secure)**
```javascript
// Store in variable/state
let authToken = token;
// Lost when page refresh - most secure but less convenient
```

### **Kelebihan:**
- ✅ **Flexible** - bisa cross-domain
- ✅ **Mobile friendly** - cocok untuk native apps
- ✅ **No CSRF issues** - token di header aman dari CSRF
- ✅ **Fine control** - bisa control kapan send token

### **Kekurangan:**
- ❌ **XSS vulnerable** - JavaScript bisa akses token
- ❌ **Manual management** - harus handle store/retrieve/refresh
- ❌ **Storage complexity** - perlu pilih storage yang tepat

---

## **3. Hybrid Approach (Best of Both) 🎯**

Implementasi saat ini mendukung **kedua strategy**:

### **Web App (Cookie-based):**
```javascript
// Login - cookie otomatis di-set
fetch('/api/auth', {
  method: 'POST',
  body: JSON.stringify({ email: 'user@example.com', password: 'pass' })
});

// Requests - cookie otomatis dikirim
fetch('/api/v1/contacts'); // Works automatically!
```

### **Mobile/API Client (Token-based):**
```javascript
// Login - get token from response
const response = await fetch('/api/auth', {
  method: 'POST',
  body: JSON.stringify({ 
    email: 'user@example.com', 
    password: 'pass',
    use_cookie: false 
  })
});

const { token } = await response.json();

// Requests - manual Authorization header
fetch('/api/v1/contacts', {
  headers: { 'Authorization': `Bearer ${token}` }
});
```

---

## **🏆 Rekomendasi Berdasarkan Use Case:**

### **Web Application (React, Vue, Angular):**
```
🍪 Cookie-Only Strategy
✅ Paling aman dan mudah
✅ Tidak perlu code tambahan
✅ HTTP-only protection
```

### **Mobile App (React Native, Flutter):**
```
📱 Manual Token Strategy  
✅ Store in secure storage
✅ Send via Authorization header
✅ Handle refresh token logic
```

### **SPA with API calls to different domains:**
```
📱 Manual Token Strategy
✅ localStorage/sessionStorage
✅ Authorization header
✅ Cross-domain support
```

### **Server-to-Server API:**
```
📱 Manual Token Strategy
✅ Environment variables
✅ Authorization header
✅ Service account tokens
```

---

## **🔒 Security Best Practices:**

### **Cookie Strategy:**
- ✅ HTTP-only cookies
- ✅ Secure flag (HTTPS only)
- ✅ SameSite attribute
- ✅ Short expiration time
- ✅ CSRF protection

### **Manual Token Strategy:**
- ✅ Secure storage (not localStorage untuk sensitive apps)
- ✅ Token refresh mechanism
- ✅ Clear token on logout
- ✅ Validate token on app start
- ✅ Handle token expiration gracefully

---

## **💡 Kesimpulan:**

**Untuk aplikasi web tradisional:** Gunakan **cookie-only** - paling aman dan mudah.

**Untuk mobile apps/API clients:** Gunakan **manual token** dengan secure storage.

**Current implementation mendukung kedua strategi**, jadi Anda bisa pilih sesuai kebutuhan!
