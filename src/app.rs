use crate::errors::ErrorKind;
use tui::widgets::TableState;
use crate::app::RunningState::NotRunning;
use crate::config::Config;
use crate::utils::get_running_games;

pub enum InputMode {
    Normal,
    Editing,
}

pub enum Status {
    Success,
    Error(ErrorKind),
}

pub enum RunningState {
    Running,
    NotRunning,
}

pub enum Selecting {
    Score,
    Lives,
    Bombs,
    Power,
}

#[derive(PartialEq)]
pub enum Menu {
    Main,
    GameSelection,
    CheatSelection,
}

pub struct App<'a> {
    pub state: TableState,
    pub input: String,
    pub input_mode: InputMode,
    pub process_name: Option<String>,
    pub pid: Option<i32>,
    pub available_games: Vec<String>,
    pub game_selection: Option<Vec<Vec<&'a str>>>,
    pub items: Vec<Vec<&'a str>>,
    pub status: Option<Status>,
    pub running_state: RunningState,
    pub selecting_game: bool,
    pub selecting: Option<Selecting>,
    pub history: Vec<Menu>,
    pub history_index: usize,
    pub selected_game: usize,
    pub cfg: Config,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let running_games = get_running_games();

        let pid = None;

        let config = Config::new();

        App {
            state: Default::default(),
            input: String::new(),
            input_mode: InputMode::Normal,
            process_name: None,
            pid,
            available_games: running_games,
            game_selection: None,
            items: vec![vec!["Select a game"]],
            status: None,
            running_state: NotRunning,
            selecting_game: false,
            selecting: None,
            history: vec![Menu::Main],
            history_index: 0,
            selected_game: 0,
            cfg: config,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}