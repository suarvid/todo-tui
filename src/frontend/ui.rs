use tui::widgets::ListState;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let idx = match self.state.selected() {
            Some(idx) => {
                if idx >= self.items.len() - 1 {
                    0
                } else {
                    idx + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(idx))
    }

    fn prev(&mut self) {
        let idx = match self.state.selected() {
            Some(idx) => {
                if idx == 0 {
                    self.items.len() - 1
                } else {
                    idx - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(idx))
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
