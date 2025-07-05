#!/bin/bash

# Test the fixed authentication flow
BASE_URL="http://127.0.0.1:5001"

echo "🧪 Testing Authentication Flow"
echo "=============================="

# Step 1: Login
echo "1. Logging in..."
LOGIN_RESPONSE=$(curl -s -c cookies.txt -X POST "$BASE_URL/api/auth" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "yqwhjahsdjhuuushdajshdjh@mailinator.com",
    "password": "123123"
  }')

echo "Login Response:"
echo "$LOGIN_RESPONSE" | jq .

# Step 2: Test protected endpoint (create contact)
echo -e "\n2. Testing protected endpoint (create contact)..."
CONTACT_RESPONSE=$(curl -s -b cookies.txt -X POST "$BASE_URL/api/v1/contacts" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "company": "Test Company",
    "is_customer": true
  }')

echo "Create Contact Response:"
echo "$CONTACT_RESPONSE" | jq .

# Step 3: Test get contacts
echo -e "\n3. Testing get contacts..."
GET_CONTACTS_RESPONSE=$(curl -s -b cookies.txt "$BASE_URL/api/v1/contacts")

echo "Get Contacts Response:"
echo "$GET_CONTACTS_RESPONSE" | jq .

# Cleanup
rm -f cookies.txt

echo -e "\n✅ Authentication flow test completed!"
