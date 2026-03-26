-- Additional indexes for query performance
CREATE INDEX IF NOT EXISTS idx_users_username       ON users(username);
CREATE INDEX IF NOT EXISTS idx_login_logs_user_created ON login_logs(user_id, created_at);
CREATE INDEX IF NOT EXISTS idx_operation_logs_user_created ON operation_logs(user_id, created_at);
