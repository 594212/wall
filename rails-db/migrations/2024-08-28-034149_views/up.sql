-- Your SQL goes here
CREATE TABLE views(
  user_id INTEGER REFERENCES users(id),
  episode_id INTEGER REFERENCES episodes(id),
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(user_id, episode_id)
);