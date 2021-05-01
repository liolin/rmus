use anyhow::Result;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteRow},
    FromRow, Row, SqlitePool,
};

#[derive(Debug, Hash, PartialEq, Eq, FromRow)]
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
    pub artist: Artist,
    pub file_path: String,
}

pub async fn establish_database_connection(db_uri: &String) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new().connect(db_uri).await?;
    Ok(pool)
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

    pub async fn select_all(pool: &SqlitePool) -> Result<Vec<Artist>> {
        let all_artist = sqlx::query_as("SELECT id, name FROM artists;")
            .fetch_all(pool)
            .await?;

        Ok(all_artist)
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
    pub async fn insert_into_db(
        track: &str,
        album: &Album,
        artist: &Artist,
        file_path: &String,
        pool: &SqlitePool,
    ) -> Result<Track> {
        let queryed_track = sqlx::query_as(
            "SELECT trackId, title, albumId, album, artistId, artist, filePath FROM v_tracks WHERE title = ? AND albumId = ?;",
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

                let _: Album = sqlx::query_as!(
                    Album,
                    "SELECT id, name FROM albums WHERE id = ? AND name = ?;",
                    album.id,
                    album.name
                )
                .fetch_one(pool)
                .await?;

                sqlx::query_as("INSERT INTO tracks (title, album, artist, filePath) VALUES (?, ?, ?, ?); SELECT trackId, title, albumId, album, artistId, artist, filePath FROM v_tracks WHERE title = ? AND albumId = ?;")
                .bind(track)
                .bind(album.id)
                .bind(artist.id)
                .bind(file_path)
                .bind(track)
                .bind(album.id)
                .fetch_one(pool)
                .await?
            }
        };

        return Ok(a);
    }

    pub async fn select_all(pool: &SqlitePool) -> Result<Vec<Track>> {
        let all_tracks = sqlx::query_as(
            "SELECT trackId, title, albumId, album, artistId, artist, filePath FROM v_tracks;",
        )
        .fetch_all(pool)
        .await?;

        Ok(all_tracks)
    }

    pub async fn by_artist(pool: &SqlitePool, artist: &Artist) -> Result<Vec<Track>> {
        let tracks = sqlx::query_as(
	    "SELECT trackId, title, albumId, album, artistId, artist, filePath FROM v_tracks WHERE artistId = ?;"
	)
	    .bind(artist.id)
	    .fetch_all(pool)
	    .await?;

        Ok(tracks)
    }
}

impl<'r> FromRow<'r, SqliteRow> for Track {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let track_id = row.try_get("trackId")?;
        let title = row.try_get("title")?;
        let album_id = row.try_get("albumId")?;
        let album = row.try_get("album")?;
        let artist_id = row.try_get("artistId")?;
        let artist = row.try_get("artist")?;
        let file_path = row.try_get("filePath")?;

        Ok(Track {
            id: track_id,
            title,
            album: Album {
                id: album_id,
                name: album,
            },
            artist: Artist {
                id: artist_id,
                name: artist,
            },
            file_path,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::test_helpers;
    use async_std::task;
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    #[test]
    fn test_artist_insert_into_db() {
        test_helpers::test_against_database(|database_url| {
            task::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .connect(database_url)
                    .await
                    .unwrap();
                let artist = Artist {
                    id: 1,
                    name: String::from("Epica"),
                };

                let artist_from_db = Artist::insert_into_db(&artist.name, &pool).await.unwrap();
                assert_eq!(artist.name, artist_from_db.name);
            });
        });
    }

    #[test]
    fn test_album_insert_into_db() {
        test_helpers::test_against_database(|database_url| {
            task::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .connect(database_url)
                    .await
                    .unwrap();
                let album = Album {
                    id: 1,
                    name: String::from("Omega"),
                };

                let album_from_db = Album::insert_into_db(&album.name, &pool).await.unwrap();
                assert_eq!(album.name, album_from_db.name);
            });
        });
    }

    #[test]
    fn test_track_insert_into_db_with_existing_album() {
        test_helpers::test_against_database(|database_url| {
            task::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .connect(database_url)
                    .await
                    .unwrap();

                let track = Track {
                    id: 1,
                    title: String::from("Alpha – Anteludium"),
                    album: Album {
                        id: 1,
                        name: String::from("Omega"),
                    },
                    artist: Artist {
                        id: 1,
                        name: String::from("Epica"),
                    },
                    file_path: String::from("/music/epica/alpah.flac"),
                };

                let album_from_db = Album::insert_into_db(&track.album.name, &pool)
                    .await
                    .unwrap();
                assert_eq!(track.album.name, album_from_db.name);

                let artist_from_db = Artist::insert_into_db(&track.artist.name, &pool)
                    .await
                    .unwrap();
                assert_eq!(track.artist.name, artist_from_db.name);

                let track_from_db = Track::insert_into_db(
                    &track.title,
                    &track.album,
                    &track.artist,
                    &track.file_path,
                    &pool,
                )
                .await
                .unwrap();
                assert_eq!(track.title, track_from_db.title);
            });
        });
    }

    #[test]
    fn test_track_insert_into_db_with_non_existing_album() {
        test_helpers::test_against_database(|database_url| {
            task::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .connect(database_url)
                    .await
                    .unwrap();

                let track = Track {
                    id: 1,
                    title: String::from("Alpha – Anteludium"),
                    album: Album {
                        id: 1,
                        name: String::from("Omega"),
                    },
                    artist: Artist {
                        id: 1,
                        name: String::from("Epica"),
                    },
                    file_path: String::from("/music/epica/alpah.flac"),
                };

                let track_from_db = Track::insert_into_db(
                    &track.title,
                    &track.album,
                    &track.artist,
                    &track.file_path,
                    &pool,
                )
                .await;
                assert_eq!(
                    track_from_db.err().unwrap().to_string(),
                    "no rows returned by a query that expected to return at least one row"
                        .to_string()
                );
            });
        });
    }

    #[test]
    fn test_track_insert_multiple_song_into_db_with_same_album() {
        test_helpers::test_against_database(|database_url| {
            task::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .connect(database_url)
                    .await
                    .unwrap();

                let track = Track {
                    id: 1,
                    title: String::from("Alpha – Anteludium"),
                    album: Album {
                        id: 1,
                        name: String::from("Omega"),
                    },
                    file_path: String::from("/music/epica/alpah.flac"),
                    artist: Artist {
                        id: 1,
                        name: String::from("Epica"),
                    },
                };

                let track2 = Track {
                    id: 2,
                    title: String::from("Abyss of Time – Countdown to Singularity"),
                    album: Album {
                        id: 1,
                        name: String::from("Omega"),
                    },
                    artist: Artist {
                        id: 1,
                        name: String::from("Epica"),
                    },
                    file_path: String::from("/music/epica/abyss.flac"),
                };

                let track_from_db = Track::insert_into_db(
                    &track.title,
                    &track.album,
                    &track.artist,
                    &track.file_path,
                    &pool,
                )
                .await;
                assert_eq!(
                    track_from_db.err().unwrap().to_string(),
                    "no rows returned by a query that expected to return at least one row"
                        .to_string()
                );

                let track_from_db = Track::insert_into_db(
                    &track2.title,
                    &track2.album,
                    &track2.artist,
                    &track2.file_path,
                    &pool,
                )
                .await;
                assert_eq!(
                    track_from_db.err().unwrap().to_string(),
                    "no rows returned by a query that expected to return at least one row"
                        .to_string()
                );
            });
        });
    }
}
