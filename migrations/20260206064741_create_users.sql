-- Enable uuid generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =========================
-- users (main table)
-- =========================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- =========================
-- user_names (i18n join table)
-- =========================
CREATE TABLE IF NOT EXISTS user_names (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lang VARCHAR(5) NOT NULL,              -- 'th', 'en', 'jp', etc.
    first_name VARCHAR(255) NOT NULL,
    middle_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id, lang)
);

-- Index for fast lookup
CREATE INDEX IF NOT EXISTS idx_user_names_lang
    ON user_names (lang);

CREATE INDEX IF NOT EXISTS idx_user_names_lang_first
    ON user_names (lang, first_name);
