use crossterm::event::{Event, KeyCode};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Instant;
use std::{error::Error, time::Duration};

use backend::Backend;
use frontend::ui::{self, StatefulList};
use tui::{backend::CrosstermBackend, Terminal};

mod backend;
mod frontend;

// TODO: This needs to also include a bool for if it is completed
type ItemViewModel = String;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

pub struct App {
    backend: Backend,
    ui_items: StatefulList<ItemViewModel>,
}

impl App {
    fn new() -> App {
        let mut backend = Backend::new();
        if let Err(_) = backend.restore_items() {
            println!("Failed to restore item state.");
        }

        let items: Vec<String> = backend
            .get_items()
            .iter()
            .map(|i| i.get_title().to_string())
            .collect();

        App {
            backend,
            ui_items: StatefulList::with_items(items),
        }
    }

    fn update_item_list(&mut self) {
        let state = self.ui_items.get_state();
        self.ui_items = StatefulList::with_items(
            self.backend
                .get_items()
                .iter()
                .map(|i| i.get_title().to_string())
                .collect(),
        );
        self.ui_items.set_state(state);
    }

    pub fn on_tick(&mut self) {
        self.update_item_list();
    }
}

fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::ui(frame, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Up => app.ui_items.prev(),
                    KeyCode::Down => app.ui_items.next(),
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => app.backend.save_items().expect("Failed to save items"),
                    KeyCode::Char('a') => {
                        let mut buf = String::new();
                        // TODO: Fortsätt här, kolla på user-input exemplet på tui-github, går att ändra input-mode osv osv
                        io::stdin().read_line(&mut buf).expect("Failed to read line!");
                        app.backend.add_item(buf.as_str());
                    }
                    // TODO: Fill in keybindings here
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
