use gpui::{prelude::*, *};
use crate::theme::use_theme;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SliderSize {
    Sm,
    Md,
    Lg,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SliderAxis {
    Horizontal,
    Vertical,
}

impl SliderSize {
    fn track_height(&self) -> Pixels {
        match self {
            SliderSize::Sm => px(2.0),
            SliderSize::Md => px(4.0),
            SliderSize::Lg => px(6.0),
        }
    }

    fn thumb_width(&self) -> Pixels {
        match self {
            SliderSize::Sm => px(16.0),
            SliderSize::Md => px(20.0),
            SliderSize::Lg => px(24.0),
        }
    }

    fn thumb_height(&self) -> Pixels {
        match self {
            SliderSize::Sm => px(12.0),
            SliderSize::Md => px(16.0),
            SliderSize::Lg => px(20.0),
        }
    }
}

pub struct SliderState {
    min: f32,
    max: f32,
    value: f32,
    step: f32,
    focus_handle: FocusHandle,
    is_dragging: bool,
    bounds: Bounds<Pixels>,
}

impl SliderState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            value: 0.0,
            step: 1.0,
            focus_handle: cx.focus_handle(),
            is_dragging: false,
            bounds: Bounds::default(),
        }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn set_min(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min = min;
        self.value = self.value.clamp(self.min, self.max);
        cx.notify();
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn set_max(&mut self, max: f32, cx: &mut Context<Self>) {
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
        cx.notify();
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32, cx: &mut Context<Self>) {
        let clamped = value.clamp(self.min, self.max);
        let stepped = ((clamped / self.step).round() * self.step).clamp(self.min, self.max);

        if (self.value - stepped).abs() > f32::EPSILON {
            self.value = stepped;
            cx.notify();
        }
    }

    pub fn step(&self) -> f32 {
        self.step
    }

    pub fn set_step(&mut self, step: f32, cx: &mut Context<Self>) {
        self.step = step;
        cx.notify();
    }

    fn percentage(&self) -> f32 {
        if self.max == self.min {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    fn update_from_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let track_width = self.bounds.size.width;
        if track_width <= px(0.0) {
            return;
        }

        let relative_x = (position.x - self.bounds.left()).clamp(px(0.0), track_width);
        let percentage = (relative_x / track_width).clamp(0.0, 1.0);
        let new_value = self.min + percentage * (self.max - self.min);

        self.set_value(new_value, cx);
    }

    fn update_from_position_vertical(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let track_height = self.bounds.size.height;
        if track_height <= px(0.0) {
            return;
        }

        let relative_y = (position.y - self.bounds.top()).clamp(px(0.0), track_height);
        let percentage = 1.0 - (relative_y / track_height).clamp(0.0, 1.0);
        let new_value = self.min + percentage * (self.max - self.min);

        self.set_value(new_value, cx);
    }
}

impl Focusable for SliderState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SliderState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct Slider {
    state: Entity<SliderState>,
    size: SliderSize,
    axis: SliderAxis,
    disabled: bool,
    show_value: bool,
    on_change: Option<Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Slider {
    pub fn new(state: Entity<SliderState>) -> Self {
        Self {
            state,
            size: SliderSize::Md,
            axis: SliderAxis::Horizontal,
            disabled: false,
            show_value: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.axis = SliderAxis::Horizontal;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.axis = SliderAxis::Vertical;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Slider {
    fn render_horizontal(
        self,
        window: &mut Window,
        theme: crate::theme::Theme,
        focus_handle: FocusHandle,
        is_focused: bool,
        percentage: f32,
        value: f32,
        track_height: Pixels,
        thumb_width: Pixels,
        thumb_height: Pixels,
        track_bg: Hsla,
        active_bg: Hsla,
        thumb_bg: Hsla,
        focus_ring: BoxShadow,
        user_style: StyleRefinement,
    ) -> Div {
        div()
            .flex()
            .items_center()
            .gap_3()
            .w_full()
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(
                div()
                    .relative()
                    .flex_1()
                    .h(thumb_height)
                    .flex()
                    .items_center()
                    .when(!self.disabled, |this| {
                        this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
                    })
                    .when(is_focused && !self.disabled, |this| {
                        this.shadow(vec![focus_ring])
                    })
                    .rounded(theme.tokens.radius_md)
                    .child(
                        canvas(
                            {
                                let state = self.state.clone();
                                move |bounds, _, cx| {
                                    state.update(cx, |state, _| {
                                        state.bounds = bounds;
                                    });
                                }
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full()
                    )
                    .child(
                        div()
                            .relative()
                            .w_full()
                            .h(track_height)
                            .rounded_full()
                            .bg(track_bg)
                            .overflow_hidden()
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .top_0()
                                    .h_full()
                                    .w(relative(percentage))
                                    .bg(active_bg),
                            ),
                    )
                    .child({
                        let state_clone = self.state.clone();
                        let on_change_thumb = self.on_change.clone();

                        div()
                            .absolute()
                            .left(relative(percentage))
                            .top_0()
                            .ml(-(thumb_width / 2.0))
                            .w(thumb_width)
                            .h(thumb_height)
                            .rounded(thumb_height / 2.0)
                            .bg(thumb_bg)
                            .border_2()
                            .border_color(theme.tokens.background)
                            .when(!self.disabled, |this| {
                                this.shadow(vec![theme.tokens.shadow_sm])
                                    .cursor(CursorStyle::PointingHand)
                            })
                            .when(!self.disabled, |this| {
                                this.on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &state_clone,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            state.is_dragging = true;
                                            state.update_from_position(e.position, cx);

                                            if let Some(ref handler) = on_change_thumb {
                                                handler(state.value, window, cx);
                                            }

                                            cx.stop_propagation();
                                        },
                                    ),
                                )
                            })
                    })
                    .when(!self.disabled, |this| {
                        let state_bar = self.state.clone();
                        let on_change_bar = self.on_change.clone();

                        this.on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(
                                &state_bar,
                                move |state, e: &MouseDownEvent, window, cx| {
                                    state.is_dragging = true;
                                    state.update_from_position(e.position, cx);

                                    if let Some(ref handler) = on_change_bar {
                                        handler(state.value, window, cx);
                                    }
                                },
                            ),
                        )
                        .on_mouse_move({
                            let state_move = self.state.clone();
                            let on_change_move = self.on_change.clone();

                            window.listener_for(
                                &state_move,
                                move |state, e: &MouseMoveEvent, window, cx| {
                                    if state.is_dragging {
                                        state.update_from_position(e.position, cx);

                                        if let Some(ref handler) = on_change_move {
                                            handler(state.value, window, cx);
                                        }
                                    }
                                },
                            )
                        })
                        .on_mouse_up(
                            MouseButton::Left,
                            window.listener_for(
                                &self.state,
                                move |state, _: &MouseUpEvent, _, _cx| {
                                    state.is_dragging = false;
                                },
                            ),
                        )
                    }),
            )
            .when(self.show_value, |this| {
                this.child(
                    div()
                        .min_w(px(40.0))
                        .text_center()
                        .text_sm()
                        .text_color(theme.tokens.foreground)
                        .child(format!("{:.0}", value)),
                )
            })
    }

    fn render_vertical(
        self,
        window: &mut Window,
        theme: crate::theme::Theme,
        focus_handle: FocusHandle,
        is_focused: bool,
        percentage: f32,
        value: f32,
        track_height: Pixels,
        thumb_width: Pixels,
        thumb_height: Pixels,
        track_bg: Hsla,
        active_bg: Hsla,
        thumb_bg: Hsla,
        focus_ring: BoxShadow,
        user_style: StyleRefinement,
    ) -> Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .h_full()
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(
                div()
                    .relative()
                    .flex_1()
                    .w(thumb_width)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(!self.disabled, |this| {
                        this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
                    })
                    .when(is_focused && !self.disabled, |this| {
                        this.shadow(vec![focus_ring])
                    })
                    .rounded(theme.tokens.radius_md)
                    .child(
                        canvas(
                            {
                                let state = self.state.clone();
                                move |bounds, _, cx| {
                                    state.update(cx, |state, _| {
                                        state.bounds = bounds;
                                    });
                                }
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full()
                    )
                    .child(
                        div()
                            .relative()
                            .h_full()
                            .w(track_height)
                            .rounded_full()
                            .bg(track_bg)
                            .overflow_hidden()
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .bottom_0()
                                    .w_full()
                                    .h(relative(percentage))
                                    .bg(active_bg),
                            ),
                    )
                    .child({
                        let state_clone = self.state.clone();
                        let on_change_thumb = self.on_change.clone();

                        div()
                            .absolute()
                            .left_0()
                            .bottom(relative(percentage))
                            .mb(-(thumb_height / 2.0))
                            .w(thumb_width)
                            .h(thumb_height)
                            .rounded(thumb_width / 2.0)
                            .bg(thumb_bg)
                            .border_2()
                            .border_color(theme.tokens.background)
                            .when(!self.disabled, |this| {
                                this.shadow(vec![theme.tokens.shadow_sm])
                                    .cursor(CursorStyle::PointingHand)
                            })
                            .when(!self.disabled, |this| {
                                this.on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &state_clone,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            state.is_dragging = true;
                                            state.update_from_position_vertical(e.position, cx);

                                            if let Some(ref handler) = on_change_thumb {
                                                handler(state.value, window, cx);
                                            }

                                            cx.stop_propagation();
                                        },
                                    ),
                                )
                            })
                    })
                    .when(!self.disabled, |this| {
                        let state_bar = self.state.clone();
                        let on_change_bar = self.on_change.clone();

                        this.on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(
                                &state_bar,
                                move |state, e: &MouseDownEvent, window, cx| {
                                    state.is_dragging = true;
                                    state.update_from_position_vertical(e.position, cx);

                                    if let Some(ref handler) = on_change_bar {
                                        handler(state.value, window, cx);
                                    }
                                },
                            ),
                        )
                        .on_mouse_move({
                            let state_move = self.state.clone();
                            let on_change_move = self.on_change.clone();

                            window.listener_for(
                                &state_move,
                                move |state, e: &MouseMoveEvent, window, cx| {
                                    if state.is_dragging {
                                        state.update_from_position_vertical(e.position, cx);

                                        if let Some(ref handler) = on_change_move {
                                            handler(state.value, window, cx);
                                        }
                                    }
                                },
                            )
                        })
                        .on_mouse_up(
                            MouseButton::Left,
                            window.listener_for(
                                &self.state,
                                move |state, _: &MouseUpEvent, _, _cx| {
                                    state.is_dragging = false;
                                },
                            ),
                        )
                    }),
            )
            .when(self.show_value, |this| {
                this.child(
                    div()
                        .min_h(px(24.0))
                        .text_center()
                        .text_sm()
                        .text_color(theme.tokens.foreground)
                        .child(format!("{:.0}", value)),
                )
            })
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let focus_handle = state.focus_handle(cx);
        let is_focused = focus_handle.is_focused(window);
        let percentage = state.percentage();
        let value = state.value;

        let track_height = self.size.track_height();
        let thumb_width = self.size.thumb_width();
        let thumb_height = self.size.thumb_height();

        let (track_bg, active_bg, thumb_bg) = if self.disabled {
            (
                theme.tokens.muted.opacity(0.3),
                theme.tokens.primary.opacity(0.3),
                theme.tokens.primary.opacity(0.3),
            )
        } else {
            (
                theme.tokens.muted,
                theme.tokens.primary,
                theme.tokens.primary,
            )
        };

        let focus_ring = theme.tokens.focus_ring_light();
        let user_style = self.style.clone();

        match self.axis {
            SliderAxis::Horizontal => self.render_horizontal(
                window,
                theme,
                focus_handle,
                is_focused,
                percentage,
                value,
                track_height,
                thumb_width,
                thumb_height,
                track_bg,
                active_bg,
                thumb_bg,
                focus_ring,
                user_style,
            ),
            SliderAxis::Vertical => self.render_vertical(
                window,
                theme,
                focus_handle,
                is_focused,
                percentage,
                value,
                track_height,
                thumb_width,
                thumb_height,
                track_bg,
                active_bg,
                thumb_bg,
                focus_ring,
                user_style,
            ),
        }
    }
}
