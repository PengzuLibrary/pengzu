-- Your SQL goes here

CREATE TABLE import_libraries
(
    id SERIAL PRIMARY KEY,
    calibre_library_path TEXT NOT NULL,
    library_path TEXT NOT NULL,
    total INTEGER NOT NULL,
    finished BOOLEAN NOT NULL,
    options TEXT NOT NULL,
    worker_pid INTEGER,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)