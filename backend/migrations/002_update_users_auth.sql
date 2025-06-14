-- Update users table for authentication system
-- Add new columns and update existing ones

-- First, add the new columns
ALTER TABLE users 
ADD COLUMN password_hash VARCHAR(255),
ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE;

-- Update existing users to have a placeholder password hash
-- (This is safe since there shouldn't be any real users yet)
UPDATE users SET password_hash = '$2b$10$placeholder' WHERE password_hash IS NULL;

-- Make password_hash required
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;

-- Remove the old password column if it exists
ALTER TABLE users DROP COLUMN IF EXISTS password;

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Create index on is_admin for admin queries
CREATE INDEX IF NOT EXISTS idx_users_is_admin ON users(is_admin);
