-- License activation records (SQLite)
CREATE TABLE IF NOT EXISTS licenses (
    id           TEXT NOT NULL PRIMARY KEY,
    token        TEXT NOT NULL,                        -- raw license token string
    project_name TEXT NOT NULL,                        -- decoded licensee name
    expires_at   TEXT NOT NULL,                        -- ISO-8601 UTC
    status       TEXT NOT NULL DEFAULT 'active',       -- active | expired | replaced
    activated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_licenses_status ON licenses(status);
