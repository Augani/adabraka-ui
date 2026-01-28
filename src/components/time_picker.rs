use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFormat {
    Hour12,
    Hour24,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePeriod {
    AM,
    PM,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeValue {
    pub hour: u8,
    pub minute: u8,
    pub second: Option<u8>,
    pub period: Option<TimePeriod>,
}

impl TimeValue {
    pub fn new(hour: u8, minute: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
            second: None,
            period: None,
        }
    }

    pub fn with_seconds(mut self, second: u8) -> Self {
        self.second = Some(second.min(59));
        self
    }

    pub fn with_period(mut self, period: TimePeriod) -> Self {
        self.period = Some(period);
        self
    }

    pub fn to_24h(&self) -> (u8, u8, Option<u8>) {
        let hour = match self.period {
            Some(TimePeriod::AM) => {
                if self.hour == 12 {
                    0
                } else {
                    self.hour
                }
            }
            Some(TimePeriod::PM) => {
                if self.hour == 12 {
                    12
                } else {
                    self.hour + 12
                }
            }
            None => self.hour,
        };
        (hour, self.minute, self.second)
    }

    pub fn from_24h(hour: u8, minute: u8, second: Option<u8>, format: TimeFormat) -> Self {
        match format {
            TimeFormat::Hour24 => Self {
                hour: hour.min(23),
                minute: minute.min(59),
                second,
                period: None,
            },
            TimeFormat::Hour12 => {
                let (h12, period) = if hour == 0 {
                    (12, TimePeriod::AM)
                } else if hour < 12 {
                    (hour, TimePeriod::AM)
                } else if hour == 12 {
                    (12, TimePeriod::PM)
                } else {
                    (hour - 12, TimePeriod::PM)
                };
                Self {
                    hour: h12,
                    minute: minute.min(59),
                    second,
                    period: Some(period),
                }
            }
        }
    }

    pub fn format_string(&self, format: TimeFormat) -> String {
        let hour_str = format!("{:02}", self.hour);
        let minute_str = format!("{:02}", self.minute);

        let base = if let Some(sec) = self.second {
            format!("{}:{}:{:02}", hour_str, minute_str, sec)
        } else {
            format!("{}:{}", hour_str, minute_str)
        };

        match (format, self.period) {
            (TimeFormat::Hour12, Some(TimePeriod::AM)) => format!("{} AM", base),
            (TimeFormat::Hour12, Some(TimePeriod::PM)) => format!("{} PM", base),
            _ => base,
        }
    }
}

impl Default for TimeValue {
    fn default() -> Self {
        Self::new(12, 0)
    }
}

pub struct TimePickerState {
    value: TimeValue,
    format: TimeFormat,
    show_seconds: bool,
    open: bool,
    focus_handle: FocusHandle,
}

impl TimePickerState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: TimeValue::default(),
            format: TimeFormat::Hour24,
            show_seconds: false,
            open: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn value(&self) -> TimeValue {
        self.value
    }

    pub fn set_value(&mut self, value: TimeValue, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
    }

    pub fn set_hour(&mut self, hour: u8, cx: &mut Context<Self>) {
        let max_hour = match self.format {
            TimeFormat::Hour12 => 12,
            TimeFormat::Hour24 => 23,
        };
        self.value.hour = hour.min(max_hour);
        if self.format == TimeFormat::Hour12 && self.value.hour == 0 {
            self.value.hour = 12;
        }
        cx.notify();
    }

    pub fn set_minute(&mut self, minute: u8, cx: &mut Context<Self>) {
        self.value.minute = minute.min(59);
        cx.notify();
    }

    pub fn set_second(&mut self, second: u8, cx: &mut Context<Self>) {
        self.value.second = Some(second.min(59));
        cx.notify();
    }

    pub fn set_period(&mut self, period: TimePeriod, cx: &mut Context<Self>) {
        self.value.period = Some(period);
        cx.notify();
    }

    pub fn increment_hour(&mut self, cx: &mut Context<Self>) {
        let max = match self.format {
            TimeFormat::Hour12 => 12,
            TimeFormat::Hour24 => 23,
        };
        let min = match self.format {
            TimeFormat::Hour12 => 1,
            TimeFormat::Hour24 => 0,
        };
        self.value.hour = if self.value.hour >= max {
            min
        } else {
            self.value.hour + 1
        };
        cx.notify();
    }

    pub fn decrement_hour(&mut self, cx: &mut Context<Self>) {
        let max = match self.format {
            TimeFormat::Hour12 => 12,
            TimeFormat::Hour24 => 23,
        };
        let min = match self.format {
            TimeFormat::Hour12 => 1,
            TimeFormat::Hour24 => 0,
        };
        self.value.hour = if self.value.hour <= min {
            max
        } else {
            self.value.hour - 1
        };
        cx.notify();
    }

    pub fn increment_minute(&mut self, cx: &mut Context<Self>) {
        self.value.minute = if self.value.minute >= 59 {
            0
        } else {
            self.value.minute + 1
        };
        cx.notify();
    }

    pub fn decrement_minute(&mut self, cx: &mut Context<Self>) {
        self.value.minute = if self.value.minute == 0 {
            59
        } else {
            self.value.minute - 1
        };
        cx.notify();
    }

    pub fn increment_second(&mut self, cx: &mut Context<Self>) {
        if let Some(sec) = self.value.second {
            self.value.second = Some(if sec >= 59 { 0 } else { sec + 1 });
            cx.notify();
        }
    }

    pub fn decrement_second(&mut self, cx: &mut Context<Self>) {
        if let Some(sec) = self.value.second {
            self.value.second = Some(if sec == 0 { 59 } else { sec - 1 });
            cx.notify();
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.open = !self.open;
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }

    pub fn format(&self) -> TimeFormat {
        self.format
    }

    pub fn set_format(&mut self, format: TimeFormat, cx: &mut Context<Self>) {
        if self.format != format {
            let (h24, m, s) = self.value.to_24h();
            self.value = TimeValue::from_24h(h24, m, s, format);
            self.format = format;
            cx.notify();
        }
    }

    pub fn show_seconds(&self) -> bool {
        self.show_seconds
    }

    pub fn set_show_seconds(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_seconds = show;
        if show && self.value.second.is_none() {
            self.value.second = Some(0);
        } else if !show {
            self.value.second = None;
        }
        cx.notify();
    }
}

impl Focusable for TimePickerState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TimePickerState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct TimePicker {
    state: Entity<TimePickerState>,
    placeholder: SharedString,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&TimeValue, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl TimePicker {
    pub fn new(state: Entity<TimePickerState>) -> Self {
        Self {
            state,
            placeholder: "Select time".into(),
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&TimeValue, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for TimePicker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TimePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state_data = self.state.read(cx);
        let is_open = state_data.open;
        let value = state_data.value;
        let format = state_data.format;
        let show_seconds = state_data.show_seconds;
        let state = self.state.clone();

        let display_text = value.format_string(format);

        div()
            .relative()
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(
                div()
                    .id("time-picker-trigger")
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(12.0))
                    .py(px(8.0))
                    .min_w(px(140.0))
                    .bg(theme.tokens.background)
                    .border_1()
                    .border_color(if is_open {
                        theme.tokens.ring
                    } else {
                        theme.tokens.input
                    })
                    .rounded(theme.tokens.radius_md)
                    .text_size(px(14.0))
                    .text_color(theme.tokens.foreground)
                    .font_family(theme.tokens.font_family.clone())
                    .when(!self.disabled, |d| d.cursor_pointer())
                    .when(self.disabled, |d| d.opacity(0.5))
                    .when(!self.disabled, {
                        let state = state.clone();
                        move |d| {
                            d.on_click(move |_, _, cx| {
                                state.update(cx, |s, cx| s.toggle(cx));
                            })
                        }
                    })
                    .child(div().flex_1().child(display_text))
                    .child(div().text_color(theme.tokens.muted_foreground).child("🕐")),
            )
            .when(is_open && !self.disabled, |this| {
                this.child(
                    div()
                        .absolute()
                        .top_full()
                        .left_0()
                        .mt(px(4.0))
                        .bg(theme.tokens.popover)
                        .border_1()
                        .border_color(theme.tokens.border)
                        .rounded(theme.tokens.radius_md)
                        .shadow(vec![BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.15),
                            offset: point(px(0.0), px(4.0)),
                            blur_radius: px(12.0),
                            spread_radius: px(0.0),
                        }])
                        .p(px(16.0))
                        .child({
                            let on_change = self.on_change.clone();
                            let state_for_seconds = state.clone();
                            let state_for_period = state.clone();
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(Self::render_spinner(
                                    "hour",
                                    value.hour,
                                    state.clone(),
                                    "hour",
                                    &theme,
                                    on_change.clone(),
                                ))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child(":"),
                                )
                                .child(Self::render_spinner(
                                    "minute",
                                    value.minute,
                                    state.clone(),
                                    "minute",
                                    &theme,
                                    on_change.clone(),
                                ))
                                .when(show_seconds, {
                                    let on_change = on_change.clone();
                                    move |this| {
                                        this.child(
                                            div()
                                                .text_size(px(20.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.tokens.foreground)
                                                .child(":"),
                                        )
                                        .child(
                                            Self::render_spinner(
                                                "second",
                                                value.second.unwrap_or(0),
                                                state_for_seconds.clone(),
                                                "second",
                                                &theme,
                                                on_change,
                                            ),
                                        )
                                    }
                                })
                                .when(format == TimeFormat::Hour12, {
                                    let is_am = value.period == Some(TimePeriod::AM);
                                    let on_change = on_change.clone();
                                    move |this| {
                                        this.child(
                                            div()
                                                .ml(px(8.0))
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.0))
                                                .child(
                                                    Button::new("am", "AM")
                                                        .size(ButtonSize::Sm)
                                                        .variant(if is_am {
                                                            ButtonVariant::Default
                                                        } else {
                                                            ButtonVariant::Ghost
                                                        })
                                                        .on_click({
                                                            let state = state_for_period.clone();
                                                            let on_change = on_change.clone();
                                                            move |_, window, cx| {
                                                                state.update(cx, |s, cx| {
                                                                    s.set_period(
                                                                        TimePeriod::AM,
                                                                        cx,
                                                                    );
                                                                    if let Some(ref handler) =
                                                                        on_change
                                                                    {
                                                                        handler(
                                                                            &s.value, window, cx,
                                                                        );
                                                                    }
                                                                });
                                                            }
                                                        }),
                                                )
                                                .child(
                                                    Button::new("pm", "PM")
                                                        .size(ButtonSize::Sm)
                                                        .variant(if !is_am {
                                                            ButtonVariant::Default
                                                        } else {
                                                            ButtonVariant::Ghost
                                                        })
                                                        .on_click({
                                                            let state = state_for_period.clone();
                                                            let on_change = on_change.clone();
                                                            move |_, window, cx| {
                                                                state.update(cx, |s, cx| {
                                                                    s.set_period(
                                                                        TimePeriod::PM,
                                                                        cx,
                                                                    );
                                                                    if let Some(ref handler) =
                                                                        on_change
                                                                    {
                                                                        handler(
                                                                            &s.value, window, cx,
                                                                        );
                                                                    }
                                                                });
                                                            }
                                                        }),
                                                ),
                                        )
                                    }
                                })
                        }),
                )
            })
    }
}

