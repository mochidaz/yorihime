use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::Game;

#[derive(Serialize, Deserialize)]
pub struct Config {
    games: HashMap<String, Game>,
}

impl Config {
    pub fn new() -> Config {
        let cfg = include_str!("../resources/address_db.json");

        let games: HashMap<String, Game> = serde_json::from_str(cfg).unwrap();

        Config { games }
    }

    pub fn get_game(&self, name: &str) -> Option<&Game> {
        self.games.get(name)
    }
}
