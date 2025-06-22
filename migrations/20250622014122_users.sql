-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- add example data
-- pass 123123
INSERT INTO users (username, email, password_hash) VALUES
('zombie', 'qwejicmkkkskasdkajsdo0qwiasdjk@mailinator.com', '$2y$10$7rEvi09JyTFknoVr/OEU6usYNIIOrojBUPBpm5BS7dvn4MLyMIPDi')
ON CONFLICT (username) DO NOTHING;