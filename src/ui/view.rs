use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::App;

#[derive(Debug)]
pub enum View {
    TrackView,
}

pub fn render_track<T>(frame: &mut Frame<T>, app: &mut App)
where
    T: tui::backend::Backend,
{
    let size = frame.size();

    let list_entries: Vec<ListItem> = app
        .tracks
        .items
        .iter()
        .map(|e| ListItem::new(e.title.as_str()))
        .collect();

    let l = List::new(list_entries)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    frame.render_stateful_widget(l, size, &mut app.tracks.state);
}
