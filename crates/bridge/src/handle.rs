use std::sync::Arc;

#[cfg(debug_assertions)]
use tokio::sync::mpsc::{Receiver, Sender};

#[cfg(not(debug_assertions))]
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::message::{MessageToBackend, MessageToFrontend, NotificationType};

pub fn create_pair() -> (
    BackendReceiver,
    BackendHandle,
    FrontendReceiver,
    FrontendHandle,
) {
    #[cfg(debug_assertions)]
    let (frontend_send, frontend_recv) = tokio::sync::mpsc::channel(64);
    #[cfg(debug_assertions)]
    let (backend_send, backend_recv) = tokio::sync::mpsc::channel(64);

    #[cfg(not(debug_assertions))]
    let (frontend_send, frontend_recv) = tokio::sync::mpsc::unbounded_channel();
    #[cfg(not(debug_assertions))]
    let (backend_send, backend_recv) = tokio::sync::mpsc::unbounded_channel();

    (
        BackendReceiver {
            receiver: backend_recv,
        },
        BackendHandle {
            sender: backend_send,
        },
        FrontendReceiver {
            receiver: frontend_recv,
        },
        FrontendHandle {
            sender: frontend_send,
        },
    )
}

#[derive(Debug)]
pub struct BackendReceiver {
    #[cfg(debug_assertions)]
    receiver: Receiver<MessageToBackend>,
    #[cfg(not(debug_assertions))]
    receiver: UnboundedReceiver<MessageToBackend>,
}

impl BackendReceiver {
    pub async fn recv(&mut self) -> Option<MessageToBackend> {
        let message = self.receiver.recv().await?;

        Some(message)
    }
}

#[derive(Debug)]
pub struct FrontendReceiver {
    #[cfg(debug_assertions)]
    receiver: Receiver<MessageToFrontend>,
    #[cfg(not(debug_assertions))]
    receiver: UnboundedReceiver<MessageToFrontend>,
}

impl FrontendReceiver {
    pub async fn recv(&mut self) -> Option<MessageToFrontend> {
        let message = self.receiver.recv().await?;

        Some(message)
    }

    pub fn try_recv(&mut self) -> Option<MessageToFrontend> {
        let message = self.receiver.try_recv().ok()?;

        Some(message)
    }
}

#[derive(Clone, Debug)]
pub struct BackendHandle {
    #[cfg(debug_assertions)]
    sender: Sender<MessageToBackend>,
    #[cfg(not(debug_assertions))]
    sender: UnboundedSender<MessageToBackend>,
}

unsafe impl Send for BackendHandle {}
unsafe impl Sync for BackendHandle {}

impl BackendHandle {
    pub fn send(&self, message: MessageToBackend) {
        #[cfg(debug_assertions)]
        self.sender.try_send(message).unwrap();
        #[cfg(not(debug_assertions))]
        let _ = self.sender.send(message);
    }
}

#[derive(Debug, Clone)]
pub struct FrontendHandle {
    #[cfg(debug_assertions)]
    sender: Sender<MessageToFrontend>,
    #[cfg(not(debug_assertions))]
    sender: UnboundedSender<MessageToFrontend>,
}

unsafe impl Send for FrontendHandle {}
unsafe impl Sync for FrontendHandle {}

impl FrontendHandle {
    pub fn send(&self, message: MessageToFrontend) {
        #[cfg(debug_assertions)]
        if let Err(tokio::sync::mpsc::error::TrySendError::Full(v)) = self.sender.try_send(message)
        {
            panic!("Sender is full, unable to send message: {v:?}");
        }
        #[cfg(not(debug_assertions))]
        let _ = self.sender.send(message);
    }

    pub fn send_info(&self, info: impl Into<Arc<str>>) {
        self.send(MessageToFrontend::AddNotification {
            notification_type: NotificationType::Info,
            message: info.into(),
        })
    }

    pub fn send_success(&self, success: impl Into<Arc<str>>) {
        self.send(MessageToFrontend::AddNotification {
            notification_type: NotificationType::Success,
            message: success.into(),
        })
    }

    pub fn send_warning(&self, warning: impl Into<Arc<str>>) {
        self.send(MessageToFrontend::AddNotification {
            notification_type: NotificationType::Warning,
            message: warning.into(),
        })
    }

    pub fn send_error(&self, error: impl Into<Arc<str>>) {
        self.send(MessageToFrontend::AddNotification {
            notification_type: NotificationType::Error,
            message: error.into(),
        })
    }
}
