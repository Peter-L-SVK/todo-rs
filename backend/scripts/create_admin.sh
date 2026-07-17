#!/bin/bash
# scripts/create_admin.sh
# Create admin user for Todo App

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}=== Todo App - Create Admin User ===${NC}"
echo ""

# Check if running from backend directory
if [ ! -f "todo.db" ]; then
    echo -e "${RED}Error: Database not found.${NC}"
    echo "Please run this script from the backend directory."
    exit 1
fi

# Get admin credentials
read -p "Username [admin]: " USERNAME
USERNAME=${USERNAME:-admin}

read -p "Email [admin@example.com]: " EMAIL
EMAIL=${EMAIL:-admin@example.com}

read -sp "Password [min 8 chars]: " PASSWORD
echo ""

if [ ${#PASSWORD} -lt 8 ]; then
    echo -e "${RED}Error: Password must be at least 8 characters.${NC}"
    exit 1
fi

# Generate password hash
echo -e "${YELLOW}Generating password hash...${NC}"
HASH=$(echo "$PASSWORD" | cargo run --quiet --bin generate_hash 2>/dev/null)

if [ -z "$HASH" ]; then
    echo -e "${RED}Error: Failed to generate hash.${NC}"
    exit 1
fi

# Add role column if missing
if ! sqlite3 todo.db "PRAGMA table_info(users);" | grep -q "role"; then
    echo -e "${YELLOW}Adding role column...${NC}"
    sqlite3 todo.db "ALTER TABLE users ADD COLUMN role TEXT DEFAULT 'user';"
fi

# Check if user exists
if sqlite3 todo.db "SELECT id FROM users WHERE email = '$EMAIL';" | grep -q .; then
    echo -e "${YELLOW}User '$EMAIL' already exists. Updating to admin...${NC}"
    sqlite3 todo.db "UPDATE users SET role = 'admin' WHERE email = '$EMAIL';"
else
    # Generate UUID
    USER_ID=$(uuidgen 2>/dev/null || echo "admin-$(date +%s)")
    
    # Create admin user
    sqlite3 todo.db "INSERT INTO users (id, username, email, password_hash, role, created_at) 
        VALUES ('$USER_ID', '$USERNAME', '$EMAIL', '$HASH', 'admin', datetime('now'));"
fi

echo -e "${GREEN}Admin user created/updated successfully!${NC}"
echo ""
echo -e "${GREEN}=== Admin Details ===${NC}"
echo "  Username: $USERNAME"
echo "  Email:    $EMAIL"
echo ""
echo "You can now login at: http://localhost:5173/admin"
