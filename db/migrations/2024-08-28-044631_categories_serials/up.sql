-- Your SQL goes here
CREATE TABLE categories_serials(
    category_id INTEGER REFERENCES categories(id),
    serial_id INTEGER REFERENCES serials(id),
    PRIMARY KEY(category_id, serial_id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)