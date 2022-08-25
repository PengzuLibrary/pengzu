-- Your SQL goes here

CREATE TABLE reading_history
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book INTEGER NOT NULL,
    page INTEGER NOT NULL DEFAULT 0,
    percent INTEGER NOT NULL DEFAULT 0,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, user_id)
)