use std::time::Instant;
use std::{error::Error, time::Duration};
use std::io;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}
};

use backend::{Backend};
use tui::{backend::CrosstermBackend, Terminal};

mod backend;
mod frontend;

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

struct App {
    backend: Backend,
}

impl App {
    fn new() -> App {
        let mut backend = Backend::new();
        if let Err(_) = backend.restore_items() {
            println!("Failed to restore item state.");
        }

        App {
            backend,
        }

    }
}


fn run_app<B: tui::backend::Backend> (
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        // Draw stuff here
    }

    Ok(())
}