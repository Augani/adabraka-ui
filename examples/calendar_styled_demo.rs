use adabraka_ui::{
    prelude::*,
    components::{
        calendar::{Calendar, DateValue, CalendarLocale},
        scrollable::scrollable_vertical,
    },
};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Calendar Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| CalendarStyledDemo::new()),
            )
            .unwrap();
        });
}

struct CalendarStyledDemo {
    selected_date: Option<DateValue>,
    current_month: DateValue,
}

impl CalendarStyledDemo {
    fn new() -> Self {
        Self {
            selected_date: Some(DateValue::new(2025, 1, 15)),
            current_month: DateValue::new(2025, 1, 1),
        }
    }
}

impl Render for CalendarStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Calendar Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent)
                            .child(
                                if let Some(date) = self.selected_date {
                                    format!("Selected: {}/{}/{}", date.month, date.day, date.year)
                                } else {
                                    "No date selected".to_string()
                                }
                            )
                    )
            )
            // 1. Custom Padding Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Padding (via Styled trait)")
                    )
                    .child(
                        HStack::new()
                            .gap(px(16.0))
                            .flex_wrap()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Default padding")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(self.current_month)
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Custom p_4()")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .p_4()  // <- Styled trait method
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Custom p_8()")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .p_8()  // <- Styled trait method
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // 2. Custom Background Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Background Colors")
                    )
                    .child(
                        HStack::new()
                            .gap(px(16.0))
                            .flex_wrap()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Blue gradient background")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 2, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .bg(rgb(0x1e3a8a))  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Purple background")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 2, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .bg(rgb(0x581c87))  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Green background")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 2, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .bg(rgb(0x14532d))  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // 3. Custom Borders
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Borders")
                    )
                    .child(
                        HStack::new()
                            .gap(px(16.0))
                            .flex_wrap()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Blue border 2px")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 3, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x3b82f6))
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Red border 2px")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 3, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0xef4444))
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Purple border 2px")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 3, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x8b5cf6))
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // 4. Custom Border Radius
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Border Radius")
                    )
                    .child(
                        HStack::new()
                            .gap(px(16.0))
                            .flex_wrap()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("No radius")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 4, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .rounded(px(0.0))  // <- Styled trait
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Large radius (16px)")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 4, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .rounded(px(16.0))  // <- Styled trait
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Extra large radius (24px)")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 4, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .rounded(px(24.0))  // <- Styled trait
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // 5. Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Width Control")
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Full width calendar")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 5, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .w_full()  // <- Styled trait
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Custom width (400px)")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 5, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .w(px(400.0))  // <- Styled trait
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // 6. Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Shadow Effects")
                    )
                    .child(
                        HStack::new()
                            .gap(px(16.0))
                            .flex_wrap()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Shadow small")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 6, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .shadow_sm()  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Shadow medium")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 6, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .shadow_md()  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Shadow large")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 6, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .shadow_lg()  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // 7. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Purple with rounded corners and shadow")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 7, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .p_8()  // <- Styled trait
                                            .rounded(px(20.0))  // <- Styled trait
                                            .bg(rgb(0x581c87))  // <- Styled trait
                                            .shadow_lg()  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Full width with blue background, border, and shadow")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 7, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .w_full()  // <- Styled trait
                                            .p(px(24.0))  // <- Styled trait
                                            .bg(rgb(0x1e3a8a))  // <- Styled trait
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x60a5fa))
                                            .rounded(px(12.0))  // <- Styled trait
                                            .shadow_md()  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Ultra custom: Green background, thick border, large radius, custom width")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 7, 1))
                                            .selected_date(self.selected_date.unwrap_or(DateValue::new(2025, 1, 15)))
                                            .p(px(28.0))  // <- Styled trait
                                            .bg(rgb(0x14532d))  // <- Styled trait
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x22c55e))
                                            .rounded(px(16.0))  // <- Styled trait
                                            .shadow_lg()  // <- Styled trait
                                            .w(px(450.0))  // <- Styled trait
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // Section: Internationalization (i18n)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.tokens.foreground)
                                    .child("Internationalization (i18n)")
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Customize weekday and month names for different languages")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            // French Locale
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("French locale with .locale(CalendarLocale::french())")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .locale(CalendarLocale::french())
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // Spanish Locale
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Spanish locale with .locale(CalendarLocale::spanish())")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .locale(CalendarLocale::spanish())
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // German Locale
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("German locale with .locale(CalendarLocale::german())")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .locale(CalendarLocale::german())
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // Portuguese Locale
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Portuguese locale with .locale(CalendarLocale::portuguese())")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .locale(CalendarLocale::portuguese())
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // Italian Locale
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Italian locale with .locale(CalendarLocale::italian())")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .locale(CalendarLocale::italian())
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // Custom Locale Example
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Custom locale: CalendarLocale::new([weekdays], [months])")
                                    )
                                    .child(
                                        Calendar::new()
                                            .current_month(DateValue::new(2025, 1, 1))
                                            .locale(CalendarLocale::new(
                                                ["S", "M", "T", "W", "T", "F", "S"].map(|s| s.into()),
                                                ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].map(|s| s.into())
                                            ))
                                            .on_date_select(cx.listener(|view, date, _, cx| {
                                                view.selected_date = Some(*date);
                                                cx.notify();
                                            }))
                                            .on_month_change(cx.listener(|view, date, _, cx| {
                                                view.current_month = *date;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // Info Box
            .child(
                div()
                    .mt(px(16.0))
                    .p(px(16.0))
                    .bg(theme.tokens.accent)
                    .rounded(px(8.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Calendar supports full i18n customization!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Built-in locales: English (default), French, Spanish, German, Portuguese, Italian")
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Or create custom with: CalendarLocale::new([weekdays], [months])")
                    )
            )
                )
            )
    }
}
