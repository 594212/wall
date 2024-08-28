-- Your SQL goes here
CREATE TABLE likes (
    user_id INTEGER REFERENCES users(id),
    episode_id INTEGER REFERENCES episodes(id),
    PRIMARY KEY(user_id, episode_id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);