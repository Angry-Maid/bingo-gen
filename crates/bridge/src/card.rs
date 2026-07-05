use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BingoSyncCard {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LockoutLiveCard {
    pub goal: String,
    pub forced_positions: Vec<usize>,
    range: Vec<String>,
    board_categories: Vec<String>,
    line_categories: Vec<String>,
    icons: Vec<String>,
    progression: Vec<String>,
}

impl Default for LockoutLiveCard {
    fn default() -> Self {
        Self {
            goal: Default::default(),
            forced_positions: Default::default(),
            range: Default::default(),
            board_categories: Default::default(),
            line_categories: Default::default(),
            icons: Default::default(),
            progression: vec![
                "e".to_owned(),
                "m".to_owned(),
                "l".to_owned(),
                "n".to_owned(),
            ],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LockoutLiveBoard {
    pub schema_version: usize,
    pub schema_mode: String,
    pub game_name: String,
    pub tag_names: Vec<String>,
    pub objectives: Vec<LockoutLiveCard>,
    pub limits: HashMap<String, HashMap<String, usize>>,
}

impl LockoutLiveCard {
    pub fn new(goal: String, forced_positions: Vec<usize>) -> Self {
        Self {
            goal,
            forced_positions,
            ..Default::default()
        }
    }
}
