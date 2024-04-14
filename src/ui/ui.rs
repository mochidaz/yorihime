use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Color::Rgb;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::app::{App, AppMenu, InputMode};
use crate::errors::ErrorKind;

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
    let block = Block::default()
        .title("Notification")
        .borders(Borders::ALL)
        .style(Style::default().bg(Rgb(0, 0, 0)).fg(Rgb(144, 238, 144)));
    let paragraph = Paragraph::new(msg.to_string())
        .block(block.clone())
        .alignment(Alignment::Center);
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
    f.render_widget(block, area);
}

fn error<B: Backend>(f: &mut Frame<B>, error: &ErrorKind) {
    let block = Block::default()
        .title(format!("An error occured!"))
        .borders(Borders::ALL)
        .style(Style::default().bg(Rgb(0, 0, 0)).fg(Rgb(255, 0, 0)));
    let text = vec![Spans::from(format!("Error: {}", error))];
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

    if let Some(err) = &app.status.error() {
        error(f, err);
        return;
    }

    if let Some(success) = &app.status.success() {
        confirmation(f, success);
        return;
    }

    match app.current_menu {
        AppMenu::Main => main_menu(f, chunks, app),
        AppMenu::GameSelection => game_selection(f, chunks, app),
        AppMenu::CheatSelection => cheat_selection(f, chunks, app),
    }
}

fn main_menu<B: Backend>(f: &mut Frame<B>, chunks: Vec<Rect>, app: &mut App) {
    let wrapper = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Rgb(255, 255, 255)))
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled(
            "Yorihime",
            Style::default().add_modifier(Modifier::BOLD),
        )));

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

    f.render_stateful_widget(t, chunks[0], &mut app.table_state);
}

fn game_selection<B: Backend>(f: &mut Frame<B>, chunks: Vec<Rect>, app: &mut App) {
    let wrapper = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Rgb(255, 255, 255)))
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled(
            "Yorihime",
            Style::default().add_modifier(Modifier::BOLD),
        )));

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Rgb(144, 238, 144));
    let header_cells = ["Select Game"]
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
        let cells = item.iter().map(|c| {
            let mut content = c.to_string();
            if content.width() > 30 {
                content = content.chars().take(27).collect::<String>();
                content.push_str("...");
            }
            Cell::from(content)
        });
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

    f.render_stateful_widget(t, chunks[0], &mut app.table_state);
}

fn cheat_selection<B: Backend>(f: &mut Frame<B>, chunks: Vec<Rect>, app: &mut App) {
    let wrapper = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Rgb(255, 255, 255)))
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled(
            "Yorihime",
            Style::default().add_modifier(Modifier::BOLD),
        )));

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Rgb(144, 238, 144));
    let header_cells = ["Select Cheat"]
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

    f.render_stateful_widget(t, chunks[0], &mut app.table_state);

    if let Some(selecting) = &app.selected_cheat {
        let input = Paragraph::new(app.input.as_ref())
            .style(match app.input_mode {
                InputMode::Selecting => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "Value",
                Style::default().add_modifier(Modifier::BOLD),
            )));
        f.render_widget(input, chunks[2]);
        match app.input_mode {
            InputMode::Selecting => {}

            InputMode::Editing => {
                f.set_cursor(chunks[2].x + app.input.width() as u16 + 1, chunks[2].y + 1)
            }
        }
    }
}
