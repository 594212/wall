-- Your SQL goes here
CREATE TABLE ratings (
    id SERIAL PRIMARY KEY,
    number SMALLINT CHECK (number BETWEEN 0 AND 5),
    user_id INTEGER NOT NULL REFERENCES users(id),
    serial_id INTEGER NOT NULL REFERENCES serials(id),
    avrg FLOAT4 CHECK (avrg BETWEEN 0 AND 5)
)