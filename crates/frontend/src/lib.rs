use std::borrow::Cow;

use bridge::handle::{BackendHandle, FrontendReceiver};
use gpui::{
    App, AppContext, Application, AssetSource, Bounds, Result, SharedString, WindowBounds,
    WindowOptions, px, size,
};
use gpui_component::{Root, ThemeMode, TitleBar};

use crate::{
    processor::Processor,
    root::{AppRoot, AppRootGlobal},
};

pub mod entity;
pub mod pages;
pub mod processor;
pub mod root;
pub mod ui;

#[derive(rust_embed::RustEmbed)]
#[folder = "../../assets"]
#[include = "icons/**/*.svg"]
#[include = "fonts/**/*.ttf"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow::anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

pub fn start(title: &str, backend_handle: BackendHandle, mut recv: FrontendReceiver) {
    let title = SharedString::from(title.to_string());

    Application::new()
        .with_assets(Assets)
        .run(move |cx: &mut App| {
            gpui_component::init(cx);
            gpui_component::Theme::change(ThemeMode::Dark, None, cx);

            let theme = gpui_component::Theme::global_mut(cx);
            theme.scrollbar_show = gpui_component::scroll::ScrollbarShow::Always;

            let mut window_size = size(px(1600.0), px(1200.0));

            if let Some(display) = cx.primary_display() {
                let display_size = display.bounds().size;
                window_size.width = window_size.width.min(display_size.width * 0.85);
                window_size.height = window_size.height.min(display_size.height * 0.85);
            }

            let window_bounds = Bounds::centered(None, window_size, cx);

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            let window = cx
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                        window_min_size: Some(size(px(480.), px(320.))),
                        titlebar: Some(TitleBar::title_bar_options()),
                        kind: gpui::WindowKind::Normal,
                        ..Default::default()
                    },
                    |window, cx| {
                        let root = cx.new(|cx| {
                            AppRoot::new(title.clone(), backend_handle.clone(), window, cx)
                        });

                        cx.set_global(AppRootGlobal { root: root.clone() });

                        cx.new(|cx| Root::new(root, window, cx))
                    },
                )
                .expect("failed to open window");

            window
                .update(cx, |_, window, _| {
                    window.activate_window();
                    window.set_window_title(&title);
                })
                .expect("failed to update window");

            cx.activate(true);

            let mut processor = Processor::new(backend_handle.clone());

            while let Some(message) = recv.try_recv() {
                processor.process(message, cx);
            }

            processor.set_main_window_handle(window.into(), cx);

            cx.spawn(async move |cx| {
                while let Some(message) = recv.recv().await {
                    _ = cx.update(|cx| {
                        processor.process(message, cx);
                    });
                }
            })
            .detach();
        });
}
