//! Calendar component - Date selection with month/year navigation.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;
use crate::components::button::{Button, ButtonVariant, ButtonSize};

/// Default English weekday abbreviations
pub const DEFAULT_WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Default English month names
pub const DEFAULT_MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

/// Localization configuration for the Calendar component
#[derive(Clone)]
pub struct CalendarLocale {
    /// Weekday abbreviations (Sunday to Saturday)
    pub weekdays: [SharedString; 7],
    /// Full month names (January to December)
    pub months: [SharedString; 12],
}

impl CalendarLocale {
    /// Create a new locale with custom weekdays and months
    pub fn new(weekdays: [SharedString; 7], months: [SharedString; 12]) -> Self {
        Self {
            weekdays,
            months,
        }
    }

    /// English locale (default)
    pub fn english() -> Self {
        Self {
            weekdays: DEFAULT_WEEKDAYS.map(|s| s.into()),
            months: DEFAULT_MONTHS.map(|s| s.into()),
        }
    }

    /// French locale
    pub fn french() -> Self {
        Self {
            weekdays: ["Di", "Lu", "Ma", "Me", "Je", "Ve", "Sa"].map(|s| s.into()),
            months: ["Janvier", "Février", "Mars", "Avril", "Mai", "Juin",
                     "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre"].map(|s| s.into()),
        }
    }

    /// Spanish locale
    pub fn spanish() -> Self {
        Self {
            weekdays: ["Do", "Lu", "Ma", "Mi", "Ju", "Vi", "Sá"].map(|s| s.into()),
            months: ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio",
                     "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"].map(|s| s.into()),
        }
    }

    /// German locale
    pub fn german() -> Self {
        Self {
            weekdays: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"].map(|s| s.into()),
            months: ["Januar", "Februar", "März", "April", "Mai", "Juni",
                     "Juli", "August", "September", "Oktober", "November", "Dezember"].map(|s| s.into()),
        }
    }

    /// Portuguese locale
    pub fn portuguese() -> Self {
        Self {
            weekdays: ["Do", "Se", "Te", "Qa", "Qi", "Sx", "Sá"].map(|s| s.into()),
            months: ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho",
                     "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"].map(|s| s.into()),
        }
    }

    /// Italian locale
    pub fn italian() -> Self {
        Self {
            weekdays: ["Do", "Lu", "Ma", "Me", "Gi", "Ve", "Sa"].map(|s| s.into()),
            months: ["Gennaio", "Febbraio", "Marzo", "Aprile", "Maggio", "Giugno",
                     "Luglio", "Agosto", "Settembre", "Ottobre", "Novembre", "Dicembre"].map(|s| s.into()),
        }
    }
}

impl Default for CalendarLocale {
    fn default() -> Self {
        Self::english()
    }
}

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
}

#[derive(IntoElement)]
pub struct Calendar {
    current_month: DateValue,
    selected_date: Option<DateValue>,
    on_date_select: Option<Rc<dyn Fn(&DateValue, &mut Window, &mut App)>>,
    on_month_change: Option<Rc<dyn Fn(&DateValue, &mut Window, &mut App)>>,
    locale: CalendarLocale,
    style: StyleRefinement,
}

impl Calendar {
    pub fn new() -> Self {
        let current_month = DateValue::new(2025, 1, 1);

        Self {
            current_month,
            selected_date: None,
            on_date_select: None,
            on_month_change: None,
            locale: CalendarLocale::default(),
            style: StyleRefinement::default(),
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

    pub fn locale(mut self, locale: CalendarLocale) -> Self {
        self.locale = locale;
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

impl Styled for Calendar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Calendar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let current_month = self.current_month;
        let selected_date = self.selected_date;
        let locale = self.locale.clone();

        let prev_month_date = self.prev_month();
        let next_month_date = self.next_month();

        let on_month_change_handler = self.on_month_change.clone();
        let on_date_select_handler = self.on_date_select;

        let days_in_month = current_month.days_in_month();
        let first_day_of_week = current_month.first_day_of_week();

        let month_name = if current_month.month >= 1 && current_month.month <= 12 {
            locale.months[(current_month.month - 1) as usize].clone()
        } else {
            "Unknown".into()
        };

        let user_style = self.style;

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
                            .child(format!("{} {}", month_name, current_month.year))
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
                        locale.weekdays.iter().map(|day| {
                            div()
                                .flex_1()
                                .text_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.muted_foreground)
                                .child(day.clone())
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
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
