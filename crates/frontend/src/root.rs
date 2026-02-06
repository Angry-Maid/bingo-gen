use bridge::handle::BackendHandle;
use gpui::{
    AppContext, Context, Entity, Global, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, StatefulInteractiveElement, Styled, Window, div,
};
use gpui_component::{Root, StyledExt, TitleBar, label::Label, v_flex};

use crate::ui::Ui;

pub struct AppRootGlobal {
    pub root: Entity<AppRoot>,
}

impl Global for AppRootGlobal {}

pub struct AppRoot {
    pub ui: Entity<Ui>,
    pub name: SharedString,
    pub backend_handle: BackendHandle,
}

impl AppRoot {
    pub fn new(
        title: impl Into<SharedString>,
        backend_handle: BackendHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let ui = cx.new(|cx| Ui::new(backend_handle.clone(), window, cx));

        Self {
            ui,
            name: title.into(),
            backend_handle,
        }
    }
}

impl Render for AppRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        v_flex()
            .id("root-container")
            .size_full()
            .overflow_y_scroll()
            .child(TitleBar::new().child(Label::new(&self.name).font_semibold()))
            .child(
                div()
                    .size_full()
                    .child(self.ui.clone())
                    .children(sheet_layer)
                    .children(dialog_layer)
                    .children(notification_layer),
            )
    }
}
