#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::{collections::HashSet, env};
use std::{fs, path::PathBuf};

#[derive(Debug, Hash, Eq)]
struct Artist {
    id: i64,
    name: String,
}

impl PartialEq for Artist {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
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

    let mut all_artist = HashSet::new();
    for file in all_files {
        let reader = claxon::FlacReader::open(file)?;
        let artists = reader.get_tag("artist").collect::<Vec<_>>();

        for artist in artists {
            let queryed_artist = sqlx::query_as!(
                Artist,
                "SELECT id, name FROM artists WHERE name = ?;",
                artist
            )
            .fetch_optional(&pool)
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
                    .fetch_one(&pool)
                    .await?
                }
            };

            all_artist.insert(a);
        }
    }

    println!("{:?}", all_artist);
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
