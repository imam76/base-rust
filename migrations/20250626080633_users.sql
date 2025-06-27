-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    is_active BOOLEAN DEFAULT true,
    is_admin BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_active ON users(is_active);

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert seed data
-- all password is 123
INSERT INTO users (username, email, password_hash, first_name, last_name, is_admin) VALUES
    ('admin', 'admin@example.com', '$2y$10$1V28ANYexhEmKZ.pe0OdPuSqfcbitRkqtiJ3DiY6P0dSRfxNu8OUC', 'System', 'Administrator', true),
    ('john_doe', 'john.doe@example.com', '$2y$10$1V28ANYexhEmKZ.pe0OdPuSqfcbitRkqtiJ3DiY6P0dSRfxNu8OUC', 'John', 'Doe', false),
    ('jane_smith', 'jane.smith@example.com', '$2y$10$1V28ANYexhEmKZ.pe0OdPuSqfcbitRkqtiJ3DiY6P0dSRfxNu8OUC', 'Jane', 'Smith', false),
    ('bob_wilson', 'bob.wilson@example.com', '$2y$10$1V28ANYexhEmKZ.pe0OdPuSqfcbitRkqtiJ3DiY6P0dSRfxNu8OUC', 'Bob', 'Wilson', false),
    ('alice_brown', 'alice.brown@example.com', '$2y$10$1V28ANYexhEmKZ.pe0OdPuSqfcbitRkqtiJ3DiY6P0dSRfxNu8OUC', 'Alice', 'Brown', false)
ON CONFLICT (username) DO NOTHING;