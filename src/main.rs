#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::{collections::HashMap, env};
use std::{fs, path::PathBuf};

use termion::event::Key;
use tui::widgets::ListState;

mod model;
#[cfg(test)]
mod test_helpers;
mod ui;
mod util;

use crate::model::{Album, App, Artist, Track};
use crate::ui::view::{self, View};
use crate::ui::widget::StatefulList;
use crate::util::Events;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    let music_dir = env::var("MUSIC_DIR")?;
    let db_uri = env::var("DATABASE_URL")?;
    let pool = model::establish_database_connection(&db_uri).await?;
    let n = env::args();

    if n.len() > 1 {
        info!("Init database");
        build_database_from_dir(&music_dir, &pool).await?;
    }

    let tracks = Track::select_all(&pool).await?;
    let mut state = ListState::default();
    state.select(Some(1));

    let mut app = App {
        tracks: StatefulList {
            state,
            items: tracks,
        },
        view: View::TrackView,
        pool,
    };

    let mut terminal = ui::init_view()?;
    let events = Events::new();
    terminal.clear()?;

    loop {
        match app.view {
            View::TrackView => {
                terminal.draw(|f| view::render_track(f, &mut app))?;
            }
        }

        match events.next()? {
            Key::Up => {
                app.tracks.previous();
            }
            Key::Down => {
                app.tracks.next();
            }
            Key::Left => {
                app.tracks.unselect();
            }
            Key::Char('q') => {
                break;
            }
            Key::Char('\n') => {
                let selected = app.tracks.state.selected().unwrap();
                let track = &app.tracks.items[selected];
                println!("Play {}", track.title)
            }
            _ => {}
        }
    }

    Ok(())
}

async fn build_database_from_dir(music_dir: &String, pool: &SqlitePool) -> Result<()> {
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
