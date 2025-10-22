use adabraka_ui::{
    components::{
        select::{Select, SelectOption, SelectEvent},
        tooltip::{TooltipPlacement, tooltip},
        button::{Button, ButtonVariant, ButtonSize},
        scrollable::scrollable_vertical,
    },
    display::{
        badge::{Badge, BadgeVariant},
        card::Card,
    },
    layout::{VStack, HStack, Grid, Justify},
    theme::{install_theme, Theme, use_theme},
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

    fn list(&self, path: &str) -> Result<Vec<gpui::SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(gpui::SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Select & Tooltip Component Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| SelectTooltipDemoApp::new(cx)),
        )
        .unwrap();
    });
}

struct SelectTooltipDemoApp {
    country_select: Entity<Select<String>>,
    theme_select: Entity<Select<String>>,
    language_select: Entity<Select<String>>,
    disabled_select: Entity<Select<String>>,
    selected_country: Option<String>,
    selected_theme: Option<String>,
    selected_language: Option<String>,
}

impl SelectTooltipDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());

        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");

        let app = Self {
            country_select: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("us".to_string(), "United States").with_group("North America").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("ca".to_string(), "Canada").with_group("North America").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("uk".to_string(), "United Kingdom").with_group("Europe").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("de".to_string(), "Germany").with_group("Europe").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("fr".to_string(), "France").with_group("Europe").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("au".to_string(), "Australia").with_group("Oceania").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("jp".to_string(), "Japan").with_group("Asia").with_icon("src/icons/regular/globe.svg"),
                        SelectOption::new("cn".to_string(), "China").with_group("Asia").with_icon("src/icons/regular/globe.svg"),
                    ])
                    .placeholder("Select a country")
                    .selected_index(Some(0))
                    .leading_icon("src/icons/regular/globe.svg")
                    .clearable(true)
            }),
            theme_select: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("dark".to_string(), "Dark Theme").with_icon("src/icons/regular/palette.svg"),
                        SelectOption::new("light".to_string(), "Light Theme").with_icon("src/icons/regular/palette.svg"),
                        SelectOption::new("auto".to_string(), "Auto (System)").with_icon("src/icons/regular/palette.svg"),
                    ])
                    .placeholder("Select theme")
                    .selected_index(Some(0))
                    .leading_icon("src/icons/regular/palette.svg")
            }),
            language_select: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("en".to_string(), "English"),
                        SelectOption::new("es".to_string(), "Spanish"),
                        SelectOption::new("fr".to_string(), "French"),
                        SelectOption::new("de".to_string(), "German"),
                        SelectOption::new("it".to_string(), "Italian"),
                        SelectOption::new("pt".to_string(), "Portuguese"),
                        SelectOption::new("ru".to_string(), "Russian"),
                        SelectOption::new("ja".to_string(), "Japanese"),
                        SelectOption::new("zh".to_string(), "Chinese"),
                        SelectOption::new("ko".to_string(), "Korean"),
                    ])
                    .placeholder("Select language")
                    .searchable(true)
            }),
            disabled_select: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Option 1"),
                    ])
                    .placeholder("Disabled select")
                    .disabled(true)
            }),
            selected_country: Some("us".to_string()),
            selected_theme: Some("dark".to_string()),
            selected_language: None,
        };

        // Set up change handlers
        let country_entity = app.country_select.clone();
        cx.subscribe(&country_entity.clone(), move |this, _select, _event: &SelectEvent, cx| {
            let value = country_entity.read(cx).selected_value().cloned();
            this.selected_country = value.clone();
            if let Some(country) = value {
                println!("[Select Demo] Country changed to: {}", country);
            }
            cx.notify();
        }).detach();

        let theme_entity = app.theme_select.clone();
        cx.subscribe(&theme_entity.clone(), move |this, _select, _event: &SelectEvent, cx| {
            let value = theme_entity.read(cx).selected_value().cloned();
            this.selected_theme = value.clone();
            if let Some(theme) = value {
                println!("[Select Demo] Theme changed to: {}", theme);
            }
            cx.notify();
        }).detach();

        let language_entity = app.language_select.clone();
        cx.subscribe(&language_entity.clone(), move |this, _select, _event: &SelectEvent, cx| {
            let value = language_entity.read(cx).selected_value().cloned();
            this.selected_language = value.clone();
            if let Some(lang) = value {
                println!("[Select Demo] Language changed to: {}", lang);
            }
            cx.notify();
        }).detach();

        app
    }
}

