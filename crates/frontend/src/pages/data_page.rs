use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{h_flex, label::Label, v_flex};

pub struct DataPage {}

impl DataPage {
    pub fn new(mut window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for DataPage {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        v_flex().size_full().child(
            h_flex()
                .flex_wrap()
                .justify_center()
                .items_center()
                .size_full()
                .gap_4()
                .text_xl()
                .child(div().child(Label::new("Data Page"))),
        )
    }
}
