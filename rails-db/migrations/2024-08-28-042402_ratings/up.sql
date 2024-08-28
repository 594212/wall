-- Your SQL goes here
CREATE TABLE ratings (
    number SMALLINT CHECK (number BETWEEN 0 AND 5),
    user_id INTEGER REFERENCES users(id),
    serial_id INTEGER REFERENCES serials(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(user_id, serial_id)
)