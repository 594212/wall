-- Your SQL goes here
CREATE TABLE scrapers_serials (
    scraper_id INTEGER NOT NULL REFERENCES scrapers(id),
    serial_id INTEGER NOT NULL REFERENCES serials(id),
    PRIMARY KEY(scraper_id, serial_id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ind_scrapper_serial ON scrapers_serials(serial_id, scraper_id);

