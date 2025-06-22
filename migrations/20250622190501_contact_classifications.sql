-- Enhanced contacts table migration
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts_classification (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    alias_name VARCHAR(50) NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID NOT NULL REFERENCES users(id),

    -- Constraints
    CONSTRAINT contacts_classification_code_not_empty CHECK (code <> ''),
    CONSTRAINT contacts_classification_name_not_empty CHECK (name <> ''),
    CONSTRAINT contacts_classification_alias_name_not_empty CHECK (alias_name <> '')
);

-- Essential indexes
CREATE INDEX IF NOT EXISTS idx_contacts_classification_code ON contacts_classification(code);
CREATE INDEX IF NOT EXISTS idx_contacts_classification_name ON contacts_classification(name);
CREATE INDEX IF NOT EXISTS idx_contacts_classification_is_active ON contacts_classification(is_active);

-- Auto-update trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_contacts_classification_updated_at
    BEFORE UPDATE ON contacts_classification
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();