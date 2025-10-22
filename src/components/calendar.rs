//! Calendar component - Date selection with month/year navigation.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;
use crate::components::button::{Button, ButtonVariant, ButtonSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateValue {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DateValue {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    fn days_in_month(&self) -> u32 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn first_day_of_week(&self) -> u32 {
        let q = 1i32;
        let m = if self.month < 3 {
            (self.month + 12) as i32
        } else {
            self.month as i32
        };
        let y = if self.month < 3 {
            self.year - 1
        } else {
            self.year
        };

        let h = (q + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
        ((h + 6) % 7) as u32
    }

    fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

#[derive(IntoElement)]
pub struct Calendar {
    current_month: DateValue,
    selected_date: Option<DateValue>,
    on_date_select: Option<Rc<dyn Fn(&DateValue, &mut Window, &mut App)>>,
    on_month_change: Option<Rc<dyn Fn(&DateValue, &mut Window, &mut App)>>,
}

impl Calendar {
    pub fn new() -> Self {
        let current_month = DateValue::new(2025, 1, 1);

        Self {
            current_month,
            selected_date: None,
            on_date_select: None,
            on_month_change: None,
        }
    }

    pub fn current_month(mut self, date: DateValue) -> Self {
        self.current_month = date;
        self
    }

    pub fn selected_date(mut self, date: DateValue) -> Self {
        self.selected_date = Some(date);
        self
    }

    pub fn on_date_select<F>(mut self, handler: F) -> Self
    where
        F: Fn(&DateValue, &mut Window, &mut App) + 'static,
    {
        self.on_date_select = Some(Rc::new(handler));
        self
    }

    pub fn on_month_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(&DateValue, &mut Window, &mut App) + 'static,
    {
        self.on_month_change = Some(Rc::new(handler));
        self
    }
    fn prev_month(&self) -> DateValue {
        if self.current_month.month == 1 {
            DateValue::new(self.current_month.year - 1, 12, 1)
        } else {
            DateValue::new(self.current_month.year, self.current_month.month - 1, 1)
        }
    }

    fn next_month(&self) -> DateValue {
        if self.current_month.month == 12 {
            DateValue::new(self.current_month.year + 1, 1, 1)
        } else {
            DateValue::new(self.current_month.year, self.current_month.month + 1, 1)
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

const DAYS_OF_WEEK: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

impl RenderOnce for Calendar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let current_month = self.current_month;
        let selected_date = self.selected_date;

        let prev_month_date = self.prev_month();
        let next_month_date = self.next_month();

        let on_month_change_handler = self.on_month_change.clone();
        let on_date_select_handler = self.on_date_select;

        let days_in_month = current_month.days_in_month();
        let first_day_of_week = current_month.first_day_of_week();

        div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .p(px(16.0))
            .bg(theme.tokens.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb(px(16.0))
                    .child({
                        let handler = on_month_change_handler.clone();
                        Button::new("prev-month-btn", "‹")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Sm)
                            .when(handler.is_some(), |btn| {
                                let handler = handler.unwrap();
                                btn.on_click(move |_, window, cx| {
                                    handler(&prev_month_date, window, cx);
                                })
                            })
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_center()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .child(format!("{} {}", current_month.month_name(), current_month.year))
                    )
                    .child({
                        let handler = on_month_change_handler;
                        Button::new("next-month-btn", "›")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Sm)
                            .when(handler.is_some(), |btn| {
                                let handler = handler.unwrap();
                                btn.on_click(move |_, window, cx| {
                                    handler(&next_month_date, window, cx);
                                })
                            })
                    })
            )
            .child(
                div()
                    .flex()
                    .mb(px(8.0))
                    .children(
                        DAYS_OF_WEEK.iter().map(|day| {
                            div()
                                .flex_1()
                                .text_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.muted_foreground)
                                .child(*day)
                        })
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children({
                        let mut weeks = Vec::new();
                        let mut current_day = 1;
                        let mut day_of_week = 0;

                        while current_day <= days_in_month {
                            let mut week_days = Vec::new();

                            for _ in 0..7 {
                                if (day_of_week < first_day_of_week && current_day == 1) || current_day > days_in_month {
                                    week_days.push(None);
                                } else {
                                    week_days.push(Some(current_day));
                                    current_day += 1;
                                }
                                day_of_week += 1;
                            }

                            day_of_week = 0;
                            weeks.push(week_days);
                        }

                        let on_date_select_for_weeks = on_date_select_handler.clone();
                        weeks.into_iter().map(move |week| {
                            let on_date_select_for_days = on_date_select_for_weeks.clone();
                            div()
                                .flex()
                                .gap(px(4.0))
                                .children(
                                    week.into_iter().map(move |day_option| {
                                        match day_option {
                                            Some(day) => {
                                                let date = DateValue::new(
                                                    current_month.year,
                                                    current_month.month,
                                                    day
                                                );
                                                let is_selected = selected_date.map_or(false, |sel| sel == date);
                                                let handler = on_date_select_for_days.clone();

                                                div()
                                                    .flex_1()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .h(px(36.0))
                                                    .text_size(px(14.0))
                                                    .rounded(theme.tokens.radius_sm)
                                                    .cursor(CursorStyle::PointingHand)
                                                    .when(is_selected, |this: Div| {
                                                        this.bg(theme.tokens.primary)
                                                            .text_color(theme.tokens.primary_foreground)
                                                            .font_weight(FontWeight::MEDIUM)
                                                    })
                                                    .when(!is_selected, |this: Div| {
                                                        this.text_color(theme.tokens.foreground)
                                                            .hover(|style| {
                                                                style.bg(theme.tokens.muted.opacity(0.5))
                                                            })
                                                    })
                                                    .when(handler.is_some(), |this: Div| {
                                                        let handler = handler.unwrap();
                                                        this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                            handler(&date, window, cx);
                                                        })
                                                    })
                                                    .child(day.to_string())
                                                    .into_any_element()
                                            }
                                            None => {
                                                div()
                                                    .flex_1()
                                                    .h(px(36.0))
                                                    .into_any_element()
                                            }
                                        }
                                    })
                                )
                        })
                    })
            )
    }
}
