-- Your SQL goes here
CREATE TYPE comment_type as ENUM('comment', 'episode', 'serial');

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    text TEXT,
    model_id INTEGER NOT NULL,
    model_type COMMENT_TYPE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ind_comment_type on comments (model_id, model_type);
