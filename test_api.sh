#!/bin/bash

# API Test Script for Rust Base Backend
# This script demonstrates the main API endpoints and functionality

BASE_URL="http://127.0.0.1:5001"

echo "🧪 Testing Rust Base Backend API"
echo "================================="

# Test health endpoint
echo "1. Testing health endpoint..."
curl -s "$BASE_URL/health" | jq .

echo -e "\n2. Testing version endpoint..."
curl -s "$BASE_URL/version" | jq .

echo -e "\n3. Testing root endpoint..."
curl -s "$BASE_URL/" | jq .

# Test users endpoints
echo -e "\n4. Testing users list (with pagination)..."
curl -s "$BASE_URL/users?page=1&perPage=10" | jq .

echo -e "\n5. Testing users search..."
curl -s "$BASE_URL/users?search=admin" | jq .

echo -e "\n6. Testing users with filters..."
curl -s "$BASE_URL/users?filter={\"status\":\"active\"}" | jq .

echo -e "\n7. Testing users with sorting..."
curl -s "$BASE_URL/users?sortBy=email&sortOrder=asc" | jq .

# Test contacts endpoints
echo -e "\n8. Testing contacts list..."
curl -s "$BASE_URL/contacts?page=1&perPage=5" | jq .

echo -e "\n9. Testing contacts search..."
curl -s "$BASE_URL/contacts?search=company" | jq .

echo -e "\n10. Testing contacts with filters..."
curl -s "$BASE_URL/contacts?filter={\"type\":\"customer\"}" | jq .

echo -e "\n✅ API tests completed!"
echo -e "\nNote: Some endpoints may return empty results if no data exists in the database."
echo "To test CRUD operations (create, update, delete), you'll need authentication tokens."
