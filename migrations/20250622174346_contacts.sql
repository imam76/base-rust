-- Enhanced contacts table migration
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create contacts table
CREATE TABLE IF NOT EXISTS contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    national_id_number VARCHAR(50),
    business_id_number VARCHAR(50),
    tax_id_number VARCHAR(50),
    tax_id_image_url TEXT,
    tax_id_address TEXT,
    is_customer BOOLEAN DEFAULT FALSE,
    is_supplier BOOLEAN DEFAULT FALSE,
    is_employee BOOLEAN DEFAULT FALSE,
    is_salesman BOOLEAN DEFAULT FALSE,
    is_auto_send_email BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    sales_target_amount NUMERIC(15,2) DEFAULT 0,
    classification_id UUID,
    default_currency_id UUID,
    supervisor_id UUID,
    manager_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID NOT NULL,
    
    CONSTRAINT contacts_sales_target_positive CHECK (sales_target_amount >= 0)
);

-- Essential indexes
CREATE INDEX IF NOT EXISTS idx_contacts_name ON contacts(name);
CREATE INDEX IF NOT EXISTS idx_contacts_is_active ON contacts(is_active);
CREATE INDEX IF NOT EXISTS idx_contacts_is_customer ON contacts(is_customer) WHERE is_customer = TRUE;
CREATE INDEX IF NOT EXISTS idx_contacts_is_supplier ON contacts(is_supplier) WHERE is_supplier = TRUE;
CREATE INDEX IF NOT EXISTS idx_contacts_classification_id ON contacts(classification_id);
CREATE INDEX IF NOT EXISTS idx_contacts_created_at ON contacts(created_at);

-- Auto-update trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_contacts_updated_at
    BEFORE UPDATE ON contacts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();