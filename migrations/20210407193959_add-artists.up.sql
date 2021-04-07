-- Add up migration script here
CREATE TABLE artists (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL UNIQUE
);
