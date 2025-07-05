#!/bin/bash

# Test auth endpoint with invalid credentials
echo "Testing auth endpoint with invalid email..."

curl -X POST http://127.0.0.1:5001/api/auth \
  -H "Content-Type: application/json" \
  -d '{
    "email": "nonexistent@example.com",
    "password": "wrongpassword"
  }' \
  | jq .

echo -e "\n\nTesting auth endpoint with valid email but wrong password..."

curl -X POST http://127.0.0.1:5001/api/auth \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "wrongpassword"
  }' \
  | jq .

echo -e "\n\nBoth requests should return: 'Invalid email or password.'"
