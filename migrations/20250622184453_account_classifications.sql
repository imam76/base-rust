-- Enhanced contacts table migration
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Add migration script here
CREATE TABLE IF NOT EXISTS account_classifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    alias_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Essential indexes
CREATE INDEX IF NOT EXISTS idx_account_classifications_code ON account_classifications(code);
CREATE INDEX IF NOT EXISTS idx_account_classifications_name ON account_classifications(name);
CREATE INDEX IF NOT EXISTS idx_account_classifications_alias_name ON account_classifications(alias_name);

-- Auto-update trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_account_classifications_updated_at
    BEFORE UPDATE ON account_classifications
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert initial data
INSERT INTO account_classifications (code, name, alias_name) VALUES
( 1, 'Harta', 'Asset'),
( 2, 'Kewajiban', 'Liabilities'),
( 3, 'Modal', 'Equity'),
( 4, 'Pendapatan', 'Revenues'),
( 5, 'Beban Atas Pendapatan', 'Cost of Revenues'),
( 6, 'Beban Operasional', 'Operating Expenses'),
( 7, 'Beban Non Operasional', 'Non Operating Expenses'),
( 8, 'Pendapatan Lain', 'Other Revenues'),
( 9, 'Beban Lain', 'Other Expenses')
ON CONFLICT (id) DO NOTHING;