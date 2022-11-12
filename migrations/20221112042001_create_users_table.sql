-- Add migration script here
CREATE TABLE Users
(
    id            TEXT NOT NULL PRIMARY KEY,
    email         TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    token         TEXT,
    UNIQUE (email)
)