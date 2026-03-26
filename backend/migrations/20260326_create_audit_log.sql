-- Create audit log table for security events
CREATE TABLE IF NOT EXISTS sys_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    username VARCHAR(100),
    event_type VARCHAR(50) NOT NULL,
    ip_address VARCHAR(50) NOT NULL,
    user_agent TEXT,
    details JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_audit_log_user_id ON sys_audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_created_at ON sys_audit_log(created_at);
CREATE INDEX IF NOT EXISTS idx_audit_log_event_type ON sys_audit_log(event_type);

-- Add comment for documentation
COMMENT ON TABLE sys_audit_log IS 'Audit log table for security events tracking';
COMMENT ON COLUMN sys_audit_log.id IS 'Unique identifier for the audit log entry';
COMMENT ON COLUMN sys_audit_log.user_id IS 'Reference to the user who performed the action';
COMMENT ON COLUMN sys_audit_log.username IS 'Username at the time of the event';
COMMENT ON COLUMN sys_audit_log.event_type IS 'Type of security event (login_attempt, login_success, login_failure, password_change, permission_denied, etc.)';
COMMENT ON COLUMN sys_audit_log.ip_address IS 'IP address of the client';
COMMENT ON COLUMN sys_audit_log.user_agent IS 'User agent string from the request';
COMMENT ON COLUMN sys_audit_log.details IS 'Additional event details in JSON format';
COMMENT ON COLUMN sys_audit_log.created_at IS 'Timestamp when the event occurred';
