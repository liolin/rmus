#[macro_use]
extern crate log;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{collections::HashMap, env};
use std::{fs, path::PathBuf};

use std::io;
use termion::{
    event::{Event, Key},
    input::Events,
    raw::IntoRawMode,
};
use tui::Terminal;
use tui::{
    backend::TermionBackend,
    widgets::{List, ListItem},
};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
};
use tui::{
    text::Span,
    widgets::{Block, Borders, ListState, Widget},
};

mod models;
#[cfg(test)]
mod test_helpers;

use crate::models::{Album, Artist, Track};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn unselect(&mut self) {
        self.state.select(None);
    }

    fn next(&mut self) {
        if let Some(i) = self.state.selected() {
            self.state.select(Some(i + 1));
        }
    }

    fn previous(&mut self) {
        if let Some(i) = self.state.selected() {
            self.state.select(Some(i - 1));
        }
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    let music_dir = env::var("MUSIC_DIR").unwrap();
    let db_uri = env::var("DATABASE_URL").unwrap();
    let pool = SqlitePoolOptions::new().connect(&db_uri).await?;

    let n = env::args();

    if n.len() > 1 {
        info!("Init database");
        build_database_from_dir(&music_dir, &pool).await?;
    }

    // INIT UI
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = Track::select_all(&pool).await.unwrap();
    let mut state = ListState::default();
    state.select(Some(1));
    let mut state_full_list = StatefulList { state, items };

    terminal.clear().unwrap();

    loop {
        terminal
            .draw(|f| {
                let size = f.size();

                let list_entries: Vec<ListItem> = state_full_list
                    .items
                    .iter()
                    .map(|e| ListItem::new(e.title.as_str()))
                    .collect();

                let l = List::new(list_entries)
                    .block(Block::default().title("List").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(">>");

                f.render_stateful_widget(l, size, &mut state_full_list.state);
            })
            .unwrap();

        // TODO Event parsing
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
