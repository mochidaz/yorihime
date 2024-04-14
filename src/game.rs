use serde::{Deserialize, Serialize};

use crate::utils::deserialize_usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub name: String,
    pub process_name: String,
    pub alternate_names: Vec<String>,
    #[serde(deserialize_with = "deserialize_usize")]
    pub score_mem_addr: usize,
    #[serde(deserialize_with = "deserialize_usize")]
    pub live_mem_addr: usize,
    #[serde(deserialize_with = "deserialize_usize")]
    pub bomb_mem_addr: usize,
}

impl Game {
    pub fn new(name: &str, process_name: &str, alternate_names: Vec<String>, score_mem_addr: usize, live_mem_addr: usize, bomb_mem_addr: usize) -> Game {
        Game {
            name: name.to_string(),
            process_name: process_name.to_string(),
            alternate_names,
            score_mem_addr,
            live_mem_addr,
            bomb_mem_addr,
        }
    }
}

