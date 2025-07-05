#!/bin/bash

# Test JWT Authentication Flow
BASE_URL="http://127.0.0.1:5001"

echo "🔐 Testing JWT Authentication Flow"
echo "=================================="

# Step 1: Login to get JWT token
echo "1. Login dengan JWT..."
LOGIN_RESPONSE=$(curl -s -c cookies.txt -X POST "$BASE_URL/api/auth" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "yqwhjahsdjhuuushdajshdjh@mailinator.com",
    "password": "123123"
  }')

echo "Login Response:"
echo "$LOGIN_RESPONSE" | jq .

# Extract JWT token from response
JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
echo -e "\nExtracted JWT Token: $JWT_TOKEN"

# Step 2: Test dengan cookie (normal flow)
echo -e "\n2. Test create contact dengan cookie..."
CONTACT_RESPONSE=$(curl -s -b cookies.txt -X POST "$BASE_URL/api/v1/contacts" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "company": "Test Company JWT",
    "is_customer": true
  }')

echo "Create Contact Response (Cookie):"
echo "$CONTACT_RESPONSE" | jq .

# Step 3: Test dengan Authorization header (API client flow)
echo -e "\n3. Test create contact dengan Bearer token..."
CONTACT_RESPONSE2=$(curl -s -X POST "$BASE_URL/api/v1/contacts" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -d '{
    "first_name": "Jane",
    "last_name": "Smith",
    "email": "jane.smith@example.com",
    "company": "Test Company Bearer",
    "is_supplier": true
  }')

echo "Create Contact Response (Bearer):"
echo "$CONTACT_RESPONSE2" | jq .

# Step 4: Test refresh token
echo -e "\n4. Test refresh token..."
REFRESH_RESPONSE=$(curl -s -b cookies.txt -X POST "$BASE_URL/api/auth/refresh")

echo "Refresh Token Response:"
echo "$REFRESH_RESPONSE" | jq .

# Step 5: Test logout
echo -e "\n5. Test logout..."
LOGOUT_RESPONSE=$(curl -s -b cookies.txt -X POST "$BASE_URL/api/auth/logout")

echo "Logout Response:"
echo "$LOGOUT_RESPONSE" | jq .

# Cleanup
rm -f cookies.txt

echo -e "\n✅ JWT Authentication flow test completed!"
