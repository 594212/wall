CREATE TABLE serials (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    serial_count INTEGER NOT NULL DEFAULT 0,
    rating REAL NOT NULL DEFAULT 4,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


INSERT INTO serials (id, title, description) VALUES
( generate_series(1,1000), 'AWESOME FILM ' || trunc(random()*1000), repeat('WOW ',10));
