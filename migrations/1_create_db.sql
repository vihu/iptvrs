CREATE TABLE IF NOT EXISTS iptvrs (
  channel_index INTEGER PRIMARY KEY,
  channel_url TEXT UNIQUE,
  channel_name TEXT,
  playlist_name TEXT
);

CREATE VIRTUAL TABLE IF NOT EXISTS iptvrs_fts USING fts5 (channel_name, playlist_name,);
