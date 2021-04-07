#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::{collections::HashMap, env};
use std::{fs, path::PathBuf};

mod models;

use crate::models::{Album, Artist, Track};

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
        let reader = claxon::FlacReader::open(&file)?;
        let artists = reader.get_tag("artist").collect::<Vec<_>>();
        let album = reader.get_tag("album").collect::<String>();
        let title = reader.get_tag("title").collect::<String>();

        for artist in artists {
            if !all_artists.contains_key(artist) {
                all_artists.insert(
                    artist.to_owned(),
                    Artist::insert_into_db(artist, &pool).await?,
                );
            }
        }

        if !all_albums.contains_key(&album) {
            let a = Album::insert_into_db(&album, &pool).await?;
            all_albums.insert(album.clone(), a);
        }

        let track = Track::insert_into_db(
            &title,
            all_albums.get(&album).unwrap(),
            &file.into_os_string().into_string().unwrap(),
            &pool,
        )
        .await?;

        println!("{:#?}\n", track);
    }

    Ok(())
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