impl Render for SelectTooltipDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .bg(theme.tokens.background)
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Page Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(32.0))
                    .py(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        VStack::new()
                            .spacing(4.0)
                            .child(
                                div()
                                    .text_size(px(28.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.tokens.foreground)
                                    .child("Select & Tooltip Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Industry-standard dropdown and tooltip components")
                            )
                    )
                    .child(
                        HStack::new()
                            .spacing(8.0)
                            .child(
                                tooltip(
                                    Button::new("refresh-btn", "Refresh")
                                        .variant(ButtonVariant::Outline)
                                        .size(ButtonSize::Sm)
                                        .on_click(|_event, _window, _cx| {
                                            println!("[Demo] Refresh clicked!");
                                        }),
                                    "Refresh the page"
                                ).placement(TooltipPlacement::Bottom)
                            )
                            .child(
                                tooltip(
                                    Button::new("settings-btn", "Settings")
                                        .variant(ButtonVariant::Ghost)
                                        .size(ButtonSize::Sm)
                                        .on_click(|_event, _window, _cx| {
                                            println!("[Demo] Settings clicked!");
                                        }),
                                    "Open settings"
                                ).placement(TooltipPlacement::Bottom)
                            )
                    )
            )
            .child(
                // Main content
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        scrollable_vertical(
                            div()
                                .p(px(32.0))
                                .child(
                                    VStack::new()
                                        .spacing(24.0)
                                        // Instructions Card
                                        .child(
                                            Card::new()
                                                .header(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("📋 Instructions")
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(8.0)
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .child("Select Component:")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .line_height(relative(1.6))
                                                                .child("• Click to open dropdown\n• Use ↑/↓ arrow keys to navigate\n• Press Enter or Space to select\n• Press Escape to close\n• Click outside to close\n• Type to search (when searchable)")
                                                        )
                                                        .child(
                                                            div()
                                                                .mt(px(8.0))
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .child("Tooltip Component:")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .line_height(relative(1.6))
                                                                .child("• Hover over elements with tooltips\n• Supports Top, Bottom, Left, Right placements\n• Configurable show/hide delays")
                                                        )
                                                )
                                        )
                                        // Select Components Grid
                                        .child(
                                            Grid::new()
                                                .columns(2)
                                                .gap(24.0)
                                                // Country Select Card
                                                .child(
                                                    Card::new()
                                                        .header(
                                                            div()
                                                                .text_size(px(16.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Country Selection")
                                                        )
                                                        .content(
                                                            VStack::new()
                                                                .spacing(12.0)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Select your country from the dropdown:")
                                                                )
                                                                .child(self.country_select.clone())
                                                                .child(
                                                                    div()
                                                                        .mt(px(8.0))
                                                                        .p(px(12.0))
                                                                        .rounded(theme.tokens.radius_md)
                                                                        .bg(theme.tokens.muted.opacity(0.3))
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.foreground)
                                                                        .child(format!(
                                                                            "Selected: {}",
                                                                            self.selected_country.clone().unwrap_or("None".to_string())
                                                                        ))
                                                                )
                                                        )
                                                )
                                                // Theme Select Card
                                                .child(
                                                    Card::new()
                                                        .header(
                                                            div()
                                                                .text_size(px(16.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Theme Preference")
                                                        )
                                                        .content(
                                                            VStack::new()
                                                                .spacing(12.0)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Choose your preferred theme:")
                                                                )
                                                                .child(self.theme_select.clone())
                                                                .child(
                                                                    div()
                                                                        .mt(px(8.0))
                                                                        .p(px(12.0))
                                                                        .rounded(theme.tokens.radius_md)
                                                                        .bg(theme.tokens.muted.opacity(0.3))
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.foreground)
                                                                        .child(format!(
                                                                            "Selected: {}",
                                                                            self.selected_theme.clone().unwrap_or("None".to_string())
                                                                        ))
                                                                )
                                                        )
                                                )
                                                // Language Select Card
                                                .child(
                                                    Card::new()
                                                        .header(
                                                            div()
                                                                .text_size(px(16.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Language (Searchable)")
                                                        )
                                                        .content(
                                                            VStack::new()
                                                                .spacing(12.0)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Searchable dropdown - type to filter options:")
                                                                )
                                                                .child(self.language_select.clone())
                                                                .child(
                                                                    div()
                                                                        .mt(px(8.0))
                                                                        .p(px(12.0))
                                                                        .rounded(theme.tokens.radius_md)
                                                                        .bg(theme.tokens.muted.opacity(0.3))
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.foreground)
                                                                        .child(format!(
                                                                            "Selected: {}",
                                                                            self.selected_language.clone().unwrap_or("None".to_string())
                                                                        ))
                                                                )
                                                        )
                                                )
                                                // Disabled Select Card
                                                .child(
                                                    Card::new()
                                                        .header(
                                                            div()
                                                                .text_size(px(16.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Disabled State")
                                                        )
                                                        .content(
                                                            VStack::new()
                                                                .spacing(12.0)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("This select is disabled and cannot be interacted with:")
                                                                )
                                                                .child(self.disabled_select.clone())
                                                        )
                                                )
                                        )
                                        // Tooltip Examples Card
                                        .child(
                                            Card::new()
                                                .header(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Tooltip Placement Examples")
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(24.0)
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .child("Hover over the buttons to see tooltips in different positions:")
                                                        )
                                                        // Top and Bottom tooltips
                                                        .child(
                                                            VStack::new()
                                                                .spacing(12.0)
                                                                .child(
                                                                    HStack::new()
                                                                        .spacing(12.0)
                                                                        .justify(Justify::Center)
                                                                        .child(
                                                                            tooltip(
                                                                                Button::new("tooltip-top-btn", "Tooltip Top")
                                                                                    .variant(ButtonVariant::Default),
                                                                                "This tooltip appears on top"
                                                                            ).placement(TooltipPlacement::Top)
                                                                        )
                                                                        .child(
                                                                            tooltip(
                                                                                Button::new("tooltip-bottom-btn", "Tooltip Bottom")
                                                                                    .variant(ButtonVariant::Default),
                                                                                "This tooltip appears on bottom"
                                                                            ).placement(TooltipPlacement::Bottom)
                                                                        )
                                                                )
                                                                // Left and Right tooltips
                                                                .child(
                                                                    HStack::new()
                                                                        .spacing(12.0)
                                                                        .justify(Justify::Center)
                                                                        .child(
                                                                            tooltip(
                                                                                Button::new("tooltip-left-btn", "Tooltip Left")
                                                                                    .variant(ButtonVariant::Outline),
                                                                                "This tooltip appears on the left"
                                                                            ).placement(TooltipPlacement::Left)
                                                                        )
                                                                        .child(
                                                                            tooltip(
                                                                                Button::new("tooltip-right-btn", "Tooltip Right")
                                                                                    .variant(ButtonVariant::Outline),
                                                                                "This tooltip appears on the right"
                                                                            ).placement(TooltipPlacement::Right)
                                                                        )
                                                                )
                                                        )
                                                        // Tooltips on other elements
                                                        .child(
                                                            div()
                                                                .mt(px(16.0))
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .child("Tooltips work on any element:")
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .spacing(12.0)
                                                                .justify(Justify::Center)
                                                                .child(
                                                                    tooltip(
                                                                        Badge::new("Primary").variant(BadgeVariant::Default),
                                                                        "Primary badge"
                                                                    )
                                                                )
                                                                .child(
                                                                    tooltip(
                                                                        Badge::new("Secondary").variant(BadgeVariant::Secondary),
                                                                        "Secondary badge"
                                                                    )
                                                                )
                                                                .child(
                                                                    tooltip(
                                                                        Badge::new("Outline").variant(BadgeVariant::Outline),
                                                                        "Outline badge"
                                                                    )
                                                                )
                                                                .child(
                                                                    tooltip(
                                                                        Badge::new("Destructive").variant(BadgeVariant::Destructive),
                                                                        "Destructive badge"
                                                                    )
                                                                )
                                                        )
                                                )
                                        )
                                )
                        )
                    )
            )
    }
}
