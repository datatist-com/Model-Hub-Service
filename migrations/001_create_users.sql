-- Users table (SQLite)
CREATE TABLE IF NOT EXISTS users (
    id            TEXT NOT NULL PRIMARY KEY,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    real_name     TEXT,
    role          TEXT NOT NULL DEFAULT 'model_developer',
    status        TEXT NOT NULL DEFAULT 'active',
    language      TEXT,
    ui_theme      TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
