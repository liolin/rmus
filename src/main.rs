#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{collections::HashMap, env};
use std::{fs, path::PathBuf};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Artist {
    id: i64,
    name: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Album {
    id: i64,
    name: String,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    let music_dir = env::var("MUSIC_DIR").unwrap();
    let db_uri = env::var("DATABASE_URL").unwrap();
    build_database_from_dir(&music_dir, &db_uri).await?;
    Ok(())
}

async fn build_database_from_dir(music_dir: &String, db_uri: &String) -> Result<()> {
    let pool = SqlitePoolOptions::new().connect(db_uri).await?;
    let all_files = find_files(music_dir)?
        .into_iter()
        .filter(|f| f.extension().unwrap() == "flac")
        .collect::<Vec<_>>();

    let mut all_artists = HashMap::new();
    let mut all_albums = HashMap::new();
    for file in all_files {
        let reader = claxon::FlacReader::open(file)?;
        let artists = reader.get_tag("artist").collect::<Vec<_>>();
        let albums = reader.get_tag("album").collect::<Vec<_>>();

        for artist in artists {
            if !all_artists.contains_key(artist) {
                all_artists.insert(artist.to_owned(), insert_artist_in_db(artist, &pool).await?);
            }
        }

        for album in albums {
            if !all_albums.contains_key(album) {
                all_albums.insert(album.to_owned(), insert_album_in_db(album, &pool).await?);
            }
        }
    }

    println!("{:?}", all_artists);
    println!("{:?}", all_albums);
    Ok(())
}

async fn insert_artist_in_db(artist: &str, pool: &SqlitePool) -> Result<Artist> {
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

async fn insert_album_in_db(album: &str, pool: &SqlitePool) -> Result<Album> {
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

fn find_files(path: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for e in fs::read_dir(path)? {
        let path = e?.path();

        if path.is_dir() {
            files.append(&mut find_files(path.to_str().unwrap())?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
