use tui::widgets::ListState;

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn next(&mut self) {
        if let Some(i) = self.state.selected() {
            self.state.select(Some(i + 1));
        }
    }

    pub fn previous(&mut self) {
        if let Some(i) = self.state.selected() {
            self.state.select(Some(i - 1));
        }
    }
}
