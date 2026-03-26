-- Login logs
CREATE TABLE IF NOT EXISTS login_logs (
    id         TEXT PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    username   TEXT NOT NULL,
    ip         TEXT,
    device     TEXT,
    result     TEXT NOT NULL DEFAULT 'success',   -- success | failed
    detail     TEXT,                                -- failure reason if any
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_login_logs_user_id    ON login_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_login_logs_created_at ON login_logs(created_at);

-- Operation logs
CREATE TABLE IF NOT EXISTS operation_logs (
    id         TEXT PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    username   TEXT NOT NULL,
    module     TEXT NOT NULL,      -- auth, users, profile, license
    action     TEXT NOT NULL,      -- login, logout, create_user, delete_user, ...
    target_id  TEXT,               -- affected resource id (optional)
    detail     TEXT,               -- human-readable detail
    ip         TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_operation_logs_user_id    ON operation_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_operation_logs_module     ON operation_logs(module);
CREATE INDEX IF NOT EXISTS idx_operation_logs_created_at ON operation_logs(created_at);
