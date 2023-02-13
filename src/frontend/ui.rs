use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::{widgets::ListState, Frame};

use crate::App;

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

    pub fn set_state(&mut self, state: ListState) {
        self.state = state;
    }

    pub fn next(&mut self) {
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

    pub fn prev(&mut self) {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn get_state(&mut self) -> ListState {
        self.state.clone()
    }
}

pub fn ui<B: tui::backend::Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    draw_items_widget(f, app, main_chunks[0]);
    draw_keybinds_widget(f, app, main_chunks[1]);
}

fn draw_items_widget<B: tui::backend::Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .ui_items
        .items
        .iter()
        .map(|item| {
            let lines = vec![Spans::from(item.as_str())];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("ToDo Items"))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("->");

    frame.render_stateful_widget(items, area, &mut app.ui_items.state);
}

// TODO
fn draw_keybinds_widget<B: tui::backend::Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
}
