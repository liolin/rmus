use crate::{
    player::Player,
    ui::view::{self, View},
    util::Events,
};
use sqlx::SqlitePool;

pub struct App<P: Player> {
    pub view: view::View,
    pub pool: SqlitePool,
    pub player: P,
    pub events: Events,
}

impl<P: Player> App<P> {
    pub fn previous(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                list.previous();
            }
            View::Library(list) => {
                let (list, _) = list;
                list.previous();
            }
        }
    }

    pub fn next(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                list.next();
            }
            View::Library(list) => {
                let (list, _) = list;
                list.next();
            }
        }
    }

    pub fn select(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                if let Some(selected) = list.state.selected() {
                    let track = &list.items[selected];
                    self.player.play_new_track(&track.file_path);
                }
            }
            _ => {}
        }
    }

    pub fn unselect(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                list.unselect();
            }
            View::Library(list) => {
                let (list, _) = list;
                list.unselect();
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.player.toggle_pause();
    }
}
