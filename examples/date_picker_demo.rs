use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        date_picker::DateSelectionMode,
    },
};
use gpui::{prelude::FluentBuilder as _, *};
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

struct DatePickerDemoApp {
    basic_picker_state: Entity<DatePickerState>,
    min_max_picker_state: Entity<DatePickerState>,
    disabled_picker_state: Entity<DatePickerState>,
    custom_format_picker_state: Entity<DatePickerState>,
    locale_picker_state: Entity<DatePickerState>,
    styled_picker_state: Entity<DatePickerState>,
    range_picker_state: Entity<DatePickerState>,

    selected_basic: Option<String>,
    selected_min_max: Option<String>,
    selected_disabled: Option<String>,
    selected_custom: Option<String>,
    selected_locale: Option<String>,
    selected_styled: Option<String>,
    selected_range: Option<String>,
}

impl DatePickerDemoApp {
    fn new(cx: &mut App) -> Self {
        // Create initial states
        let basic_picker_state = cx.new(|cx| DatePickerState::new(cx));
        let min_max_picker_state = cx.new(|cx| DatePickerState::new(cx));
        let disabled_picker_state = cx.new(|cx| DatePickerState::new(cx));
        let custom_format_picker_state = cx.new(|cx| DatePickerState::new(cx));
        let locale_picker_state = cx.new(|cx| DatePickerState::new(cx));
        let styled_picker_state = cx.new(|cx| DatePickerState::new(cx));
        let range_picker_state = cx.new(|cx| DatePickerState::new_with_mode(DateSelectionMode::Range, cx));

        Self {
            basic_picker_state,
            min_max_picker_state,
            disabled_picker_state,
            custom_format_picker_state,
            locale_picker_state,
            styled_picker_state,
            range_picker_state,
            selected_basic: None,
            selected_min_max: None,
            selected_disabled: None,
            selected_custom: None,
            selected_locale: None,
            selected_styled: None,
            selected_range: None,
        }
    }

    fn render_section(&self, title: &str, description: &str, picker: impl IntoElement, selected: &Option<String>) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .p(px(16.0))
            .bg(rgb(0xf8f9fa))
            .rounded(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(title.to_string())
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgb(0x6c757d))
                            .child(description.to_string())
                    )
            )
            .child(picker)
            .map(|this| {
                if let Some(text) = selected.clone() {
                    this.child(
                        div()
                            .p(px(12.0))
                            .bg(rgb(0xe9ecef))
                            .rounded(px(6.0))
                            .text_size(px(14.0))
                            .child(format!("Selected: {}", text))
                    )
                } else {
                    this
                }
            })
    }
}

