-- Your SQL goes here
CREATE TABLE views(
  user_id INTEGER REFERENCES users(id),
  episode_id INTEGER REFERENCES episodes(id),
  PRIMARY KEY(user_id, episode_id)
);