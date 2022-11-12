-- Add migration script here
CREATE TABLE IF NOT EXISTS Items
(
    id              INTEGER PRIMARY KEY,
    user_id         TEXT    NOT NULL,
    subscription_id TEXT    NOT NULL,
    external_id     TEXT    NOT NULL,
    title           TEXT    NOT NULL,
    content         TEXT    NOT NULL,
    author          TEXT    NOT NULL,
    url             TEXT    NOT NULL,
    created_at_ms   INTEGER NOT NULL,
    fetched_at_ms   INTEGER NOT NULL,
    starred         BOOL    NOT NULL,
    read            BOOL    NOT NULL,
    UNIQUE (user_id, subscription_id, external_id)
)