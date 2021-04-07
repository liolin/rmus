-- Add up migration script here
ALTER TABLE tracks ADD COLUMN filePath VARCHAR NOT NULL DEFAULT "";

