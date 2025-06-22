-- Enhanced contacts table migration
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create term_of_payments table
CREATE TABLE IF NOT EXISTS term_of_payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL UNIQUE REFERENCES contacts(id) ON DELETE CASCADE,
    due_days INTEGER DEFAULT 0,
    late_charge NUMERIC(5,2) DEFAULT 0,
    discount_days INTEGER DEFAULT 0,
    early_discount NUMERIC(5,2) DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID NOT NULL REFERENCES users(id),
    
    -- Constraints
    CONSTRAINT term_of_payments_due_days_positive CHECK (due_days >= 0),
    CONSTRAINT term_of_payments_late_charge_positive CHECK (late_charge >= 0),
    CONSTRAINT term_of_payments_discount_days_positive CHECK (discount_days >= 0),
    CONSTRAINT term_of_payments_early_discount_positive CHECK (early_discount >= 0 AND early_discount <= 100),
    CONSTRAINT term_of_payments_discount_logic CHECK (discount_days <= due_days)
);

-- Essential indexes
CREATE INDEX IF NOT EXISTS idx_term_of_payments_contact_id ON term_of_payments(contact_id);
CREATE INDEX IF NOT EXISTS idx_term_of_payments_is_active ON term_of_payments(is_active);

-- Auto-update function for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Auto-update trigger for updated_at
CREATE TRIGGER tr_term_of_payments_updated_at
    BEFORE UPDATE ON term_of_payments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();