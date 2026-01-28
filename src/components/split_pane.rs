use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

const MIN_PANE_SIZE: f32 = 50.0;
const DIVIDER_SIZE: Pixels = px(4.0);
const DIVIDER_HIT_AREA: Pixels = px(8.0);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub enum SplitPaneEvent {
    Resized { ratio: f32 },
    PaneCollapsed { pane: CollapsiblePane },
    PaneExpanded { pane: CollapsiblePane },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CollapsiblePane {
    First,
    Second,
}

pub struct SplitPaneState {
    ratio: f32,
    direction: SplitDirection,
    min_first: f32,
    max_first: f32,
    min_second: f32,
    max_second: f32,
    is_dragging: bool,
    collapsed_pane: Option<CollapsiblePane>,
    ratio_before_collapse: f32,
    bounds: Bounds<Pixels>,
    focus_handle: FocusHandle,
}

impl SplitPaneState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            ratio: 0.5,
            direction: SplitDirection::Horizontal,
            min_first: MIN_PANE_SIZE,
            max_first: f32::MAX,
            min_second: MIN_PANE_SIZE,
            max_second: f32::MAX,
            is_dragging: false,
            collapsed_pane: None,
            ratio_before_collapse: 0.5,
            bounds: Bounds::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn set_ratio(&mut self, ratio: f32, cx: &mut Context<Self>) {
        let clamped = self.clamp_ratio(ratio);
        if (self.ratio - clamped).abs() > f32::EPSILON {
            self.ratio = clamped;
            cx.notify();
        }
    }

    pub fn direction(&self) -> SplitDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        self.direction = direction;
        cx.notify();
    }

    pub fn set_min_first(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min_first = min.max(0.0);
        self.ratio = self.clamp_ratio(self.ratio);
        cx.notify();
    }

    pub fn set_max_first(&mut self, max: f32, cx: &mut Context<Self>) {
        self.max_first = max.max(self.min_first);
        self.ratio = self.clamp_ratio(self.ratio);
        cx.notify();
    }

    pub fn set_min_second(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min_second = min.max(0.0);
        self.ratio = self.clamp_ratio(self.ratio);
        cx.notify();
    }

    pub fn set_max_second(&mut self, max: f32, cx: &mut Context<Self>) {
        self.max_second = max.max(self.min_second);
        self.ratio = self.clamp_ratio(self.ratio);
        cx.notify();
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed_pane.is_some()
    }

    pub fn collapsed_pane(&self) -> Option<CollapsiblePane> {
        self.collapsed_pane
    }

    pub fn collapse(&mut self, pane: CollapsiblePane, cx: &mut Context<Self>) {
        if self.collapsed_pane.is_some() {
            return;
        }
        self.ratio_before_collapse = self.ratio;
        self.collapsed_pane = Some(pane);
        cx.emit(SplitPaneEvent::PaneCollapsed { pane });
        cx.notify();
    }

    pub fn expand(&mut self, cx: &mut Context<Self>) {
        if let Some(pane) = self.collapsed_pane.take() {
            self.ratio = self.ratio_before_collapse;
            cx.emit(SplitPaneEvent::PaneExpanded { pane });
            cx.notify();
        }
    }

    pub fn toggle_collapse(&mut self, pane: CollapsiblePane, cx: &mut Context<Self>) {
        if self.collapsed_pane == Some(pane) {
            self.expand(cx);
        } else {
            self.collapse(pane, cx);
        }
    }

    fn clamp_ratio(&self, ratio: f32) -> f32 {
        let total_size = self.total_size();
        if total_size <= px(0.0) {
            return ratio.clamp(0.0, 1.0);
        }

        let min_first_ratio = (px(self.min_first) / total_size).clamp(0.0, 1.0);
        let max_first_ratio = (px(self.max_first) / total_size).clamp(0.0, 1.0);
        let min_second_ratio = (px(self.min_second) / total_size).clamp(0.0, 1.0);
        let max_second_ratio = (px(self.max_second) / total_size).clamp(0.0, 1.0);

        let lower_bound = min_first_ratio.max(1.0 - max_second_ratio);
        let upper_bound = max_first_ratio.min(1.0 - min_second_ratio);

        ratio.clamp(lower_bound.min(upper_bound), upper_bound.max(lower_bound))
    }

    fn total_size(&self) -> Pixels {
        match self.direction {
            SplitDirection::Horizontal => self.bounds.size.width,
            SplitDirection::Vertical => self.bounds.size.height,
        }
    }

    fn update_from_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let total_size = self.total_size();
        if total_size <= px(0.0) {
            return;
        }

        let relative_pos = match self.direction {
            SplitDirection::Horizontal => position.x - self.bounds.left(),
            SplitDirection::Vertical => position.y - self.bounds.top(),
        };

        let new_ratio = (relative_pos / total_size).clamp(0.0, 1.0);
        let old_ratio = self.ratio;
        self.set_ratio(new_ratio, cx);

        if (self.ratio - old_ratio).abs() > f32::EPSILON {
            cx.emit(SplitPaneEvent::Resized { ratio: self.ratio });
        }
    }
}

