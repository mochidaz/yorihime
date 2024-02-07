use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    Terminal,
    text::{Span, Spans}, widgets::{Block, Borders, Paragraph},
};
use tui::layout::{Alignment, Rect};
use tui::style::Color::Rgb;
use tui::widgets::{Cell, Clear, Row, Table};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, InputMode, Menu, RunningState, Selecting, Status};

use std::io::Read;
use crate::errors::ErrorKind;
use crate::readers::{get_pid_by_name, write_mem_value};
use crate::utils::{get_running_games, get_touhou_game_name};

fn selecting_game_state(app: &mut App) {
    app.process_name = Some(app.available_games[app.selected_game].clone());
    app.pid = get_pid_by_name(app.process_name.as_ref().unwrap());
    app.selecting_game = false;
    app.items = vec![vec!["Score"], vec!["Lives"], vec!["Bombs"], vec!["Power"]];
    app.running_state = RunningState::Running;
}



fn selecting_selections_state(app: &mut App) {
    app.items = vec![vec!["Select a game"]];
    app.running_state = RunningState::NotRunning;
    app.selecting = None;
}


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(event) = event::read()? {
            match app.input_mode {
                InputMode::Normal => {
                    match event.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Left => {
                            match &app.history.get(if app.history_index > 0 { app.history_index - 1 } else { 0 }) {
                                Some(menu) => {
                                    match menu {
                                        Menu::Main => {
                                            if app.history_index > 0 {
                                                app.history_index -= 1;
                                            }
                                            app.selecting = None;
                                            app.selecting_game = false;
                                            app.items = vec![vec!["Select a game"]];
                                        }
                                        Menu::GameSelection => {
                                            app.history_index -= 1;
                                            app.selecting_game = true;
                                            app.selecting = None;
                                            let available_games: Vec<Vec<&str>> = app.available_games
                                                .clone()
                                                .into_iter()
                                                .map(|g| vec![Box::leak(get_touhou_game_name(g.as_str()).to_string().into_boxed_str()) as &str])
                                                .collect();
                                            app.items = available_games;
                                            app.running_state = RunningState::NotRunning;
                                        }
                                        Menu::CheatSelection => {
                                            app.history_index -= 1;
                                            selecting_game_state(&mut app);
                                        }
                                        _ => {}
                                    }
                                }
                                None => {}

                            }
                        }
                        KeyCode::Right => {
                            match &app.history.get(if app.history_index < app.history.len() - 1 { app.history_index + 1 } else { app.history.len() - 1 }) {
                                Some(menu) => {
                                    match menu {
                                        Menu::Main => {
                                            if app.history_index < app.history.len() - 1 {
                                                app.history_index += 1;
                                            }
                                            app.selecting_game = false;
                                            app.items = vec![vec!["Select a game"]];
                                        }
                                        Menu::GameSelection => {
                                            if app.history_index < app.history.len() - 1 {
                                                app.history_index += 1;
                                            }
                                            app.selecting_game = false;
                                            let available_games: Vec<Vec<&str>> = app.available_games
                                                .clone()
                                                .into_iter()
                                                .map(|g| vec![Box::leak(get_touhou_game_name(g.as_str()).to_string().into_boxed_str()) as &str])
                                                .collect();
                                            app.selecting = None;
                                            app.running_state = RunningState::NotRunning;
                                            app.items = available_games;
                                        }
                                        Menu::CheatSelection => {
                                            if app.history_index < app.history.len() - 1 {
                                                app.history_index += 1;
                                            }
                                            selecting_game_state(&mut app);
                                        }
                                        _ => {}
                                    }
                                }
                                None => {}
                            }
                        }
                        KeyCode::Esc => {
                            app.state.select(None);
                        }
                        KeyCode::Up => {
                            app.previous();
                        }
                        KeyCode::Down => {
                            app.next();
                        }
                        KeyCode::Enter => {
                            if let Some(s) = &app.status {
                                match s {
                                    Status::Error(e) => {
                                        match e {
                                            ErrorKind::NoGameFound => {
                                                app.status = None;
                                                app.available_games = get_running_games();
                                            }
                                            _ => {
                                                app.status = None;
                                            }
                                        }
                                    }
                                    Status::Success => {
                                        app.status = None;
                                    }
                                }
                            } else {
                                match (app.state.selected(), &app.running_state, &app.selecting_game) {
                                    (Some(0), &RunningState::NotRunning, false) => {
                                        let available_games: Vec<Vec<&str>> = app.available_games
                                            .clone()
                                            .into_iter()
                                            .map(|g| vec![Box::leak(get_touhou_game_name(g.as_str()).to_string().into_boxed_str()) as &str])
                                            .collect();

                                        if app.available_games.len() == 0 {
                                            app.status = Some(Status::Error(ErrorKind::NoGameFound));
                                            continue;
                                        }

                                        if let Some(history) = app.history.get(app.history_index + 1) {
                                            if history == &Menu::GameSelection {
                                                app.history_index += 1;
                                                app.selecting_game = true;
                                                app.items = available_games;
                                                continue;
                                            } else {
                                                app.history_index += 1;
                                                app.selecting_game = true;
                                                app.items = available_games;
                                                continue;
                                            }
                                        } else {
                                            &app.history.push(Menu::GameSelection);
                                            app.history_index += 1;
                                            app.selecting_game = true;
                                            app.items = available_games;
                                            continue;
                                        }
                                    }
                                    (Some(value), &RunningState::NotRunning, true) => {
                                        if let Some(history) = app.history.get(app.history_index + 1) {
                                            if history == &Menu::CheatSelection {
                                                app.selected_game = value;
                                                app.history_index += 1;
                                                app.pid = get_pid_by_name(app.available_games[value].as_str());
                                                app.selecting_game = false;
                                                selecting_game_state(&mut app);
                                                continue;
                                            } else {
                                                app.selected_game = value;
                                                app.history_index += 1;
                                                app.pid = get_pid_by_name(app.available_games[value].as_str());
                                                app.selecting_game = false;
                                                selecting_game_state(&mut app);
                                                continue;
                                            }
                                        }
                                        app.selected_game = value;
                                        app.pid = get_pid_by_name(app.available_games[value].as_str());
                                        app.history_index += 1;
                                        &app.history.push(Menu::CheatSelection);
                                        selecting_game_state(&mut app);
                                        continue;
                                    }
                                    (Some(value), &RunningState::Running, false) => {
                                        match value {
                                            0 => {
                                                app.selecting = Some(Selecting::Score);
                                                app.input_mode = InputMode::Editing;
                                            }
                                            1 => {
                                                app.selecting = Some(Selecting::Lives);
                                                app.input_mode = InputMode::Editing;
                                            }
                                            2 => {
                                                app.selecting = Some(Selecting::Bombs);
                                                app.status = Some(Status::Error(ErrorKind::NotSupported));
                                            }
                                            3 => {
                                                app.selecting = Some(Selecting::Power);
                                                app.status = Some(Status::Error(ErrorKind::NotSupported));
                                            }
                                            4 => {
                                                app.selecting = None;
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                InputMode::Editing => {
                    match event.code {
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => {
                            app.input_mode = InputMode::Normal;
                            if let Ok(parsed) = app.input.parse::<i32>() {
                                match &app.state.selected() {
                                    Some(0) => {
                                        match write_mem_value(app.pid.unwrap(), app.cfg.get_game(app.available_games[app.selected_game].as_str()).unwrap().score_mem_addr, parsed) {
                                            Ok(_) => {
                                                app.status = Some(Status::Success);
                                            }
                                            Err(e) => {
                                                app.status = Some(Status::Error(ErrorKind::FailedToWriteMemory));
                                            }
                                        }
                                    }
                                    Some(1) => {
                                        match write_mem_value(app.pid.unwrap(), app.cfg.get_game(app.available_games[app.selected_game].as_str()).unwrap().live_mem_addr, parsed) {
                                            Ok(_) => {
                                                app.status = Some(Status::Success);
                                            }
                                            Err(e) => {
                                                app.status = Some(Status::Error(ErrorKind::FailedToWriteMemory));
                                            }
                                        }
                                    }
                                    Some(2) => {
                                        app.status = Some(Status::Error(ErrorKind::NotSupported));
                                    }
                                    Some(3) => {
                                        app.status = Some(Status::Error(ErrorKind::NotSupported));
                                    }
                                    None => {}
                                    _ => {}
                                }
                            } else {
                                app.status = Some(Status::Error(ErrorKind::InvalidInput));
                            }
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn confirmation<B: Backend>(f: &mut Frame<B>, msg: &String) {
    let block = Block::default().title("Notification").borders(Borders::ALL)
        .style(Style::default().bg(Rgb(0,0,0)).fg(Rgb(144, 238, 144)));
    let paragraph = Paragraph::new(msg.to_string())
        .block(block.clone())
        .alignment(Alignment::Center);
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
    f.render_widget(block, area);
}

fn error<B: Backend>(f: &mut Frame<B>, error: &ErrorKind) {
    let block = Block::default().title(format!("An error occured!")).borders(Borders::ALL)
        .style(Style::default().bg(Rgb(0,0,0)).fg(Rgb(255,0,0)));
    let text = vec![
        Spans::from(format!("Error: {}", error))
    ];
    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Center);
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
    f.render_widget(block, area);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(4),
                Constraint::Length(3),
            ]
                .as_ref(),
        )
        .split(f.size());


    let wrapper =
        Block::default().borders(Borders::ALL)
            .style(Style::default().fg(Rgb(255,255,255)))
            .title_alignment(Alignment::Center)
            .title(Spans::from(Span::styled("Yorihime", Style::default().add_modifier(Modifier::BOLD))));

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Rgb(144, 238, 144));
    let header_cells = ["Selection Menu"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Rgb(0, 0, 0))));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(wrapper)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, chunks[0], &mut app.state);

    let status = match &app.running_state {
        RunningState::Running => "Running",
        RunningState::NotRunning => "Not running",
    };

    let text = vec![
        Spans::from(format!("{}", status))
    ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(text.clone())
        .block(create_block("Status"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[1]);

    match &app.status {
        Some(status) => {
            match status {
                Status::Error(err) => {
                    error(f, &err);
                }
                Status::Success=> {
                    confirmation(f, &"Success!".to_string());
                }
            }
        }
        None => {}
    }

    if let Some(selecting) = &app.selecting {
        let input = Paragraph::new(app.input.as_ref())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title(Span::styled("Value", Style::default().add_modifier(Modifier::BOLD))));
        f.render_widget(input, chunks[2]);
        match app.input_mode {
            InputMode::Normal =>
                {}

            InputMode::Editing => {
                f.set_cursor(
                    chunks[2].x + app.input.width() as u16 + 1,
                    chunks[2].y + 1,
                )
            }
        }
    }
}