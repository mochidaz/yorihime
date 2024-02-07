use serde::{Deserialize, Serialize};

use crate::utils::deserialize_usize;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    #[serde(deserialize_with = "deserialize_usize")]
    pub score_mem_addr: usize,
    #[serde(deserialize_with = "deserialize_usize")]
    pub live_mem_addr: usize,
}

impl Game {
    pub fn new(name: &str, score_mem_addr: usize, live_mem_addr: usize) -> Game {
        Game {
            name: name.to_string(),
            score_mem_addr,
            live_mem_addr,
        }
    }
}

