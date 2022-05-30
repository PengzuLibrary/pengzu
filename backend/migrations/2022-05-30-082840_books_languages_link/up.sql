-- Your SQL goes here

-- CREATE TABLE books_languages_link (
--     id INTEGER PRIMARY KEY,
--     book INTEGER NOT NULL,
--     lang_code INTEGER NOT NULL,
--     item_order INTEGER NOT NULL DEFAULT 0,
--     UNIQUE (book, lang_code)
-- )

CREATE TABLE books_languages_link (
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    lang_code INTEGER NOT NULL,
    UNIQUE (book, lang_code)
)