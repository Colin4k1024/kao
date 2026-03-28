-- Add account lockout fields to sys_user table
-- This migration adds failed_login_attempts, locked_until, and lockout_reason columns

ALTER TABLE sys_user
ADD COLUMN IF NOT EXISTS failed_login_attempts INTEGER DEFAULT 0 NOT NULL;

ALTER TABLE sys_user
ADD COLUMN IF NOT EXISTS locked_until TIMESTAMP WITH TIME ZONE;

ALTER TABLE sys_user
ADD COLUMN IF NOT EXISTS lockout_reason TEXT;

-- Create index for faster lockout checks
CREATE INDEX IF NOT EXISTS idx_user_locked_until ON sys_user(locked_until);

-- Create index for faster failed login attempts lookup
CREATE INDEX IF NOT EXISTS idx_user_failed_login ON sys_user(failed_login_attempts);
