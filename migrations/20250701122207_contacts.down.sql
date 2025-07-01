-- Revert migration for contacts table
-- This will drop the contacts table and any associated objects

-- Remove table comments
COMMENT ON TABLE contacts IS NULL;

-- Drop triggers first (they depend on the table)
DROP TRIGGER IF EXISTS trigger_contacts_updated_at ON contacts;

-- Drop indexes (they depend on the table)
DROP INDEX IF EXISTS idx_contacts_email;
DROP INDEX IF EXISTS idx_contacts_last_name;
DROP INDEX IF EXISTS idx_contacts_company;
DROP INDEX IF EXISTS idx_contacts_city;
DROP INDEX IF EXISTS idx_contacts_billing_city;
DROP INDEX IF EXISTS idx_contacts_delivery_city;
DROP INDEX IF EXISTS idx_contacts_is_customer;
DROP INDEX IF EXISTS idx_contacts_is_employee;
DROP INDEX IF EXISTS idx_contacts_is_supplier;
DROP INDEX IF EXISTS idx_contacts_is_salesman;
DROP INDEX IF EXISTS idx_contacts_is_active;
DROP INDEX IF EXISTS idx_contacts_created_at;

-- Finally, drop the contacts table
-- WARNING: This will permanently delete all contact data!
DROP TABLE IF EXISTS contacts;

-- Note: This migration will permanently delete all contact data
-- Make sure to backup your data before running this migration
