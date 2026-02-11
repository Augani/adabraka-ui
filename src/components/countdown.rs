use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::time::{Duration, SystemTime};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CountdownSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl CountdownSize {
    fn digit_size(&self) -> Pixels {
        match self {
            CountdownSize::Sm => px(20.0),
            CountdownSize::Md => px(32.0),
            CountdownSize::Lg => px(48.0),
        }
    }

    fn label_size(&self) -> Pixels {
        match self {
            CountdownSize::Sm => px(10.0),
            CountdownSize::Md => px(12.0),
            CountdownSize::Lg => px(14.0),
        }
    }

    fn separator_size(&self) -> Pixels {
        match self {
            CountdownSize::Sm => px(16.0),
            CountdownSize::Md => px(24.0),
            CountdownSize::Lg => px(36.0),
        }
    }

    fn unit_padding(&self) -> Pixels {
        match self {
            CountdownSize::Sm => px(8.0),
            CountdownSize::Md => px(12.0),
            CountdownSize::Lg => px(16.0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CountdownSeparator {
    #[default]
    Colon,
    Space,
    Dash,
    Dot,
    None,
}

impl CountdownSeparator {
    fn as_str(&self) -> &'static str {
        match self {
            CountdownSeparator::Colon => ":",
            CountdownSeparator::Space => " ",
            CountdownSeparator::Dash => "-",
            CountdownSeparator::Dot => ".",
            CountdownSeparator::None => "",
        }
    }
}

#[derive(Clone, Debug)]
pub struct CountdownFormat {
    pub show_days: bool,
    pub show_hours: bool,
    pub show_minutes: bool,
    pub show_seconds: bool,
    pub show_labels: bool,
    pub pad_zeros: bool,
}

impl Default for CountdownFormat {
    fn default() -> Self {
        Self {
            show_days: true,
            show_hours: true,
            show_minutes: true,
            show_seconds: true,
            show_labels: true,
            pad_zeros: true,
        }
    }
}

impl CountdownFormat {
    pub fn no_days() -> Self {
        Self {
            show_days: false,
            ..Default::default()
        }
    }

    pub fn time_only() -> Self {
        Self {
            show_days: false,
            show_hours: true,
            show_minutes: true,
            show_seconds: true,
            show_labels: false,
            pad_zeros: true,
        }
    }

    pub fn minimal() -> Self {
        Self {
            show_days: false,
            show_hours: false,
            show_minutes: true,
            show_seconds: true,
            show_labels: false,
            pad_zeros: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeUnits {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub total_seconds: i64,
}

impl TimeUnits {
    fn from_duration(duration: Duration) -> Self {
        let total_secs = duration.as_secs();
        let days = total_secs / 86400;
        let hours = (total_secs % 86400) / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        Self {
            days,
            hours,
            minutes,
            seconds,
            total_seconds: total_secs as i64,
        }
    }

    fn zero() -> Self {
        Self {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            total_seconds: 0,
        }
    }
}

pub struct CountdownState {
    target_time: Option<SystemTime>,
    start_time: Option<SystemTime>,
    count_up: bool,
    running: bool,
    completed: bool,
}

impl CountdownState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            target_time: None,
            start_time: None,
            count_up: false,
            running: false,
            completed: false,
        }
    }

    pub fn set_target(&mut self, target: SystemTime, cx: &mut Context<Self>) {
        self.target_time = Some(target);
        self.count_up = false;
        self.running = true;
        self.completed = false;
        self.schedule_tick(cx);
        cx.notify();
    }

    pub fn set_duration(&mut self, duration: Duration, cx: &mut Context<Self>) {
        let target = SystemTime::now() + duration;
        self.set_target(target, cx);
    }

    pub fn set_count_up(&mut self, start: SystemTime, cx: &mut Context<Self>) {
        self.start_time = Some(start);
        self.target_time = None;
        self.count_up = true;
        self.running = true;
        self.completed = false;
        self.schedule_tick(cx);
        cx.notify();
    }

    pub fn start_count_up(&mut self, cx: &mut Context<Self>) {
        self.set_count_up(SystemTime::now(), cx);
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.running = false;
        cx.notify();
    }

    pub fn resume(&mut self, cx: &mut Context<Self>) {
        if self.target_time.is_some() || self.start_time.is_some() {
            self.running = true;
            self.schedule_tick(cx);
            cx.notify();
        }
    }

    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.target_time = None;
        self.start_time = None;
        self.running = false;
        self.completed = false;
        cx.notify();
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn time_units(&self) -> TimeUnits {
        let now = SystemTime::now();

        if self.count_up {
            if let Some(start) = self.start_time {
                if let Ok(elapsed) = now.duration_since(start) {
                    return TimeUnits::from_duration(elapsed);
                }
            }
        } else if let Some(target) = self.target_time {
            if let Ok(remaining) = target.duration_since(now) {
                return TimeUnits::from_duration(remaining);
            }
        }

        TimeUnits::zero()
    }

    fn schedule_tick(&self, cx: &mut Context<Self>) {
        if !self.running {
            return;
        }

        cx.spawn(
            async | this,
            cx | {
                cx.background_executor().timer(Duration::from_secs(1)).await;

                _ = this.update(cx, |state, cx| {
                    if state.running {
                        if !state.count_up {
                            if let Some(target) = state.target_time {
                                if SystemTime::now() >= target {
                                    state.completed = true;
                                    state.running = false;
                                }
                            }
                        }

                        if state.running {
                            state.schedule_tick(cx);
                        }

                        cx.notify();
                    }
                });
            },
        )
        .detach();
    }
}

impl Render for CountdownState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct Countdown {
    id: ElementId,
    state: Entity<CountdownState>,
    size: CountdownSize,
    separator: CountdownSeparator,
    format: CountdownFormat,
    on_complete: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Countdown {
    pub fn new(id: impl Into<ElementId>, state: Entity<CountdownState>) -> Self {
        Self {
            id: id.into(),
            state,
            size: CountdownSize::Md,
            separator: CountdownSeparator::Colon,
            format: CountdownFormat::default(),
            on_complete: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: CountdownSize) -> Self {
        self.size = size;
        self
    }

    pub fn separator(mut self, separator: CountdownSeparator) -> Self {
        self.separator = separator;
        self
    }

    pub fn format(mut self, format: CountdownFormat) -> Self {
        self.format = format;
        self
    }

    pub fn show_days(mut self, show: bool) -> Self {
        self.format.show_days = show;
        self
    }

    pub fn show_hours(mut self, show: bool) -> Self {
        self.format.show_hours = show;
        self
    }

    pub fn show_minutes(mut self, show: bool) -> Self {
        self.format.show_minutes = show;
        self
    }

    pub fn show_seconds(mut self, show: bool) -> Self {
        self.format.show_seconds = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.format.show_labels = show;
        self
    }

    pub fn pad_zeros(mut self, pad: bool) -> Self {
        self.format.pad_zeros = pad;
        self
    }

    pub fn on_complete(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_complete = Some(Rc::new(handler));
        self
    }

    fn render_unit(
        &self,
        value: u64,
        label: &str,
        theme: &crate::theme::Theme,
    ) -> impl IntoElement {
        let digit_text = if self.format.pad_zeros {
            format!("{:02}", value)
        } else {
            format!("{}", value)
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(4.0))
            .px(self.size.unit_padding())
            .child(
                div()
                    .text_size(self.size.digit_size())
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.tokens.foreground)
                    .font_family(theme.tokens.font_mono.clone())
                    .child(digit_text),
            )
            .when(self.format.show_labels, |this| {
                this.child(
                    div()
                        .text_size(self.size.label_size())
                        .text_color(theme.tokens.muted_foreground)
                        .child(label.to_string()),
                )
            })
    }

    fn render_separator(&self, theme: &crate::theme::Theme) -> AnyElement {
        let sep = self.separator.as_str();
        if sep.is_empty() {
            return div().into_any_element();
        }

        div()
            .text_size(self.size.separator_size())
            .font_weight(FontWeight::BOLD)
            .text_color(theme.tokens.muted_foreground)
            .child(sep.to_string())
            .into_any_element()
    }
}

impl Styled for Countdown {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Countdown {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let units = state.time_units();
        let completed = state.is_completed();
        let user_style = self.style.clone();

        if completed {
            if let Some(ref handler) = self.on_complete {
                handler(window, cx);
            }
        }

        let mut elements: Vec<AnyElement> = Vec::new();

        if self.format.show_days {
            elements.push(
                self.render_unit(units.days, "days", &theme)
                    .into_any_element(),
            );
            if self.format.show_hours || self.format.show_minutes || self.format.show_seconds {
                elements.push(self.render_separator(&theme));
            }
        }

        if self.format.show_hours {
            let hours = if self.format.show_days {
                units.hours
            } else {
                units.days * 24 + units.hours
            };
            elements.push(self.render_unit(hours, "hours", &theme).into_any_element());
            if self.format.show_minutes || self.format.show_seconds {
                elements.push(self.render_separator(&theme));
            }
        }

        if self.format.show_minutes {
            let minutes = if self.format.show_hours {
                units.minutes
            } else {
                (units.days * 24 + units.hours) * 60 + units.minutes
            };
            elements.push(self.render_unit(minutes, "min", &theme).into_any_element());
            if self.format.show_seconds {
                elements.push(self.render_separator(&theme));
            }
        }

        if self.format.show_seconds {
            let seconds = if self.format.show_minutes {
                units.seconds
            } else {
                units.total_seconds as u64
            };
            elements.push(self.render_unit(seconds, "sec", &theme).into_any_element());
        }

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(4.0))
            .map(|this| {
                let mut d = this;
                d.style().refine(&user_style);
                d
            })
            .children(elements)
    }
}
