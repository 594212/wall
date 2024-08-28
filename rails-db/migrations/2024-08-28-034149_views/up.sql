-- Your SQL goes here
CREATE TABLE views(
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  episode_id INTEGER NOT NULL REFERENCES episodes(id)
);