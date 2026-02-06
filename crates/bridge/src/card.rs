use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BingoSyncCard {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LockoutLiveCard {
    pub goal: String,
    pub individual_limit: u32,
    pub preferred_grid_position: usize,
    range: Vec<String>,
    board_categories: Vec<String>,
    line_categories: Vec<String>,
    icons: Vec<String>,
}

impl Default for LockoutLiveCard {
    fn default() -> Self {
        Self {
            goal: Default::default(),
            individual_limit: 1,
            preferred_grid_position: Default::default(),
            range: Default::default(),
            board_categories: Default::default(),
            line_categories: Default::default(),
            icons: Default::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LockoutLiveBoard {
    pub version: String,
    pub game: String,
    pub objectives: Vec<LockoutLiveCard>,
    pub limits: HashMap<String, HashMap<String, usize>>,
}

impl LockoutLiveCard {
    pub fn new(goal: String, preferred_grid_position: usize) -> Self {
        Self {
            goal,
            preferred_grid_position,
            individual_limit: 1,
            range: Default::default(),
            board_categories: Default::default(),
            line_categories: Default::default(),
            icons: Default::default(),
        }
    }
}
