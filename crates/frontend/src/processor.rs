use bridge::{handle::BackendHandle, message::MessageToFrontend};
use gpui::{AnyWindowHandle, App, SharedString, Window};
use gpui_component::{
    WindowExt,
    notification::{Notification, NotificationType},
};

pub struct Processor {
    backend_handle: BackendHandle,
    main_window_handle: Option<AnyWindowHandle>,
    waiting_for_window: Vec<MessageToFrontend>,
}

impl Processor {
    pub fn new(backend_handle: BackendHandle) -> Self {
        Self {
            backend_handle,
            main_window_handle: None,
            waiting_for_window: Vec::new(),
        }
    }

    pub fn set_main_window_handle(&mut self, window: AnyWindowHandle, cx: &mut App) {
        self.main_window_handle = Some(window);
        self.process_messages_waiting_for_window(cx);
    }

    pub fn process_messages_waiting_for_window(&mut self, cx: &mut App) {
        for message in std::mem::take(&mut self.waiting_for_window) {
            self.process(message, cx);
        }
    }

    #[inline(always)]
    pub fn with_main_window(
        &mut self,
        message: MessageToFrontend,
        cx: &mut App,
        func: impl FnOnce(&mut Processor, MessageToFrontend, &mut Window, &mut App),
    ) {
        let Some(handle) = self.main_window_handle else {
            self.waiting_for_window.push(message);
            return;
        };

        _ = handle.update(cx, |_, window, cx| {
            (func)(self, message, window, cx);
        });
    }

    pub fn process(&mut self, message: MessageToFrontend, cx: &mut App) {
        match message {
            MessageToFrontend::AddNotification { .. } => {
                self.with_main_window(message, cx, |_, message, window, cx| {
                    let MessageToFrontend::AddNotification {
                        notification_type,
                        message,
                    } = message;

                    let notification_type = match notification_type {
                        bridge::message::NotificationType::Error => NotificationType::Error,
                        bridge::message::NotificationType::Success => NotificationType::Success,
                        bridge::message::NotificationType::Info => NotificationType::Info,
                        bridge::message::NotificationType::Warning => NotificationType::Warning,
                    };

                    let mut notification: Notification =
                        (notification_type, SharedString::from(message)).into();
                    if let NotificationType::Error = notification_type {
                        notification = notification.autohide(false);
                    }

                    window.push_notification(notification, cx);
                });
            }
        }
    }
}
