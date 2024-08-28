-- Your SQL goes here
CREATE TYPE model_type as ENUM ('serial','episode', 'comment');

CREATE TABLE medias (
    id SERIAL PRIMARY KEY,
    path VARCHAR NOT NULL,
    model_id INTEGER NOT NULL,
    model_type MODEL_TYPE
)