use std::cell::RefCell;
use std::error::Error;
use std::io;
use std::rc::Rc;

use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use process_memory::{self, Memory, TryIntoProcessHandle};
use sysinfo::{ProcessExt, SystemExt};

use crate::app::App;
use crate::ui::tui::run_app;

mod app;
mod config;
mod errors;
mod game;
mod readers;

mod inputs;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = Rc::new(RefCell::new(App::new()));
    let res = run_app(&mut terminal, &app);

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
