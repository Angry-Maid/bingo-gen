use bridge::handle::BackendHandle;
use gpui::{
    AnyElement, App, AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div,
};
use gpui_component::{
    Sizable, Size,
    tab::{Tab, TabBar},
    v_flex,
};

use crate::pages::{data_page::DataPage, generator_page::GeneratorPage};

pub struct Ui {
    focus_handle: FocusHandle,
    page: MainPage,
    backend_handle: BackendHandle,
}

#[derive(Clone)]
pub enum MainPage {
    Data(Entity<DataPage>),
    Generator(Entity<GeneratorPage>),
}

impl MainPage {
    pub fn into_any_element(self) -> AnyElement {
        match self {
            MainPage::Data(entity) => entity.into_any_element(),
            MainPage::Generator(entity) => entity.into_any_element(),
        }
    }

    pub fn page_type(&self) -> PageType {
        match self {
            MainPage::Data(_) => PageType::Data,
            MainPage::Generator(_) => PageType::Generator,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PageType {
    Data,
    Generator,
}

impl PageType {
    pub fn create(
        self,
        backend_handle: BackendHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> MainPage {
        match self {
            PageType::Data => MainPage::Data(cx.new(|cx| DataPage::new(window, cx))),
            PageType::Generator => {
                MainPage::Generator(cx.new(|cx| GeneratorPage::new(backend_handle, window, cx)))
            }
        }
    }
}

impl Ui {
    pub fn new(backend_handle: BackendHandle, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let generator_page = cx.new(|cx| GeneratorPage::new(backend_handle.clone(), window, cx));

        Self {
            backend_handle,
            focus_handle,
            page: MainPage::Generator(generator_page),
        }
    }

    fn switch_page(&mut self, page_type: PageType, window: &mut Window, cx: &mut Context<Self>) {
        if page_type == self.page.page_type() {
            return;
        }

        self.page = page_type.create(self.backend_handle.clone(), window, cx);
    }
}

impl Render for Ui {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_index = match &self.page {
            MainPage::Generator(_) => 0,
            MainPage::Data(_) => 1,
        };

        v_flex()
            .id("tab-bar-container")
            .size_full()
            .overflow_y_scroll()
            .track_focus(&self.focus_handle)
            .child(
                TabBar::new("bar")
                    .with_size(Size::Large)
                    .prefix(div().w_4())
                    .selected_index(selected_index)
                    .child(Tab::new().label("Generator"))
                    // .child(Tab::new().label("Data"))
                    .on_click(cx.listener(|page, idx, window, cx| {
                        let page_type = match *idx {
                            0 => PageType::Generator,
                            1 => PageType::Data,
                            _ => {
                                return;
                            }
                        };
                        page.switch_page(page_type, window, cx);
                    })),
            )
            .child(self.page.clone().into_any_element())
    }
}
