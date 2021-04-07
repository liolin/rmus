-- Add down migration script here
DROP VIEW v_tracks;
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
