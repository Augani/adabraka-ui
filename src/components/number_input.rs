use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NumberInputSize {
    Sm,
    Md,
    Lg,
}

pub struct NumberInputState {
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    precision: usize,
    focus_handle: FocusHandle,
}

impl NumberInputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: 0.0,
            min: None,
            max: None,
            step: 1.0,
            precision: 0,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn with_value(cx: &mut Context<Self>, value: f64) -> Self {
        Self {
            value,
            min: None,
            max: None,
            step: 1.0,
            precision: 0,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64, cx: &mut Context<Self>) {
        self.value = self.clamp_value(value);
        cx.notify();
    }

    pub fn set_min(&mut self, min: Option<f64>, cx: &mut Context<Self>) {
        self.min = min;
        self.value = self.clamp_value(self.value);
        cx.notify();
    }

    pub fn set_max(&mut self, max: Option<f64>, cx: &mut Context<Self>) {
        self.max = max;
        self.value = self.clamp_value(self.value);
        cx.notify();
    }

    pub fn set_step(&mut self, step: f64) {
        self.step = step.max(0.001);
    }

    pub fn set_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    pub fn increment(&mut self, cx: &mut Context<Self>) {
        self.set_value(self.value + self.step, cx);
    }

    pub fn decrement(&mut self, cx: &mut Context<Self>) {
        self.set_value(self.value - self.step, cx);
    }

    pub fn can_increment(&self) -> bool {
        self.max.map_or(true, |max| self.value < max)
    }

    pub fn can_decrement(&self) -> bool {
        self.min.map_or(true, |min| self.value > min)
    }

    fn clamp_value(&self, value: f64) -> f64 {
        let mut v = value;
        if let Some(min) = self.min {
            v = v.max(min);
        }
        if let Some(max) = self.max {
            v = v.min(max);
        }
        v
    }

    fn format_value(&self) -> String {
        if self.precision == 0 {
            format!("{}", self.value as i64)
        } else {
            format!("{:.prec$}", self.value, prec = self.precision)
        }
    }
}

impl Focusable for NumberInputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for NumberInputState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct NumberInput {
    state: Entity<NumberInputState>,
    size: NumberInputSize,
    placeholder: Option<SharedString>,
    disabled: bool,
    show_buttons: bool,
    on_change: Option<Rc<dyn Fn(f64, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl NumberInput {
    pub fn new(state: Entity<NumberInputState>) -> Self {
        Self {
            state,
            size: NumberInputSize::Md,
            placeholder: None,
            disabled: false,
            show_buttons: true,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: NumberInputSize) -> Self {
        self.size = size;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for NumberInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NumberInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state_data = self.state.read(cx);
        let value_text = state_data.format_value();
        let can_increment = state_data.can_increment();
        let can_decrement = state_data.can_decrement();
        let focus_handle = state_data.focus_handle(cx);
        let is_focused = focus_handle.is_focused(window);
        let state = self.state.clone();

        let (height, padding_x, text_size, button_size) = match self.size {
            NumberInputSize::Sm => (px(32.0), px(8.0), px(13.0), px(24.0)),
            NumberInputSize::Md => (px(40.0), px(12.0), px(14.0), px(32.0)),
            NumberInputSize::Lg => (px(48.0), px(14.0), px(16.0), px(40.0)),
        };

        div()
            .flex()
            .items_center()
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(height)
                    .bg(theme.tokens.background)
                    .border_1()
                    .border_color(if is_focused {
                        theme.tokens.ring
                    } else {
                        theme.tokens.input
                    })
                    .rounded(theme.tokens.radius_md)
                    .when(self.disabled, |d| d.opacity(0.5))
                    .when(self.show_buttons, {
                        let state = state.clone();
                        let on_change = self.on_change.clone();
                        let disabled = self.disabled;
                        move |d| {
                            d.child(
                                div()
                                    .id("decrement")
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(button_size)
                                    .h_full()
                                    .border_r_1()
                                    .border_color(theme.tokens.border)
                                    .text_color(if can_decrement && !disabled {
                                        theme.tokens.foreground
                                    } else {
                                        theme.tokens.muted_foreground
                                    })
                                    .when(can_decrement && !disabled, |d| {
                                        d.cursor_pointer()
                                            .hover(|s| s.bg(theme.tokens.accent.opacity(0.5)))
                                    })
                                    .when(can_decrement && !disabled, {
                                        let state = state.clone();
                                        let on_change = on_change.clone();
                                        move |d| {
                                            d.on_click(move |_, window, cx| {
                                                state.update(cx, |s, cx| {
                                                    s.decrement(cx);
                                                    if let Some(ref handler) = on_change {
                                                        handler(s.value, window, cx);
                                                    }
                                                });
                                            })
                                        }
                                    })
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("−"),
                                    ),
                            )
                        }
                    })
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px(padding_x)
                            .h_full()
                            .min_w(px(60.0))
                            .text_size(text_size)
                            .text_color(theme.tokens.foreground)
                            .font_family(theme.tokens.font_family.clone())
                            .child(value_text),
                    )
                    .when(self.show_buttons, {
                        let state = state.clone();
                        let on_change = self.on_change.clone();
                        let disabled = self.disabled;
                        move |d| {
                            d.child(
                                div()
                                    .id("increment")
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(button_size)
                                    .h_full()
                                    .border_l_1()
                                    .border_color(theme.tokens.border)
                                    .text_color(if can_increment && !disabled {
                                        theme.tokens.foreground
                                    } else {
                                        theme.tokens.muted_foreground
                                    })
                                    .when(can_increment && !disabled, |d| {
                                        d.cursor_pointer()
                                            .hover(|s| s.bg(theme.tokens.accent.opacity(0.5)))
                                    })
                                    .when(can_increment && !disabled, {
                                        let state = state.clone();
                                        let on_change = on_change.clone();
                                        move |d| {
                                            d.on_click(move |_, window, cx| {
                                                state.update(cx, |s, cx| {
                                                    s.increment(cx);
                                                    if let Some(ref handler) = on_change {
                                                        handler(s.value, window, cx);
                                                    }
                                                });
                                            })
                                        }
                                    })
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("+"),
                                    ),
                            )
                        }
                    }),
            )
    }
}