impl Render for DatePickerDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(64.0))
                    .px(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("DatePicker Component Demo")
                    )
            )
            .child(
                // Main content with scroll
                scrollable_vertical(
                        div()
                            .max_w(px(1200.0))
                            .mx_auto()
                            .p(px(24.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(24.0))
                                    // Basic DatePicker
                                    .child(
                                        self.render_section(
                                            "Basic DatePicker",
                                            "Simple date picker with default settings",
                                            DatePicker::new(self.basic_picker_state.clone())
                                                .placeholder("Select a date")
                                                .on_select(cx.listener(|this, date, _window, cx| {
                                                    let formatted = DateFormat::IsoDate.format(date, &CalendarLocale::english());
                                                    this.selected_basic = Some(formatted);
                                                    cx.notify();
                                                })),
                                            &self.selected_basic,
                                        )
                                    )
                                    // Min/Max DatePicker
                                    .child(
                                        self.render_section(
                                            "DatePicker with Min/Max Dates",
                                            "Date picker restricted to January 2025 - March 2025",
                                            DatePicker::new(self.min_max_picker_state.clone())
                                                .placeholder("Select date (Jan-Mar 2025)")
                                                .min_date(DateValue::new(2025, 1, 1))
                                                .max_date(DateValue::new(2025, 3, 31))
                                                .on_select(cx.listener(|this, date, _window, cx| {
                                                    let formatted = DateFormat::IsoDate.format(date, &CalendarLocale::english());
                                                    this.selected_min_max = Some(formatted);
                                                    cx.notify();
                                                })),
                                            &self.selected_min_max,
                                        )
                                    )
                                    // Disabled Dates DatePicker
                                    .child(
                                        self.render_section(
                                            "DatePicker with Disabled Dates",
                                            "Date picker with specific dates disabled (15th and 20th of January)",
                                            DatePicker::new(self.disabled_picker_state.clone())
                                                .placeholder("Select date (some disabled)")
                                                .disabled_dates(vec![
                                                    DateValue::new(2025, 1, 15),
                                                    DateValue::new(2025, 1, 20),
                                                    DateValue::new(2025, 1, 25),
                                                ])
                                                .on_select(cx.listener(|this, date, _window, cx| {
                                                    let formatted = DateFormat::IsoDate.format(date, &CalendarLocale::english());
                                                    this.selected_disabled = Some(formatted);
                                                    cx.notify();
                                                })),
                                            &self.selected_disabled,
                                        )
                                    )
                                    // Custom Format DatePicker
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(12.0))
                                            .p(px(16.0))
                                            .bg(rgb(0xf8f9fa))
                                            .rounded(px(8.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(16.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("DatePicker with Custom Formats")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .text_color(rgb(0x6c757d))
                                                            .child("Different date formats: US, EU, ISO, and Long format")
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(12.0))
                                                    .child(
                                                        DatePicker::new(self.custom_format_picker_state.clone())
                                                            .placeholder("US Format: MM/DD/YYYY")
                                                            .format(DateFormat::UsDate)
                                                            .on_select(cx.listener(|this, date, _window, cx| {
                                                                let formatted = DateFormat::UsDate.format(date, &CalendarLocale::english());
                                                                this.selected_custom = Some(format!("US: {}", formatted));
                                                                cx.notify();
                                                            }))
                                                    )
                                                    .child(
                                                        DatePicker::new(self.custom_format_picker_state.clone())
                                                            .placeholder("EU Format: DD/MM/YYYY")
                                                            .format(DateFormat::EuDate)
                                                    )
                                                    .child(
                                                        DatePicker::new(self.custom_format_picker_state.clone())
                                                            .placeholder("Long Format: Month DD, YYYY")
                                                            .format(DateFormat::LongDate)
                                                    )
                                            )
                                            .map(|this: Div| {
                                                if let Some(text) = self.selected_custom.clone() {
                                                    this.child(
                                                        div()
                                                            .p(px(12.0))
                                                            .bg(rgb(0xe9ecef))
                                                            .rounded(px(6.0))
                                                            .text_size(px(14.0))
                                                            .child(format!("Selected: {}", text))
                                                    )
                                                } else {
                                                    this
                                                }
                                            })
                                    )
                                    // Locale DatePicker
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(12.0))
                                            .p(px(16.0))
                                            .bg(rgb(0xf8f9fa))
                                            .rounded(px(8.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(16.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("DatePicker with Different Locales")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .text_color(rgb(0x6c757d))
                                                            .child("Calendars in different languages")
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(12.0))
                                                    .child(
                                                        DatePicker::new(self.locale_picker_state.clone())
                                                            .placeholder("French Locale")
                                                            .locale(CalendarLocale::french())
                                                            .format(DateFormat::LongDate)
                                                            .on_select(cx.listener(|this, date, _window, cx| {
                                                                let formatted = DateFormat::LongDate.format(date, &CalendarLocale::french());
                                                                this.selected_locale = Some(format!("French: {}", formatted));
                                                                cx.notify();
                                                            }))
                                                    )
                                                    .child(
                                                        DatePicker::new(self.locale_picker_state.clone())
                                                            .placeholder("Spanish Locale")
                                                            .locale(CalendarLocale::spanish())
                                                            .format(DateFormat::LongDate)
                                                    )
                                                    .child(
                                                        DatePicker::new(self.locale_picker_state.clone())
                                                            .placeholder("German Locale")
                                                            .locale(CalendarLocale::german())
                                                            .format(DateFormat::LongDate)
                                                    )
                                            )
                                            .map(|this: Div| {
                                                if let Some(text) = self.selected_locale.clone() {
                                                    this.child(
                                                        div()
                                                            .p(px(12.0))
                                                            .bg(rgb(0xe9ecef))
                                                            .rounded(px(6.0))
                                                            .text_size(px(14.0))
                                                            .child(format!("Selected: {}", text))
                                                    )
                                                } else {
                                                    this
                                                }
                                            })
                                    )
                                    // Range Selection DatePicker
                                    .child(
                                        self.render_section(
                                            "DatePicker with Range Selection",
                                            "Select a date range by clicking two dates - popover stays open until range is complete",
                                            DatePicker::new(self.range_picker_state.clone())
                                                .placeholder("Select date range")
                                                .on_select(cx.listener(|this, _date, _window, cx| {
                                                    // Read the range from state
                                                    let state = this.range_picker_state.read(cx);
                                                    if let Some(range) = state.selected_range {
                                                        let start_str = DateFormat::IsoDate.format(&range.start, &CalendarLocale::english());
                                                        let end_str = DateFormat::IsoDate.format(&range.end, &CalendarLocale::english());
                                                        this.selected_range = Some(format!("{} to {}", start_str, end_str));
                                                        cx.notify();
                                                    }
                                                })),
                                            &self.selected_range,
                                        )
                                    )
                                    // Styled DatePicker
                                    .child(
                                        self.render_section(
                                            "Styled DatePicker",
                                            "Custom styled date picker with custom background and border",
                                            DatePicker::new(self.styled_picker_state.clone())
                                                .placeholder("Custom styled picker")
                                                .on_select(cx.listener(|this, date, _window, cx| {
                                                    let formatted = DateFormat::LongDate.format(date, &CalendarLocale::english());
                                                    this.selected_styled = Some(formatted);
                                                    cx.notify();
                                                }))
                                                .w(px(400.0)),
                                            &self.selected_styled,
                                        )
                                    )
                                    // Feature Summary
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(12.0))
                                            .p(px(16.0))
                                            .bg(rgb(0xe7f5ff))
                                            .border_1()
                                            .border_color(rgb(0x74c0fc))
                                            .rounded(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(16.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x1971c2))
                                                    .child("DatePicker Features")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(8.0))
                                                    .text_size(px(14.0))
                                                    .text_color(rgb(0x1864ab))
                                                    .child("✓ Calendar popup with month/year navigation")
                                                    .child("✓ Single date and date range selection modes")
                                                    .child("✓ Date selection with immediate visual feedback")
                                                    .child("✓ Today button to jump to current date")
                                                    .child("✓ Min/max date restrictions")
                                                    .child("✓ Disabled dates support (visually greyed out)")
                                                    .child("✓ Multiple date formats (ISO, US, EU, Long)")
                                                    .child("✓ Multiple locales (English, French, Spanish, German, etc.)")
                                                    .child("✓ Clear button for easy date removal")
                                                    .child("✓ Auto-closes popover after selection")
                                                    .child("✓ Fully styled with Styled trait")
                                                    .child("✓ Custom callbacks for select and clear events")
                                                    .child("✓ Accessible with proper ARIA attributes")
                                            )
                                    )
                            )
                    )
            )
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
                        title: Some("DatePicker Demo - Adabraka UI".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1400.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| DatePickerDemoApp::new(cx)),
            )
            .unwrap();
        });
}
