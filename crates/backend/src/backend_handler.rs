use bridge::message::MessageToBackend;

use crate::backend::BackendState;

impl BackendState {
    pub async fn handle_message(&self, message: MessageToBackend) {
        match message {
            MessageToBackend::CreateBingoSyncFile { data } => {
                match self.export_bingo_sync(data).await {
                    Ok(f) => self.send.send_success(format!("Created file '{}'", f)),
                    Err(e) => self.send.send_error(format!("Error: '{}'", e)),
                }
            }
            MessageToBackend::CreateLockoutLiveFile { data } => {
                match self.export_lockout_live(data).await {
                    Ok(f) => self.send.send_success(format!("Created file '{}'", f)),
                    Err(e) => self.send.send_error(format!("Error: '{}'", e)),
                }
            }
        };
    }
}
