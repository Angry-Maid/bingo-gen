use anyhow::Result;
use bridge::card::{BingoSyncCard, LockoutLiveBoard};
use chrono::Local;

use crate::backend::BackendState;

impl BackendState {
    pub async fn export_bingo_sync(&self, data: Vec<BingoSyncCard>) -> Result<String> {
        let date = Local::now();
        let filename = format!("{}_bingo_sync.json", date.format("%Y-%m-%d_%H-%M-%S"));
        std::fs::write(
            self.export_dir.join(filename.clone()),
            serde_json::to_string_pretty(&data)?,
        )
        .ok();

        Ok(filename)
    }

    pub async fn export_lockout_live(&self, data: LockoutLiveBoard) -> Result<String> {
        let date = Local::now();
        let filename = format!("{}_lockout_live.json", date.format("%Y-%m-%d_%H-%M-%S"));
        std::fs::write(
            self.export_dir.join(filename.clone()),
            serde_json::to_string_pretty(&data)?,
        )
        .ok();

        Ok(filename)
    }
}
