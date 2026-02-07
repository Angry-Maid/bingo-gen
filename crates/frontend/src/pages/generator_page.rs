use std::collections::HashMap;

use bridge::{
    card::{BingoSyncCard, LockoutLiveBoard, LockoutLiveCard},
    handle::BackendHandle,
    message::MessageToBackend,
};
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    Disableable, Icon, Selectable, Sizable, WindowExt,
    button::{Button, ButtonGroup},
    divider::Divider,
    form::{field, v_form},
    input::{Input, InputState},
    notification::NotificationType,
    red_600, red_800, v_flex,
};
use itertools::izip;
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, EnumIter, FromRepr)]
#[repr(usize)]
enum GridSize {
    Size3 = 3,
    Size4,
    #[default]
    Size5,
    Size6,
    Size7,
    Size8,
    Size9,
}

pub struct GeneratorPage {
    focus_handle: FocusHandle,
    backend_handle: BackendHandle,
    cell_inputs: [Entity<InputState>; 9 * 9],
    selected_grid_size: GridSize,
    trigger_clear: bool,
    save_bingosync: bool,
    save_lockout: bool,
}

impl GeneratorPage {
    pub fn new(
        backend_handle: BackendHandle,
        mut window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            backend_handle,
            cell_inputs: core::array::from_fn(|_idx| {
                cx.new(|cx| InputState::new(window, cx).auto_grow(2, 2))
            }),
            selected_grid_size: Default::default(),
            trigger_clear: false,
            save_bingosync: false,
            save_lockout: false,
        }
    }
}

