-- Your SQL goes here
CREATE TYPE comment_type as ENUM('comment', 'episode', 'serial');

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    text TEXT,
    model_id INTEGER NOT NULL,
    model_type COMMENT_TYPE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);