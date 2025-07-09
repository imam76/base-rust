-- Create contacts table with comprehensive fields
CREATE TABLE contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    mobile VARCHAR(50),
    company VARCHAR(255),
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(100) DEFAULT 'United States',
    billing_address_line1 VARCHAR(255),
    billing_address_line2 VARCHAR(255),
    billing_city VARCHAR(100),
    billing_state VARCHAR(100),
    billing_postal_code VARCHAR(20),
    billing_country VARCHAR(100),
    delivery_address_line1 VARCHAR(255),
    delivery_address_line2 VARCHAR(255),
    delivery_city VARCHAR(100),
    delivery_state VARCHAR(100),
    delivery_postal_code VARCHAR(20),
    delivery_country VARCHAR(100),
    is_customer BOOLEAN NOT NULL DEFAULT false,
    is_employee BOOLEAN NOT NULL DEFAULT false,
    is_supplier BOOLEAN NOT NULL DEFAULT false,
    is_salesman BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID,
    
    -- Foreign key constraints
    CONSTRAINT fk_contacts_created_by FOREIGN KEY (created_by) REFERENCES users(id),
    CONSTRAINT fk_contacts_updated_by FOREIGN KEY (updated_by) REFERENCES users(id)
);

-- Create indexes for better performance
CREATE INDEX idx_contacts_code ON contacts(code);
CREATE INDEX idx_contacts_email ON contacts(email);
CREATE INDEX idx_contacts_last_name ON contacts(last_name);
CREATE INDEX idx_contacts_company ON contacts(company);
CREATE INDEX idx_contacts_city ON contacts(city);
CREATE INDEX idx_contacts_billing_city ON contacts(billing_city);
CREATE INDEX idx_contacts_delivery_city ON contacts(delivery_city);
CREATE INDEX idx_contacts_is_customer ON contacts(is_customer);
CREATE INDEX idx_contacts_is_employee ON contacts(is_employee);
CREATE INDEX idx_contacts_is_supplier ON contacts(is_supplier);
CREATE INDEX idx_contacts_is_salesman ON contacts(is_salesman);
CREATE INDEX idx_contacts_is_active ON contacts(is_active);
CREATE INDEX idx_contacts_created_at ON contacts(created_at);

-- Create a trigger to automatically update the updated_at column
CREATE TRIGGER trigger_contacts_updated_at
    BEFORE UPDATE ON contacts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample data for testing (optional)
INSERT INTO contacts (code, first_name, last_name, email, phone, company, is_employee, is_salesman) VALUES
('D-00001', 'John', 'Doe', 'john.doe@company.com', '555-0101', 'ABC Corp', true, true),
('S-00001', 'Jane', 'Smith', 'jane.smith@supplier.com', '555-0102', 'Supplier Inc', false, false);

-- Add comments for documentation
COMMENT ON TABLE contacts IS 'Contact management table with role flags';
COMMENT ON COLUMN contacts.code IS 'Unique code for the contact';
COMMENT ON COLUMN contacts.id IS 'Unique identifier for each contact';
COMMENT ON COLUMN contacts.first_name IS 'Contact first name';
COMMENT ON COLUMN contacts.last_name IS 'Contact last name';
COMMENT ON COLUMN contacts.email IS 'Contact email address';
COMMENT ON COLUMN contacts.address_line1 IS 'Primary address line 1';
COMMENT ON COLUMN contacts.billing_address_line1 IS 'Billing address line 1';
COMMENT ON COLUMN contacts.delivery_address_line1 IS 'Delivery address line 1';
COMMENT ON COLUMN contacts.is_customer IS 'Whether contact is a customer';
COMMENT ON COLUMN contacts.is_employee IS 'Whether contact is an employee';
COMMENT ON COLUMN contacts.is_supplier IS 'Whether contact is a supplier';
COMMENT ON COLUMN contacts.is_salesman IS 'Whether contact is a salesman';
COMMENT ON COLUMN contacts.is_active IS 'Whether the contact is active';
COMMENT ON COLUMN contacts.created_at IS 'When the contact was created';
COMMENT ON COLUMN contacts.updated_at IS 'When the contact was last updated';
