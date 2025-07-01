-- Revert migration for users table
-- This will drop the users table and any associated objects

-- Remove table comments
COMMENT ON TABLE users IS NULL;

-- Drop triggers first (they depend on the table)
DROP TRIGGER IF EXISTS trigger_users_updated_at ON users;

-- Drop the trigger function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes (they depend on the table)
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_is_active;

-- Finally, drop the users table
-- WARNING: This will permanently delete all user data!
DROP TABLE IF EXISTS users;

-- Note: This migration will permanently delete all user data
-- Make sure to backup your data before running this migration
