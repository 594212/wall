-- Your SQL goes here
CREATE TYPE category_type as ENUM('tag', 'genre', 'author', 'year', 'status');

CREATE TABLE categories(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    c_type CATEGORY_TYPE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);