impl Focusable for GeneratorPage {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for GeneratorPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.save_bingosync {
            let v: Vec<String> = self
                .cell_inputs
                .iter()
                .enumerate()
                .filter_map(|(idx, i)| {
                    let border = (9 - self.selected_grid_size as usize).div_euclid(2);
                    let even = (self.selected_grid_size as usize + 1) % 2;
                    let x = idx % 9;
                    let y = idx.div_euclid(9);

                    if x >= border + even && x < 9 - border && y >= border + even && y < 9 - border
                    {
                        Some(i.read(cx).value().to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let mut start: Vec<String> = vec![];
            let mut fill: Vec<String> = vec![];
            let mut end: Vec<String> = vec![];

            let mut res: Vec<String> = vec![];

            match self.selected_grid_size {
                GridSize::Size3 => {
                    start = vec![" ".to_string(); 6];
                    fill = vec![" ".to_string(); 2];
                    end = vec![" ".to_string(); 4];
                }
                GridSize::Size4 => {
                    start = vec![" ".to_string(); 0];
                    fill = vec![" ".to_string(); 1];
                    end = vec![" ".to_string(); 5];
                }
                _ => {}
            }

            res.extend(start);

            for chunk in v.chunks_exact(self.selected_grid_size as usize) {
                res.extend(chunk.to_vec());
                res.extend(fill.clone());
            }

            res.extend(end);

            let data = res
                .iter()
                .map(|v| BingoSyncCard { name: v.to_owned() })
                .collect::<Vec<BingoSyncCard>>();

            self.backend_handle
                .send(MessageToBackend::CreateBingoSyncFile { data });

            self.save_bingosync = false;
        }

        if self.save_lockout {
            let v: Vec<String> = self
                .cell_inputs
                .iter()
                .enumerate()
                .filter_map(|(idx, i)| {
                    let border = (9 - self.selected_grid_size as usize).div_euclid(2);
                    let even = (self.selected_grid_size as usize + 1) % 2;
                    let x = idx % 9;
                    let y = idx.div_euclid(9);

                    if x >= border + even && x < 9 - border && y >= border + even && y < 9 - border
                    {
                        Some(i.read(cx).value().to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let objectives = v
                .iter()
                .enumerate()
                .map(|(idx, v)| LockoutLiveCard::new(v.to_owned(), idx + 1))
                .collect::<Vec<LockoutLiveCard>>();

            if objectives.iter().any(|v| v.goal.len() > 60) {
                window.push_notification(
                    (
                        NotificationType::Warning,
                        "Lockout Live can't have a task text longer than 60 characters.",
                    ),
                    cx,
                );
            } else {
                let data = LockoutLiveBoard {
                    version: "0.0.0".to_string(),
                    game: "None".to_string(),
                    objectives,
                    limits: HashMap::from([
                        ("board".to_string(), HashMap::default()),
                        ("line".to_string(), HashMap::default()),
                    ]),
                };

                self.backend_handle
                    .send(MessageToBackend::CreateLockoutLiveFile { data });
            }

            self.save_lockout = false;
        }

        if self.trigger_clear {
            self.cell_inputs
                .iter()
                .for_each(|e| e.update(cx, |is, cx| is.set_value("", window, cx)));
            self.focus_handle.focus(window);
            self.trigger_clear = false;
        }

        v_flex()
            .gap_4()
            .p_4()
            .size_full()
            .child(
                v_form()
                    .layout(gpui::Axis::Horizontal)
                    .columns(2)
                    .child(
                        field().label("Grid Size").child(
                            div().child(
                                ButtonGroup::new("selected-grid-size")
                                    .outline()
                                    .compact()
                                    .children(
                                        izip!(
                                            GridSize::iter(),
                                            [
                                                "size3-grid-btn",
                                                "size4-grid-btn",
                                                "size5-grid-btn",
                                                "size6-grid-btn",
                                                "size7-grid-btn",
                                                "size8-grid-btn",
                                                "size9-grid-btn",
                                            ],
                                            ["3x3", "4x4", "5x5", "6x6", "7x7", "8x8", "9x9"],
                                        )
                                        .map(
                                            |(el, name, label)| {
                                                Button::new(name)
                                                    .label(label)
                                                    .selected(self.selected_grid_size == el)
                                            },
                                        ),
                                    )
                                    .on_click(cx.listener(|view, selected: &Vec<usize>, _, cx| {
                                        if let Some(&v) = selected.first() {
                                            if v > 6 {
                                                return;
                                            }
                                            if !(v + 3 == view.selected_grid_size as usize) {
                                                view.trigger_clear = true;
                                            }
                                            view.selected_grid_size =
                                                GridSize::from_repr(v + 3).unwrap();
                                        }

                                        cx.notify();
                                    })),
                            ),
                        ),
                    )
                    .child(
                        field().col_start(2).label("Actions").child(
                            div().child(
                                ButtonGroup::new("board-actions")
                                    .outline()
                                    .compact()
                                    .children(
                                        izip!(
                                            [
                                                "randomize-btn",
                                                "clear-btn",
                                                "save-bingosync-btn",
                                                "save-lockout-btn"
                                            ],
                                            ["Randomize", "Clear", "Bingosync", "Lockout Live",],
                                            [
                                                Some(
                                                    Icon::new(Icon::empty())
                                                        .path("icons/dices.svg")
                                                ),
                                                Some(
                                                    Icon::new(Icon::empty())
                                                        .path("icons/eraser.svg")
                                                ),
                                                Some(
                                                    Icon::new(Icon::empty()).path("icons/save.svg")
                                                ),
                                                Some(
                                                    Icon::new(Icon::empty()).path("icons/save.svg")
                                                ),
                                            ],
                                            [
                                                true,
                                                false,
                                                self.selected_grid_size as usize > 5,
                                                false,
                                            ],
                                        )
                                        .map(
                                            |(name, label, icon, disabled)| {
                                                let btn = Button::new(name)
                                                    .label(label)
                                                    .disabled(disabled);

                                                if let Some(i) = icon {
                                                    return btn.icon(i);
                                                }

                                                btn
                                            },
                                        ),
                                    )
                                    .on_click(cx.listener(
                                        |view, selected: &Vec<usize>, _w, cx| {
                                            match selected.first() {
                                                Some(0) => randomize_action(),
                                                Some(1) => view.trigger_clear = true,
                                                Some(2) => view.save_bingosync = true,
                                                Some(3) => view.save_lockout = true,
                                                _ => {
                                                    return;
                                                }
                                            }

                                            cx.notify();
                                        },
                                    )),
                            ),
                        ),
                    ),
            )
            .child(Divider::horizontal().gap_4())
            .child(
                div()
                    .gap_2()
                    .grid()
                    .size_full()
                    .grid_cols(9)
                    .grid_rows(9)
                    .children(self.cell_inputs.iter().enumerate().map(|(idx, i)| {
                        let border = (9 - self.selected_grid_size as usize).div_euclid(2);
                        let even = (self.selected_grid_size as usize + 1) % 2;
                        let x = idx % 9;
                        let y = idx.div_euclid(9);

                        Input::new(i)
                            .when(
                                i.read(cx).value().to_string().len() > 60
                                    && i.focus_handle(cx).is_focused(window),
                                |this| this.border_color(red_600()),
                            )
                            .when(
                                i.read(cx).value().to_string().len() > 60
                                    && !i.focus_handle(cx).is_focused(window),
                                |this| this.border_color(red_800()),
                            )
                            .with_size(px(24.))
                            .disabled(
                                !(x >= border + even
                                    && x < 9 - border
                                    && y >= border + even
                                    && y < 9 - border),
                            )
                            .border_4()
                    })),
            )
    }
}

fn randomize_action() {
    log::info!("randomize_action");
}
