-- Add migration script here
CREATE TABLE IF NOT EXISTS Subscriptions
(
    user_id           TEXT    NOT NULL,
    id                TEXT    NOT NULL,
    url               TEXT    NOT NULL,
    title             TEXT    NOT NULL,
    description       TEXT    NOT NULL,
    feed_url          TEXT    NOT NULL,
    joined_categories TEXT,
    last_fetch_ms     INTEGER NOT NULL,
    PRIMARY KEY (user_id, id)
)