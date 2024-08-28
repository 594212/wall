-- Your SQL goes here
CREATE TABLE likes (
    id SERIAL PRIMARY KEY,
    episode_id INTEGER NOT NULL REFERENCES episodes(id),
    user_id INTEGER NOT NULL REFERENCES users(id)
);