use anyhow::Result;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Album {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub album: Album,
}

impl Artist {
    pub async fn insert_into_db(artist: &str, pool: &SqlitePool) -> Result<Artist> {
        let queryed_artist = sqlx::query_as!(
            Artist,
            "SELECT id, name FROM artists WHERE name = ?;",
            artist
        )
        .fetch_optional(pool)
        .await?;

        let a = match queryed_artist {
            Some(a) => {
                info!("Artist {} in Database", artist);
                a
            }
            None => {
                info!("Artist {} not in Database. Create it", artist);
                sqlx::query_as!(
		Artist,
		"INSERT INTO artists (name) VALUES (?); SELECT id, name FROM artists WHERE name = ?",
		artist,
		artist
	    )
                .fetch_one(pool)
                .await?
            }
        };

        return Ok(a);
    }
}

impl Album {
    pub async fn insert_into_db(album: &str, pool: &SqlitePool) -> Result<Album> {
        let queryed_album =
            sqlx::query_as!(Album, "SELECT id, name FROM albums WHERE name = ?;", album)
                .fetch_optional(pool)
                .await?;

        let a = match queryed_album {
            Some(a) => {
                info!("Album {} in Database", album);
                a
            }
            None => {
                info!("Album {} not in Database. Create it", album);
                sqlx::query_as!(
                Album,
                "INSERT INTO albums (name) VALUES (?); SELECT id, name FROM albums WHERE name = ?",
                album,
                album
            )
                .fetch_one(pool)
                .await?
            }
        };

        return Ok(a);
    }
}

impl Track {
    pub async fn insert_into_db(track: &str, album: &Album, pool: &SqlitePool) -> Result<Track> {
        let queryed_track = sqlx::query_as(
            "SELECT trackId, title, albumId, album FROM v_tracks WHERE title = ? AND albumId = ?;",
        )
        .bind(track)
        .bind(album.id)
        .fetch_optional(pool)
        .await?;

        let a = match queryed_track {
            Some(a) => {
                info!("Track {} in Database", track);
                a
            }
            None => {
                info!("Track {} not in Database. Create it", track);
                sqlx::query_as("INSERT INTO tracks (title, album) VALUES (?, ?); SELECT trackId, title, albumId, album FROM v_tracks WHERE title = ? AND albumId = ?;")
                .bind(track)
                .bind(album.id)
                .bind(track)
                .bind(album.id)
                .fetch_one(pool)
                .await?
            }
        };

        return Ok(a);
    }
}

impl<'r> FromRow<'r, SqliteRow> for Track {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let track_id = row.try_get("trackId")?;
        let title = row.try_get("title")?;
        let album_id = row.try_get("albumId")?;
        let album = row.try_get("album")?;

        Ok(Track {
            id: track_id,
            title,
            album: Album {
                id: album_id,
                name: album,
            },
        })
    }
}
