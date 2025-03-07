-- Add phone_number column to users table
ALTER TABLE users ADD COLUMN phone_number VARCHAR(20);

-- Update schema to make full_name NOT NULL
-- First, update existing records with NULL full_name (if any)
UPDATE users SET full_name = 'Unknown' WHERE full_name IS NULL;

-- Then alter the column to be NOT NULL
ALTER TABLE users ALTER COLUMN full_name SET NOT NULL;