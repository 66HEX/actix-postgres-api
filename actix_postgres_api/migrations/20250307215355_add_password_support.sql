-- Add password_hash column to users table
ALTER TABLE users ADD COLUMN password_hash VARCHAR(255) NOT NULL DEFAULT '$2a$12$k8Y6Nt5zfQXmGO9VvQH2CehxfMY0lPuqJxzAkrxoHSJRZz8obzg4W';
-- Default hash is for 'ChangeMe123' - should only be temporary for existing users

-- Comment: The default hash is only provided to allow migration of existing records.
-- In a production environment, users with default passwords should be required to change it on next login.sqlx migrate run
