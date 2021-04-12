-- Add down migration script here
DROP VIEW v_tracks;
CREATE VIEW v_tracks (
  trackId,
  title,
  albumId,
  album,
  filePath
)
AS
SELECT
  tracks.id, 
  tracks.title,
  albums.id,
  albums.name,
  tracks.filePath
FROM
  tracks
LEFT JOIN albums on tracks.album = albums.id;
