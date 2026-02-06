use std::sync::Arc;

use crate::card::{BingoSyncCard, LockoutLiveBoard};

#[derive(Debug)]
pub enum MessageToBackend {
    CreateBingoSyncFile { data: Vec<BingoSyncCard> },
    CreateLockoutLiveFile { data: LockoutLiveBoard },
}

#[derive(Debug)]
pub enum MessageToFrontend {
    AddNotification {
        notification_type: NotificationType,
        message: Arc<str>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationType {
    Success,
    Info,
    Error,
    Warning,
}
