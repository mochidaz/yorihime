use tui::widgets::TableState;

use crate::config::Config;
use crate::errors::ErrorKind;
use crate::errors::Result;
use crate::game::Game;
use crate::inputs::actions::{Action, Actions};
use crate::inputs::key::Key;
use crate::readers::{get_pid_by_name, write_mem_value};
use crate::utils::{get_running_games, get_touhou_game_name};

pub enum InputMode {
    Selecting,
    Editing,
}

pub enum RunningState {
    NotRunning,
    Running,
}

pub enum Cheat {
    Score,
    Lives,
    Bombs,
    Power,
}

#[derive(Clone)]
pub enum AppMenu {
    Main,
    GameSelection,
    CheatSelection,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub enum AppStatus {
    Success(String),
    Error(ErrorKind),
    Running,
}

impl AppStatus {
    pub fn success(&self) -> Option<&String> {
        match self {
            AppStatus::Success(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn error(&self) -> Option<&ErrorKind> {
        match self {
            AppStatus::Error(err) => Some(err),
            _ => None,
        }
    }
}

pub struct App<'a> {
    history: Vec<AppMenu>,
    actions: Actions,
    config: Config,
    pub current_game: Option<Game>,
    pub available_games: Vec<Game>,
    pub selected_cheat: Option<Cheat>,
    pub table_state: TableState,
    pub input: String,
    pub input_mode: InputMode,
    pub items: Vec<Vec<&'a str>>,
    pub current_menu: AppMenu,
    //pub error: Option<ErrorKind>,
    pub status: AppStatus,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            history: vec![AppMenu::Main],
            actions: Actions::from(vec![
                Action::Enter,
                Action::Up,
                Action::Down,
                Action::Quit,
                Action::Prev,
                Action::Next,
            ]),
            table_state: Default::default(),
            config: Config::new(),
            input: String::new(),
            input_mode: InputMode::Selecting,
            items: vec![vec!["Select Game"]],
            current_menu: AppMenu::Main,
            selected_cheat: None,
            //error: None,
            current_game: None,
            available_games: vec![],
            status: AppStatus::Running,
        }
    }

    fn update_items(&mut self) {
        match self.current_menu {
            AppMenu::Main => {
                self.items = vec![vec!["Select Game"]];
            }
            AppMenu::GameSelection => {
                let running_games = get_running_games();

                self.available_games = Vec::new();

                for game in running_games.iter() {
                    match self.config.get_game(&game) {
                        Some(g) => {
                            self.available_games.push(g.clone());
                        }
                        None => {}
                    }
                }

                self.items = running_games
                    .iter()
                    .map(|game| {
                        vec![
                            Box::leak(get_touhou_game_name(game).to_string().into_boxed_str())
                                as &str,
                        ]
                    })
                    .collect::<Vec<Vec<&str>>>();
            }
            AppMenu::CheatSelection => {
                self.items = vec![vec!["Score"], vec!["Lives"], vec!["Bombs"], vec!["Power"]];
            }
        }
    }

    pub fn add_history(&mut self, menu: AppMenu) {
        self.history.push(menu);
    }

    pub fn pop_history(&mut self) -> Option<AppMenu> {
        self.history.pop()
    }

    pub fn get_current_menu(&self) -> Result<&AppMenu> {
        match self.history.last() {
            Some(menu) => Ok(menu),
            None => Err(ErrorKind::NoMenuInHistory),
        }
    }

    pub fn execute(&mut self, key: Key) -> Result<AppReturn> {
        match self.get_current_menu()? {
            AppMenu::Main => self.execute_main_menu(key),
            AppMenu::GameSelection => self.execute_game_selection(key),
            AppMenu::CheatSelection => self.execute_cheat_selection(key),
        }
    }

    pub fn execute_main_menu(&mut self, key: Key) -> Result<AppReturn> {
        self.add_history(AppMenu::Main);

        match self.actions.find(key) {
            Some(action) => match action {
                Action::Enter => match self.items.get(match self.table_state.selected() {
                    Some(i) => i,
                    None => return Ok(AppReturn::Continue),
                }) {
                    Some(item) => match item[0] {
                        "Select Game" => {
                            self.add_history(AppMenu::GameSelection);
                            self.current_menu = self.history[self.history.len() - 1].clone();
                            self.update_items();

                            if self.items.len() == 0 {
                                self.status = AppStatus::Error(ErrorKind::NoGameFound);
                                self.pop_history();
                                self.current_menu = self.history[self.history.len() - 1].clone();
                                self.update_items();
                                return Ok(AppReturn::Continue);
                            }

                            Ok(AppReturn::Continue)
                        }
                        _ => Ok(AppReturn::Continue),
                    },
                    None => Ok(AppReturn::Continue),
                },

                Action::Up => {
                    self.previous();
                    Ok(AppReturn::Continue)
                }

                Action::Down => {
                    self.next();
                    Ok(AppReturn::Continue)
                }

                Action::Quit => Ok(AppReturn::Exit),

                Action::Prev => Ok(AppReturn::Continue),

                _ => Ok(AppReturn::Continue),
            },
            None => Ok(AppReturn::Continue),
        }
    }

