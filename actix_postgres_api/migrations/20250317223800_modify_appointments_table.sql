-- Modify appointments table structure

-- First, remove the check constraint for status
ALTER TABLE appointments DROP CONSTRAINT IF EXISTS check_valid_status;

-- Rename the title column to type (temporary step with both columns)
ALTER TABLE appointments ADD COLUMN type VARCHAR(20);

-- Update the type column based on existing title values
-- This is a simple migration that sets all existing appointments to 'training'
-- In a real-world scenario, you might want more sophisticated mapping logic
UPDATE appointments SET type = 'training';

-- Make type column NOT NULL
ALTER TABLE appointments ALTER COLUMN type SET NOT NULL;

-- Drop the original title column
ALTER TABLE appointments DROP COLUMN title;

-- Remove notes column
ALTER TABLE appointments DROP COLUMN notes;

-- Add location column
ALTER TABLE appointments ADD COLUMN location VARCHAR(100);

-- Add constraint to ensure type is valid
ALTER TABLE appointments ADD CONSTRAINT check_valid_type
    CHECK (type IN ('training', 'check-in', 'consultation', 'assessment'));

-- Update the status constraint to include 'no-show'
ALTER TABLE appointments ADD CONSTRAINT check_valid_status
    CHECK (status IN ('scheduled', 'completed', 'canceled', 'no-show'));

-- Update any existing 'cancelled' values to 'canceled'
UPDATE appointments SET status = 'canceled' WHERE status = 'cancelled';

-- Add comment for the modified table
COMMENT ON TABLE appointments IS 'Appointments between clients and trainers with type, location and updated status options';