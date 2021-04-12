-- Add up migration script here
ALTER TABLE tracks ADD COLUMN artist INTEGER REFERENCES artists(id);

