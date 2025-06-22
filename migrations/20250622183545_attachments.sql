-- Enhanced contacts table migration
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Add migration script here
CREATE TABLE IF NOT EXISTS attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(50) NOT NULL,
    file_size BIGINT NOT NULL,
    file_url TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID NOT NULL REFERENCES users(id),

    -- Constraints
    CONSTRAINT attachments_file_name_not_empty CHECK (file_name <> ''),
    CONSTRAINT attachments_file_type_not_empty CHECK (file_type <> ''),
    CONSTRAINT attachments_file_size_positive CHECK (file_size >= 0)
);

-- Essential indexes
CREATE INDEX IF NOT EXISTS idx_attachments_file_name ON attachments(file_name);
CREATE INDEX IF NOT EXISTS idx_attachments_is_active ON attachments(is_active);

-- Auto-update trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_attachments_updated_at
    BEFORE UPDATE ON attachments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();