    pub fn execute_game_selection(&mut self, key: Key) -> Result<AppReturn> {
        match self.actions.find(key) {
            Some(action) => match action {
                Action::Enter => {
                    self.add_history(AppMenu::CheatSelection);
                    self.current_menu = self.history[self.history.len() - 1].clone();
                    self.update_items();
                    self.current_game = Some(
                        self.available_games[match self.table_state.selected() {
                            Some(i) => i,
                            None => return Err(ErrorKind::NoGameFound),
                        }]
                        .clone(),
                    );
                    Ok(AppReturn::Continue)
                }

                Action::Up => {
                    self.previous();
                    Ok(AppReturn::Continue)
                }

                Action::Down => {
                    self.next();
                    Ok(AppReturn::Continue)
                }

                Action::Quit => Ok(AppReturn::Exit),

                Action::Prev => {
                    self.pop_history();
                    self.current_menu = self.history[self.history.len() - 1].clone();
                    self.update_items();
                    Ok(AppReturn::Continue)
                }

                _ => Ok(AppReturn::Continue),
            },
            None => Ok(AppReturn::Continue),
        }
    }

    pub fn execute_cheat_selection(&mut self, key: Key) -> Result<AppReturn> {
        match self.actions.find(key) {
            Some(action) => match action {
                Action::Enter => match self.input_mode {
                    InputMode::Editing => Ok(AppReturn::Continue),

                    InputMode::Selecting => {
                        match self.items.get(self.table_state.selected().unwrap()) {
                            Some(item) => match item[0] {
                                "Score" => {
                                    self.selected_cheat = Some(Cheat::Score);
                                    self.input_mode = InputMode::Editing;
                                    Ok(AppReturn::Continue)
                                }
                                "Lives" => {
                                    self.selected_cheat = Some(Cheat::Lives);
                                    self.input_mode = InputMode::Editing;
                                    Ok(AppReturn::Continue)
                                }
                                "Bombs" => {
                                    self.selected_cheat = Some(Cheat::Bombs);
                                    self.input_mode = InputMode::Editing;
                                    Ok(AppReturn::Continue)
                                }
                                "Power" => Err(ErrorKind::NotSupported),
                                _ => Ok(AppReturn::Continue),
                            },
                            None => Ok(AppReturn::Continue),
                        }
                    }
                },

                Action::Up => {
                    self.previous();
                    Ok(AppReturn::Continue)
                }

                Action::Down => {
                    self.next();
                    Ok(AppReturn::Continue)
                }

                Action::Quit => Ok(AppReturn::Exit),

                Action::Prev => {
                    self.pop_history();
                    self.current_menu = self.history[self.history.len() - 1].clone();
                    self.update_items();
                    Ok(AppReturn::Continue)
                }
                _ => Ok(AppReturn::Continue),
            },
            None => Ok(AppReturn::Continue),
        }
    }

    pub fn execute_input(&mut self) -> Result<AppReturn> {
        let current_game = self.current_game.as_ref().unwrap();

        let pid = get_pid_by_name(&current_game.process_name).unwrap();

        match &self.selected_cheat {
            Some(cheat) => match cheat {
                Cheat::Score => {
                    write_mem_value(pid, current_game.score_mem_addr, self.input.parse::<i32>()?)?;
                    self.status = AppStatus::Success("Score updated!".to_string());
                    self.input_mode = InputMode::Selecting;
                    Ok(AppReturn::Continue)
                }
                Cheat::Lives => {
                    write_mem_value(pid, current_game.live_mem_addr, self.input.parse::<i32>()?)?;
                    self.status = AppStatus::Success("Lives updated!".to_string());
                    self.input_mode = InputMode::Selecting;
                    Ok(AppReturn::Continue)
                }
                Cheat::Bombs => {
                    write_mem_value(pid, current_game.bomb_mem_addr, self.input.parse::<i32>()?)?;
                    self.status = AppStatus::Success("Bombs updated!".to_string());
                    self.input_mode = InputMode::Selecting;
                    Ok(AppReturn::Continue)
                }
                Cheat::Power => Err(ErrorKind::NotSupported),
            },
            None => Ok(AppReturn::Continue),
        }
    }

    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn tick(&mut self) -> Result<AppReturn> {
        Ok(AppReturn::Continue)
    }
}
