-- Add up migration script here
CREATE VIEW v_tracks (
  trackId,
  title,
  albumId,
  album
)
AS
SELECT
  tracks.id, 
  tracks.title,
  albums.id,
  albums.name
FROM
  tracks
LEFT JOIN albums on tracks.id = albums.id;
