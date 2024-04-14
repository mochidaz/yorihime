use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use tui::{backend::Backend, Terminal};

use crate::app::{App, AppReturn, AppStatus, InputMode};
use crate::errors::ErrorKind;
use crate::errors::Result;
use crate::inputs::events::Events;
use crate::inputs::key::Key;
use crate::inputs::InputEvent;
use crate::ui::ui::ui;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: &Rc<RefCell<App>>) -> Result<()> {
    terminal.clear()?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();

        terminal.draw(|f| {
            ui(f, &mut app);
        })?;

        if let Some(err) = &app.status.error() {
            match events.next()? {
                InputEvent::Input(key) => match key {
                    _ => {
                        app.status = AppStatus::Running;
                        continue;
                    }
                },
                InputEvent::Tick => continue,
            }
        } else if let Some(msg) = &app.status.success() {
            match events.next()? {
                InputEvent::Input(key) => match key {
                    _ => {
                        app.status = AppStatus::Running;
                        continue;
                    }
                },
                InputEvent::Tick => continue,
            }
        }

        let result = match app.input_mode {
            InputMode::Selecting => match events.next() {
                Ok(InputEvent::Input(key)) => match app.execute(key) {
                    Ok(AppReturn::Continue) => continue,
                    Ok(AppReturn::Exit) => break,
                    Err(err) => {
                        app.status = AppStatus::Error(err);
                        continue;
                    }
                },
                Ok(InputEvent::Tick) => app.tick(),
                Err(err) => {
                    app.status = AppStatus::Error(ErrorKind::from(err));
                    continue;
                }
            },
            InputMode::Editing => match events.next()? {
                InputEvent::Input(key) => match key {
                    Key::Char(c) => {
                        app.input.push(c);
                        continue;
                    }
                    Key::Backspace => {
                        app.input.pop();
                        continue;
                    }
                    Key::Enter => match app.execute_input() {
                        Ok(AppReturn::Continue) => continue,
                        Ok(AppReturn::Exit) => break,
                        Err(err) => {
                            app.status = AppStatus::Error(err);
                            continue;
                        }
                    },
                    Key::Esc => {
                        app.input_mode = InputMode::Selecting;
                        continue;
                    }
                    _ => {
                        continue;
                    }
                },

                InputEvent::Tick => app.tick(),
            },
        };
    }

    Ok(())
}
