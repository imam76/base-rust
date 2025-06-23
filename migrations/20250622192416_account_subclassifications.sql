-- Enhanced contacts table migration
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Add migration script here
CREATE TABLE IF NOT EXISTS account_subclassifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    alias_name VARCHAR(255) NOT NULL,
    cash_flow_type VARCHAR(50) NOT NULL CHECK (cash_flow_type IN ('operating', 'investing', 'financing')),
    ratio_type VARCHAR(50) NOT NULL,
    is_variable_cost BOOLEAN DEFAULT FALSE,
    is_parent BOOLEAN DEFAULT FALSE,
    account_classification_id UUID NOT NULL,
    parent_id UUID NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID NOT NULL REFERENCES users(id),

    -- Constraints
    CONSTRAINT account_subclassifications_code_not_empty CHECK (code <> ''),
    CONSTRAINT account_subclassifications_name_not_empty CHECK (name <> ''),
    CONSTRAINT account_subclassifications_alias_name_not_empty CHECK (alias_name <> ''),
    CONSTRAINT account_subclassifications_cash_flow_type_valid CHECK (cash_flow_type IN ('operating', 'investing', 'financing')),
    CONSTRAINT account_subclassifications_ratio_type_not_empty CHECK (ratio_type <> ''),
    CONSTRAINT account_subclassifications_parent_id_exists FOREIGN KEY (parent_id) REFERENCES account_subclassifications(id) ON DELETE SET NULL
);

-- Essential indexes
CREATE INDEX IF NOT EXISTS idx_account_subclassifications_code ON account_subclassifications(code);
CREATE INDEX IF NOT EXISTS idx_account_subclassifications_name ON account_subclassifications(name);
CREATE INDEX IF NOT EXISTS idx_account_subclassifications_alias_name ON account_subclassifications(alias_name);
CREATE INDEX IF NOT EXISTS idx_account_subclassifications_is_active ON account_subclassifications(is_active);
CREATE INDEX IF NOT EXISTS idx_account_subclassifications_account_classification_id ON account_subclassifications(account_classification_id);
CREATE INDEX IF NOT EXISTS idx_account_subclassifications_parent_id ON account_subclassifications(parent_id); 

-- Auto-update trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_account_subclassifications_updated_at
    BEFORE UPDATE ON account_subclassifications
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();