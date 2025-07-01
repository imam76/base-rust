-- Create users table with comprehensive fields
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_is_active ON users(is_active);

-- Create a trigger to automatically update the updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER trigger_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample data for testing (optional)
INSERT INTO users (username, email, password_hash, first_name, last_name, is_verified) VALUES
('admin', 'yqwhjahsdjhuuushdajshdjh@mailinator.com', '$2a$12$sYofX.hqDW0gUiGZXbSj0.SzQ4heFwPpWCP0KoFzMrNW2rtfCYjQm', 'Admin', 'User', true),
('testuser', 'yqwhjahsdjhuuushdajshdjh@example.com', '$2a$12$sYofX.hqDW0gUiGZXbSj0.SzQ4heFwPpWCP0KoFzMrNW2rtfCYjQm', 'Test', 'User', false);

-- Add comments for documentation
COMMENT ON TABLE users IS 'User accounts table';
COMMENT ON COLUMN users.id IS 'Unique identifier for each user';
COMMENT ON COLUMN users.username IS 'Unique username for login';
COMMENT ON COLUMN users.email IS 'Unique email address for login and communication';
COMMENT ON COLUMN users.password_hash IS 'Bcrypt hashed password';
COMMENT ON COLUMN users.is_active IS 'Whether the user account is active';
COMMENT ON COLUMN users.is_verified IS 'Whether the user email has been verified';
COMMENT ON COLUMN users.last_login_at IS 'Timestamp of last successful login';
COMMENT ON COLUMN users.created_at IS 'When the user account was created';
COMMENT ON COLUMN users.updated_at IS 'When the user account was last updated';