impl Focusable for SplitPaneState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<SplitPaneEvent> for SplitPaneState {}

impl Render for SplitPaneState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct SplitPane {
    state: Entity<SplitPaneState>,
    first_child: Option<AnyElement>,
    second_child: Option<AnyElement>,
    direction: Option<SplitDirection>,
    show_collapse_buttons: bool,
    on_resize: Option<Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl SplitPane {
    pub fn new(state: Entity<SplitPaneState>) -> Self {
        Self {
            state,
            first_child: None,
            second_child: None,
            direction: None,
            show_collapse_buttons: false,
            on_resize: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn horizontal(state: Entity<SplitPaneState>) -> Self {
        Self::new(state).direction(SplitDirection::Horizontal)
    }

    pub fn vertical(state: Entity<SplitPaneState>) -> Self {
        Self::new(state).direction(SplitDirection::Vertical)
    }

    pub fn direction(mut self, direction: SplitDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn first(mut self, child: impl IntoElement) -> Self {
        self.first_child = Some(child.into_any_element());
        self
    }

    pub fn second(mut self, child: impl IntoElement) -> Self {
        self.second_child = Some(child.into_any_element());
        self
    }

    pub fn show_collapse_buttons(mut self, show: bool) -> Self {
        self.show_collapse_buttons = show;
        self
    }

    pub fn on_resize(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_resize = Some(Rc::new(handler));
        self
    }
}

impl Styled for SplitPane {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SplitPane {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let direction = self.direction.unwrap_or(state.direction);
        let ratio = state.ratio;
        let collapsed_pane = state.collapsed_pane;
        let user_style = self.style;

        let (first_size, second_size) = match collapsed_pane {
            Some(CollapsiblePane::First) => (relative(0.0), relative(1.0)),
            Some(CollapsiblePane::Second) => (relative(1.0), relative(0.0)),
            None => (relative(ratio), relative(1.0 - ratio)),
        };

        let state_for_canvas = self.state.clone();
        let state_for_drag = self.state.clone();
        let state_for_move = self.state.clone();
        let state_for_up = self.state.clone();
        let on_resize_drag = self.on_resize.clone();
        let on_resize_move = self.on_resize.clone();

        let is_horizontal = direction == SplitDirection::Horizontal;
        let is_dragging = state.is_dragging;

        let divider = div()
            .id("split-pane-divider")
            .flex_shrink_0()
            .bg(theme.tokens.border)
            .when(is_horizontal, |this| {
                this.w(DIVIDER_SIZE)
                    .h_full()
                    .cursor_col_resize()
                    .px((DIVIDER_HIT_AREA - DIVIDER_SIZE) / 2.0)
            })
            .when(!is_horizontal, |this| {
                this.h(DIVIDER_SIZE)
                    .w_full()
                    .cursor_row_resize()
                    .py((DIVIDER_HIT_AREA - DIVIDER_SIZE) / 2.0)
            })
            .when(collapsed_pane.is_none(), |this| {
                this.hover(|s| s.bg(theme.tokens.accent))
            })
            .when(is_dragging, |this| this.bg(theme.tokens.accent))
            .when(collapsed_pane.is_none(), |this| {
                this.on_mouse_down(
                    MouseButton::Left,
                    window.listener_for(
                        &state_for_drag,
                        move |state, e: &MouseDownEvent, window, cx| {
                            state.is_dragging = true;
                            state.update_from_position(e.position, cx);

                            if let Some(ref handler) = on_resize_drag {
                                handler(state.ratio, window, cx);
                            }
                        },
                    ),
                )
            });

        let container_mouse_move = window.listener_for(
            &state_for_move,
            move |state, e: &MouseMoveEvent, window, cx| {
                if state.is_dragging {
                    state.update_from_position(e.position, cx);

                    if let Some(ref handler) = on_resize_move {
                        handler(state.ratio, window, cx);
                    }
                }
            },
        );

        let container_mouse_up =
            window.listener_for(&state_for_up, move |state, _: &MouseUpEvent, _, _cx| {
                state.is_dragging = false;
            });

        let mut container = div()
            .flex()
            .size_full()
            .overflow_hidden()
            .when(is_horizontal, |this| this.flex_row())
            .when(!is_horizontal, |this| this.flex_col())
            .on_mouse_move(container_mouse_move)
            .on_mouse_up(MouseButton::Left, container_mouse_up);

        container = container.map(|mut this| {
            this.style().refine(&user_style);
            this
        });

        let is_first_collapsed = collapsed_pane == Some(CollapsiblePane::First);
        let is_second_collapsed = collapsed_pane == Some(CollapsiblePane::Second);

        let first_pane = div()
            .flex_shrink_0()
            .overflow_hidden()
            .when(is_horizontal && !is_first_collapsed, |this| {
                this.h_full().w(first_size)
            })
            .when(!is_horizontal && !is_first_collapsed, |this| {
                this.w_full().h(first_size)
            })
            .when(is_first_collapsed, |this| this.size_0())
            .when_some(self.first_child, |this, child| this.child(child));

        let second_pane = div()
            .flex_shrink_0()
            .overflow_hidden()
            .when(is_horizontal && !is_second_collapsed, |this| {
                this.h_full().w(second_size)
            })
            .when(!is_horizontal && !is_second_collapsed, |this| {
                this.w_full().h(second_size)
            })
            .when(is_second_collapsed, |this| this.size_0())
            .when_some(self.second_child, |this, child| this.child(child));

        let collapse_buttons = if self.show_collapse_buttons && collapsed_pane.is_none() {
            let state_first = self.state.clone();
            let state_second = self.state.clone();

            Some(
                div()
                    .absolute()
                    .when(is_horizontal, |this| {
                        this.top(px(4.0))
                            .left(relative(ratio))
                            .ml(-px(8.0))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                    })
                    .when(!is_horizontal, |this| {
                        this.left(px(4.0))
                            .top(relative(ratio))
                            .mt(-px(8.0))
                            .flex()
                            .flex_row()
                            .gap(px(2.0))
                    })
                    .child(
                        div()
                            .id("collapse-first")
                            .size(px(16.0))
                            .rounded_full()
                            .bg(theme.tokens.muted)
                            .hover(|s| s.bg(theme.tokens.accent))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(10.0))
                            .text_color(theme.tokens.muted_foreground)
                            .when(is_horizontal, |this| this.child("<"))
                            .when(!is_horizontal, |this| this.child("^"))
                            .on_click(move |_, _, cx| {
                                state_first.update(cx, |state, cx| {
                                    state.collapse(CollapsiblePane::First, cx);
                                });
                            }),
                    )
                    .child(
                        div()
                            .id("collapse-second")
                            .size(px(16.0))
                            .rounded_full()
                            .bg(theme.tokens.muted)
                            .hover(|s| s.bg(theme.tokens.accent))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(10.0))
                            .text_color(theme.tokens.muted_foreground)
                            .when(is_horizontal, |this| this.child(">"))
                            .when(!is_horizontal, |this| this.child("v"))
                            .on_click(move |_, _, cx| {
                                state_second.update(cx, |state, cx| {
                                    state.collapse(CollapsiblePane::Second, cx);
                                });
                            }),
                    ),
            )
        } else if self.show_collapse_buttons && collapsed_pane.is_some() {
            let state_expand = self.state.clone();

            Some(
                div()
                    .absolute()
                    .when(is_horizontal, |this| {
                        if collapsed_pane == Some(CollapsiblePane::First) {
                            this.top(px(4.0)).left(px(4.0))
                        } else {
                            this.top(px(4.0)).right(px(4.0))
                        }
                    })
                    .when(!is_horizontal, |this| {
                        if collapsed_pane == Some(CollapsiblePane::First) {
                            this.left(px(4.0)).top(px(4.0))
                        } else {
                            this.left(px(4.0)).bottom(px(4.0))
                        }
                    })
                    .child(
                        div()
                            .id("expand-pane")
                            .size(px(20.0))
                            .rounded_full()
                            .bg(theme.tokens.accent)
                            .hover(|s| s.bg(theme.tokens.primary))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .when(
                                is_horizontal && collapsed_pane == Some(CollapsiblePane::First),
                                |this| this.child(">"),
                            )
                            .when(
                                is_horizontal && collapsed_pane == Some(CollapsiblePane::Second),
                                |this| this.child("<"),
                            )
                            .when(
                                !is_horizontal && collapsed_pane == Some(CollapsiblePane::First),
                                |this| this.child("v"),
                            )
                            .when(
                                !is_horizontal && collapsed_pane == Some(CollapsiblePane::Second),
                                |this| this.child("^"),
                            )
                            .on_click(move |_, _, cx| {
                                state_expand.update(cx, |state, cx| {
                                    state.expand(cx);
                                });
                            }),
                    ),
            )
        } else {
            None
        };

        container
            .relative()
            .child(first_pane)
            .when(collapsed_pane.is_none(), |this| this.child(divider))
            .child(second_pane)
            .when_some(collapse_buttons, |this, buttons| this.child(buttons))
            .child(
                canvas(
                    move |bounds, _, cx| {
                        state_for_canvas.update(cx, |state, _| {
                            state.bounds = bounds;
                        });
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
    }
}
