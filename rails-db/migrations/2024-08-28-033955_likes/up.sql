-- Your SQL goes here
CREATE TABLE likes (
    user_id INTEGER NOT NULL REFERENCES users(id),
    episode_id INTEGER NOT NULL REFERENCES episodes(id),
    PRIMARY KEY(user_id, episode_id)
);