-- Session tokens table (SQLite)
CREATE TABLE IF NOT EXISTS tokens (
    id         TEXT NOT NULL PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token      TEXT NOT NULL UNIQUE,
    ip         TEXT,
    device     TEXT,
    status     TEXT NOT NULL DEFAULT 'active',   -- active | revoked
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tokens_token   ON tokens(token);
CREATE INDEX IF NOT EXISTS idx_tokens_user_id ON tokens(user_id);
