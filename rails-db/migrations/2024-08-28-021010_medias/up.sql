-- Your SQL goes here
CREATE TYPE model_type as ENUM ('serial','episode', 'comment', 'user');

CREATE TABLE medias (
    id SERIAL PRIMARY KEY,
    uuid uuid DEFAULT gen_random_uuid(),
    model_id  BIGINT NOT NULL,
    model_type MODEL_TYPE,
    file_name VARCHAR NOT NULL,
    mime_type VARCHAR NOT NULL,
    conversion TEXT NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)