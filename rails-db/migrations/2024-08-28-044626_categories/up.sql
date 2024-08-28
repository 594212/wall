-- Your SQL goes here
CREATE TYPE category_type as ENUM('tag', 'genre', 'author', 'year', 'status');

CREATE TABLE categories(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    category_type CATEGORY_TYPE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);