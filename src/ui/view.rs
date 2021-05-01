use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::model::{Artist, Track};
use crate::ui::widget::StatefulList;

pub trait ViewTrait<T>
where
    T: tui::backend::Backend,
{
    fn render(&mut self, frame: &mut Frame<T>);
    fn previous(&mut self);
    fn next(&mut self);
    fn current(&mut self) -> Option<&Track>;
    fn unselect(&mut self);
    fn change_focus(&mut self);
}

pub struct TrackView {
    track_list: StatefulList<Track>,
}

impl TrackView {
    pub fn new(track_list: StatefulList<Track>) -> Self {
        TrackView { track_list }
    }
}

impl<T> ViewTrait<T> for TrackView
where
    T: tui::backend::Backend,
{
    fn render(&mut self, frame: &mut Frame<T>) {
        let list_entries: Vec<ListItem> = self
            .track_list
            .items
            .iter()
            .map(|e| ListItem::new(format!("{} - {}", e.title.as_str(), e.album.name.as_str())))
            .collect();

        let tracks = List::new(list_entries)
            .block(Block::default().title("Track").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(tracks, frame.size(), &mut self.track_list.state);
    }

    fn previous(&mut self) {
        self.track_list.previous();
    }

    fn next(&mut self) {
        self.track_list.next();
    }

    fn current(&mut self) -> Option<&Track> {
        if let Some(selected) = self.track_list.state.selected() {
            Some(&self.track_list.items[selected])
        } else {
            None
        }
    }

    fn unselect(&mut self) {
        self.track_list.unselect();
    }

    fn change_focus(&mut self) {}
}

pub struct LibraryView {
    track_list: StatefulList<Track>,
    artist_list: StatefulList<Artist>,
    active_list: u32,
}

impl LibraryView {
    pub fn new(artist_list: StatefulList<Artist>, track_list: StatefulList<Track>) -> Self {
        LibraryView {
            track_list,
            artist_list,
            active_list: 0,
        }
    }
}

impl<T> ViewTrait<T> for LibraryView
where
    T: tui::backend::Backend,
{
    fn render(&mut self, frame: &mut Frame<T>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(frame.size());

        let track_entries: Vec<ListItem> = self
            .track_list
            .items
            .iter()
            .map(|e| ListItem::new(format!("{} - {}", e.title.as_str(), e.album.name.as_str())))
            .collect();

        let tracks = List::new(track_entries)
            .block(Block::default().title("Track").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let list_entries: Vec<ListItem> = self
            .artist_list
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

        frame.render_stateful_widget(artists, chunks[0], &mut self.artist_list.state);
        frame.render_stateful_widget(tracks, chunks[1], &mut self.track_list.state);
    }

    fn previous(&mut self) {
        match self.active_list {
            0 => self.artist_list.previous(),
            1 => self.track_list.previous(),
            _ => {}
        }
    }

    fn next(&mut self) {
        match self.active_list {
            0 => self.artist_list.next(),
            1 => self.track_list.next(),
            _ => {}
        }
    }

    fn unselect(&mut self) {
        match self.active_list {
            0 => self.artist_list.unselect(),
            1 => self.track_list.unselect(),
            _ => {}
        }
    }

    fn current(&mut self) -> Option<&Track> {
        if let Some(selected) = self.track_list.state.selected() {
            Some(&self.track_list.items[selected])
        } else {
            None
        }
    }

    fn change_focus(&mut self) {
        match self.active_list {
            0 => self.active_list = 1,
            1 => self.active_list = 0,
            _ => {}
        }
    }
}
