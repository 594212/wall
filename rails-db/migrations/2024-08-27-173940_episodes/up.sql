CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    number INT NOT NULL,
    name VARCHAR,
    serial_id INTEGER NOT NULL REFERENCES serials(id)
);