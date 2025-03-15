-- Add admin role to the allowed values in the check_valid_role constraint
ALTER TABLE users DROP CONSTRAINT check_valid_role;

-- Add new constraint with admin role included
ALTER TABLE users ADD CONSTRAINT check_valid_role CHECK (role IN ('client', 'trainer', 'admin'));

-- Update the comment for the role column to include admin role
COMMENT ON COLUMN users.role IS 'Role użytkownika: client (klient siłowni), trainer (trener) lub admin (administrator)';