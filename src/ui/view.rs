use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::model::{Artist, Track};
use crate::ui::widget::StatefulList;
use crate::App;

#[derive(Debug)]
pub enum View {
    Library((StatefulList<Artist>, StatefulList<Track>)),
    Track(StatefulList<Track>),
}

pub fn update_view<T>(frame: &mut Frame<T>, app: &mut App)
where
    T: tui::backend::Backend,
{
    match &mut app.view {
        View::Track(track_list) => {
            render_track(frame, track_list);
        }
        View::Library(artist_list) => {
            render_library(frame, artist_list);
        }
    }
}

fn render_track<T>(frame: &mut Frame<T>, track_list: &mut StatefulList<Track>)
where
    T: tui::backend::Backend,
{
    let list_entries: Vec<ListItem> = track_list
        .items
        .iter()
        .map(|e| ListItem::new(format!("{} - {}", e.title.as_str(), e.album.name.as_str())))
        .collect();

    let tracks = List::new(list_entries)
        .block(Block::default().title("Track").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    frame.render_stateful_widget(tracks, frame.size(), &mut track_list.state);
}

fn render_library<T>(frame: &mut Frame<T>, list: &mut (StatefulList<Artist>, StatefulList<Track>))
where
    T: tui::backend::Backend,
{
    let (artist_list, track_list) = list;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(frame.size());

    let track_entries: Vec<ListItem> = track_list
        .items
        .iter()
        .map(|e| ListItem::new(format!("{} - {}", e.title.as_str(), e.album.name.as_str())))
        .collect();

    let tracks = List::new(track_entries)
        .block(Block::default().title("Track").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    let list_entries: Vec<ListItem> = artist_list
        .items
        .iter()
        .map(|e| ListItem::new(e.name.as_str()))
        .collect();

    let artists = List::new(list_entries)
        .block(
            Block::default()
                .title("Artist / Album")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    frame.render_stateful_widget(artists, chunks[0], &mut artist_list.state);
    frame.render_stateful_widget(tracks, chunks[1], &mut track_list.state);
}
