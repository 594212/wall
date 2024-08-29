-- Your SQL goes here
CREATE TYPE model_type as ENUM ('serial','episode', 'comment', 'user');
CREATE TYPE collection_type as ENUM ('video', 'avatar');

CREATE TABLE medias (
    id SERIAL PRIMARY KEY,
    uuid uuid NOT NULL DEFAULT gen_random_uuid(),
    model_id  BIGINT NOT NULL,
    model_type MODEL_TYPE NOT NULL ,
    collection_type COLLECTION_TYPE NOT NULL DEFAULT 'video',
    file_name VARCHAR NOT NULL,
    mime_type VARCHAR NOT NULL,
    conversion TEXT NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ind_media_type ON medias (model_id, model_type);
