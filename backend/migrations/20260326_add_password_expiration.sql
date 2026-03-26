-- Add password expiration columns to sys_user table
ALTER TABLE sys_user 
ADD COLUMN IF NOT EXISTS last_password_changed_at TIMESTAMP WITH TIME ZONE,
ADD COLUMN IF NOT EXISTS password_expires_at TIMESTAMP WITH TIME ZONE,
ADD COLUMN IF NOT EXISTS password_expiry_days INTEGER DEFAULT 90;

-- Set default last_password_changed_at for existing users
UPDATE sys_user 
SET last_password_changed_at = created_at 
WHERE last_password_changed_at IS NULL;

-- Set default password expiry for existing users (90 days from last change)
UPDATE sys_user 
SET password_expires_at = last_password_changed_at + INTERVAL '90 days'
WHERE password_expires_at IS NULL AND last_password_changed_at IS NOT NULL;

-- Create index for password expiration checks
CREATE INDEX IF NOT EXISTS idx_user_password_expires ON sys_user(password_expires_at);

-- Add comment for documentation
COMMENT ON COLUMN sys_user.last_password_changed_at IS 'Timestamp when the user last changed their password';
COMMENT ON COLUMN sys_user.password_expires_at IS 'Timestamp when the password will expire';
COMMENT ON COLUMN sys_user.password_expiry_days IS 'Number of days after which password expires (default: 90)';
