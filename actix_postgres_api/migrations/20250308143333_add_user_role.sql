-- Add role column to users table
ALTER TABLE users ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'client';

-- Dodanie CHECK constraint dla dozwolonych wartości
ALTER TABLE users ADD CONSTRAINT check_valid_role CHECK (role IN ('client', 'trainer'));

-- Dodanie komentarza dla kolumny role
COMMENT ON COLUMN users.role IS 'Role użytkownika: client (klient siłowni) lub trainer (trener)';

-- Aktualizacja wszystkich istniejących użytkowników, aby mieli rolę client
UPDATE users SET role = 'client' WHERE role IS NULL OR role = '';