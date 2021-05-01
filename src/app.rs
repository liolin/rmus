use crate::{player::Player, ui::view::ViewTrait, util::Events};
use sqlx::SqlitePool;

pub struct App<T, P>
where
    T: tui::backend::Backend,
    P: Player,
{
    pub view: Box<dyn ViewTrait<T>>,
    pub pool: SqlitePool,
    pub player: P,
    pub events: Events,
}

impl<T, P> App<T, P>
where
    T: tui::backend::Backend,
    P: Player,
{
    pub fn previous(&mut self) {
        self.view.previous();
    }

    pub fn next(&mut self) {
        self.view.next();
    }

    pub fn select(&mut self) {
        if let Some(track) = self.view.current() {
            self.player.play_new_track(&track.file_path);
        }
    }

    pub fn unselect(&mut self) {
        self.view.unselect();
    }

    pub fn change_focus(&mut self) {
        self.view.change_focus();
    }

    pub fn toggle_pause(&mut self) {
        self.player.toggle_pause();
    }
}
