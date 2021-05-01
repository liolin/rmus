#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::{collections::HashMap, env};
use std::{fs, path::PathBuf};

use termion::event::Key;

use rmus::{
    app::App,
    model::{self, Album, Artist, Track},
    player::Player,
    ui::{
        self,
        view::{self, View},
        widget::StatefulList,
    },
    util::Events,
};

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
    let mut terminal = ui::init_view()?;

    let mut app = App {
        view: View::Track(StatefulList::from_vec(tracks)),
        pool,
        player: Player::new(),
        events: Events::new(),
    };
    terminal.clear()?;

    loop {
        terminal.draw(|f| view::update_view(f, &mut app))?;

        match app.events.next()? {
            Key::Up => {
                app.previous();
            }
            Key::Down => {
                app.next();
            }
            Key::Left => {
                app.unselect();
            }
            Key::Char('q') => {
                break;
            }
            Key::Char('1') => {
                let artists = Artist::select_all(&app.pool).await?;
                let tracks = Track::by_artist(&app.pool, &artists[0]).await?;
                app.view = View::Library((
                    StatefulList::from_vec(artists),
                    StatefulList::from_vec(tracks),
                ));
            }
            Key::Char('5') => {
                let tracks = Track::select_all(&app.pool).await?;
                app.view = View::Track(StatefulList::from_vec(tracks));
            }
            Key::Char('\n') => {
                app.select();
            }
            _ => {}
        }
    }

    Ok(())
}

async fn build_database_from_dir(music_dir: &String, pool: &SqlitePool) -> Result<()> {
    let all_files = find_files(music_dir)?
        .into_iter()
        .filter(|f| {
            if let Some(e) = f.extension() {
                e == "flac"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    let mut all_artists = HashMap::new();
    let mut all_albums = HashMap::new();

    for file in all_files {
        let reader = claxon::FlacReader::open(&file)?;
        let artists = reader.get_tag("artist").collect::<Vec<_>>();
        let album = reader.get_tag("album").collect::<String>();
        let title = reader.get_tag("title").collect::<String>();

        // TODO: no clone, and other improvments
        for artist in artists.clone() {
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

        if let Ok(path) = &file.into_os_string().into_string() {
            let track = Track::insert_into_db(
                &title,
                all_albums.get(&album).unwrap(),
                all_artists.get(&artists[0].to_string()).unwrap(),
                path,
                &pool,
            )
            .await?;
            println!("{:#?}\n", track);
        } else {
            warn!("Could not convert a file path to a string");
        }
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
