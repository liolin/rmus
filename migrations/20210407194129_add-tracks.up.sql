-- Add up migration script here
CREATE TABLE tracks (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  album INTEGER NOT NULL,

  FOREIGN KEY(album) REFERENCES album(id)
);