impl TimePicker {
    fn render_spinner(
        id: &str,
        value: u8,
        state: Entity<TimePickerState>,
        field: &str,
        theme: &crate::theme::Theme,
        on_change: Option<Rc<dyn Fn(&TimeValue, &mut Window, &mut App)>>,
    ) -> impl IntoElement {
        let field_up = field.to_string();
        let field_down = field.to_string();
        let state_up = state.clone();
        let state_down = state.clone();
        let on_change_up = on_change.clone();
        let on_change_down = on_change;

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(4.0))
            .child(
                div()
                    .id(SharedString::from(format!("{}-up", id)))
                    .w(px(40.0))
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.tokens.accent))
                    .text_color(theme.tokens.muted_foreground)
                    .on_click(move |_, window, cx| {
                        state_up.update(cx, |s, cx| {
                            match field_up.as_str() {
                                "hour" => s.increment_hour(cx),
                                "minute" => s.increment_minute(cx),
                                "second" => s.increment_second(cx),
                                _ => {}
                            }
                            if let Some(ref handler) = on_change_up {
                                handler(&s.value, window, cx);
                            }
                        });
                    })
                    .child("▲"),
            )
            .child(
                div()
                    .w(px(48.0))
                    .h(px(40.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(theme.tokens.muted)
                    .rounded(px(6.0))
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.tokens.foreground)
                    .font_family(theme.tokens.font_family.clone())
                    .child(format!("{:02}", value)),
            )
            .child(
                div()
                    .id(SharedString::from(format!("{}-down", id)))
                    .w(px(40.0))
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.tokens.accent))
                    .text_color(theme.tokens.muted_foreground)
                    .on_click(move |_, window, cx| {
                        state_down.update(cx, |s, cx| {
                            match field_down.as_str() {
                                "hour" => s.decrement_hour(cx),
                                "minute" => s.decrement_minute(cx),
                                "second" => s.decrement_second(cx),
                                _ => {}
                            }
                            if let Some(ref handler) = on_change_down {
                                handler(&s.value, window, cx);
                            }
                        });
                    })
                    .child("▼"),
            )
    }
}
