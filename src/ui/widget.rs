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
        if let Some(mut i) = self.state.selected() {
            i += 1;
            if i == self.items.len() {
                i = 0;
            }
            self.state.select(Some(i));
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn previous(&mut self) {
        if let Some(mut i) = self.state.selected() {
            if i == 0 {
                i = self.items.len() - 1;
            } else {
                i -= 1;
            }
            self.state.select(Some(i));
        } else {
            self.state.select(Some(0));
        }
    }
}
