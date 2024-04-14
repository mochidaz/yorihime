use core::slice::Iter;
use std::fmt;
use std::fmt::Display;

use crate::inputs::key::Key;

// app/actions.rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Enter,
    Up,
    Down,
    Quit,
    Prev,
    Next,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 6] = [
            Action::Quit,
            Action::Enter,
            Action::Up,
            Action::Down,
            Action::Prev,
            Action::Next,
        ];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Enter => &[Key::Enter],
            Action::Up => &[Key::Up],
            Action::Down => &[Key::Down],
            Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
            Action::Prev => &[Key::Left],
            Action::Next => &[Key::Right],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Enter => write!(f, "Enter"),
            Action::Up => write!(f, "Up"),
            Action::Down => write!(f, "Down"),
            Action::Quit => write!(f, "Quit"),
            Action::Prev => write!(f, "Prev"),
            Action::Next => write!(f, "Next"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: Key) -> Option<&Action> {
        self.0.iter().find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        Self(actions)
    }
}
