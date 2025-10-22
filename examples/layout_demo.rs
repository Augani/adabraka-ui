use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Layout System Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| LayoutDemo::new(window, cx)),
        )
        .unwrap();
    });
}

struct LayoutDemo {
    _theme: Theme,
    clicked_item: Option<String>,
}

impl LayoutDemo {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());
        Self {
            _theme: theme,
            clicked_item: None,
        }
    }
}

impl Render for LayoutDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let clicked_item = self.clicked_item.clone();

        div()
            .bg(theme.tokens.background)
            .size_full()
            .child(
                VStack::new()
                    .w_full()
                    .h_full()
                    .child(
                        // Header - showcasing styled HStack
                        HStack::new()
                            .w_full()
                            .p(px(24.0))
                            .bg(theme.tokens.card)
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .align(Align::Center)
                            .justify(Justify::Between)
                            .child(
                                HStack::new()
                                    .spacing(12.0)
                                    .align(Align::Center)
                                    .child(
                                        div()
                                            .size(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(theme.tokens.primary)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_color(theme.tokens.primary_foreground)
                                            .font_weight(FontWeight::BOLD)
                                            .child("L"),
                                    )
                                    .child(
                                        VStack::new()
                                            .spacing(2.0)
                                            .child(
                                                div()
                                                    .text_size(px(20.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.tokens.foreground)
                                                    .child("Advanced Layout System"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Fully styleable • Interactive • Type-safe"),
                                            ),
                                    ),
                            )
                            .child(
                                HStack::new()
                                    .spacing(8.0)
                                    .child(Button::new("docs-btn", "Docs").variant(ButtonVariant::Ghost))
                                    .child(Button::new("settings-btn", "Settings").variant(ButtonVariant::Ghost))
                                    .child(Button::new("profile-btn", "Profile").variant(ButtonVariant::Outline)),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .child(
                                scrollable_vertical(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .w_full()
                                        .p(px(24.0))
                                        .gap(px(32.0))
                                        // Section 1: Styled VStack with borders and backgrounds
                                        .child(demo_section(
                                            "🎨 Styled VStack",
                                            "VStack with borders, backgrounds, padding, and rounded corners",
                                            VStack::new()
                                                .w_full()
                                                .p(px(20.0))
                                                .bg(theme.tokens.card)
                                                .border_1()
                                                .border_color(theme.tokens.primary)
                                                .rounded(px(12.0))
                                                .spacing(12.0)
                                                .child(demo_card("Styled Item 1", theme.tokens.primary))
                                                .child(demo_card("Styled Item 2", theme.tokens.secondary))
                                                .child(demo_card("Styled Item 3", theme.tokens.accent)),
                                        ))
                                        // Section 2: Interactive layouts with hover effects
                                        .child(demo_section(
                                            "✨ Interactive Layouts",
                                            "Layouts with hover effects and click handlers",
                                            VStack::new()
                                                .w_full()
                                                .spacing(12.0)
                                                .child(
                                                    HStack::new()
                                                        .w_full()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.card)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(8.0))
                                                        .spacing(12.0)
                                                        .align(Align::Center)
                                                        .hover(|style| {
                                                            style
                                                                .bg(theme.tokens.accent.opacity(0.1))
                                                                .border_color(theme.tokens.accent)
                                                        })
                                                        .on_click(cx.listener(|_view, _event, _window, _cx| {
                                                            println!("[Layout Demo] HStack clicked!");
                                                        }))
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Hover over me! Click me!"),
                                                        )
                                                        .child(Spacer::new())
                                                        .child(Badge::new("Interactive").variant(BadgeVariant::Secondary)),
                                                )
                                                .child(
                                                    Grid::new()
                                                        .columns(3)
                                                        .gap(12.0)
                                                        .w_full()
                                                        .child(clickable_card("Card 1", theme.tokens.primary, "card-1", clicked_item.clone(), cx))
                                                        .child(clickable_card("Card 2", theme.tokens.secondary, "card-2", clicked_item.clone(), cx))
                                                        .child(clickable_card("Card 3", theme.tokens.accent, "card-3", clicked_item.clone(), cx)),
                                                )
                                                .children(
                                                    clicked_item.as_ref().map(|item| {
                                                        HStack::new()
                                                            .w_full()
                                                            .p(px(12.0))
                                                            .bg(theme.tokens.primary.opacity(0.1))
                                                            .border_1()
                                                            .border_color(theme.tokens.primary)
                                                            .rounded(px(6.0))
                                                            .justify(Justify::Center)
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(theme.tokens.primary)
                                                                    .child(format!("✓ You clicked: {}", item)),
                                                            )
                                                    })
                                                ),
                                        ))
                                        // Section 3: Database Record Pattern
                                        .child(demo_section(
                                            "📊 Database Record Pattern",
                                            "Real-world pattern for rendering database records with infinite scroll",
                                            VStack::new()
                                                .w_full()
                                                .spacing(8.0)
                                                .child(database_record_header())
                                                .child(database_record("1", "John Doe", "john@example.com", "Active", &theme))
                                                .child(database_record("2", "Jane Smith", "jane@example.com", "Active", &theme))
                                                .child(database_record("3", "Bob Johnson", "bob@example.com", "Offline", &theme))
                                                .child(database_record("4", "Alice Williams", "alice@example.com", "Active", &theme))
                                                .child(database_record("5", "Charlie Brown", "charlie@example.com", "Away", &theme))
                                                .child(
                                                    HStack::new()
                                                        .w_full()
                                                        .p(px(12.0))
                                                        .bg(theme.tokens.muted.opacity(0.3))
                                                        .rounded(px(6.0))
                                                        .justify(Justify::Center)
                                                        .child(
                                                            div()
                                                                .text_size(px(12.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .child("💡 These records use styled HStack layouts with hover effects"),
                                                        ),
                                                ),
                                        ))
                                        // Section 4: Complex nested layouts
                                        .child(demo_section(
                                            "🏗️ Complex Nested Layouts",
                                            "Combining multiple styled layouts for advanced UIs",
                                            HStack::new()
                                                .w_full()
                                                .spacing(16.0)
                                                .child(
                                                    VStack::new()
                                                        .flex_1()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.card)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(8.0))
                                                        .spacing(12.0)
                                                        .child(
                                                            div()
                                                                .text_size(px(16.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Sidebar Panel"),
                                                        )
                                                        .child(demo_card("Item 1", theme.tokens.primary))
                                                        .child(demo_card("Item 2", theme.tokens.primary))
                                                        .child(demo_card("Item 3", theme.tokens.primary)),
                                                )
                                                .child(
                                                    VStack::new()
                                                        .flex_grow()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.card)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(8.0))
                                                        .spacing(16.0)
                                                        .child(
                                                            HStack::new()
                                                                .w_full()
                                                                .justify(Justify::Between)
                                                                .align(Align::Center)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(16.0))
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .text_color(theme.tokens.foreground)
                                                                        .child("Main Content"),
                                                                )
                                                                .child(Badge::new("Live").variant(BadgeVariant::Destructive)),
                                                        )
                                                        .child(
                                                            Grid::new()
                                                                .columns(2)
                                                                .gap(12.0)
                                                                .w_full()
                                                                .child(demo_card("1", theme.tokens.secondary))
                                                                .child(demo_card("2", theme.tokens.secondary))
                                                                .child(demo_card("3", theme.tokens.secondary))
                                                                .child(demo_card("4", theme.tokens.secondary)),
                                                        ),
                                                ),
                                        ))
                                        // Section 5: Flow with styling
                                        .child(demo_section(
                                            "🌊 Styled Flow Layout",
                                            "Flow layout with background and borders",
                                            Flow::new()
                                                .w_full()
                                                .p(px(16.0))
                                                .bg(theme.tokens.card)
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .rounded(px(8.0))
                                                .spacing(8.0)
                                                .child(Badge::new("React"))
                                                .child(Badge::new("Vue").variant(BadgeVariant::Secondary))
                                                .child(Badge::new("Angular").variant(BadgeVariant::Outline))
                                                .child(Badge::new("Svelte"))
                                                .child(Badge::new("Solid").variant(BadgeVariant::Secondary))
                                                .child(Badge::new("Qwik").variant(BadgeVariant::Outline))
                                                .child(Badge::new("Preact"))
                                                .child(Badge::new("Alpine").variant(BadgeVariant::Secondary))
                                                .child(Badge::new("Lit"))
                                                .child(Badge::new("Ember").variant(BadgeVariant::Outline)),
                                        ))
                                        // Section 6: Spacer Demo
                                        .child(demo_section(
                                            "↔️ Spacer - Flexible Spacing",
                                            "Spacer expands to fill available space",
                                            VStack::new()
                                                .w_full()
                                                .spacing(16.0)
                                                .child(
                                                    HStack::new()
                                                        .w_full()
                                                        .p(px(12.0))
                                                        .bg(theme.tokens.card)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(6.0))
                                                        .child(Button::new("left-btn", "Left"))
                                                        .child(Spacer::new())
                                                        .child(Button::new("right-btn", "Right")),
                                                )
                                                .child(
                                                    HStack::new()
                                                        .w_full()
                                                        .p(px(12.0))
                                                        .bg(theme.tokens.card)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(6.0))
                                                        .spacing(8.0)
                                                        .child(Button::new("first-btn", "First"))
                                                        .child(Spacer::new())
                                                        .child(Button::new("middle-btn", "Middle"))
                                                        .child(Spacer::new())
                                                        .child(Button::new("last-btn", "Last")),
                                                ),
                                        )),
                                ),
                            ),
                    )
                    .child(
                        // Footer - styled HStack
                        HStack::new()
                            .w_full()
                            .p(px(16.0))
                            .bg(theme.tokens.card)
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .justify(Justify::Center)
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Layout System - Fully Styleable, Interactive, Type-Safe"),
                            ),
                    ),
            )
    }
}

fn demo_section(
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    content: impl IntoElement,
) -> impl IntoElement {
    let theme = use_theme();
    let title: SharedString = title.into();
    let description: SharedString = description.into();

    VStack::new()
        .spacing(16.0)
        .child(
            VStack::new()
                .spacing(4.0)
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.tokens.foreground)
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(description),
                ),
        )
        .child(
            div()
                .bg(theme.tokens.card)
                .border_1()
                .border_color(theme.tokens.border)
                .rounded(theme.tokens.radius_lg)
                .p(px(24.0))
                .child(content),
        )
}

fn demo_card(label: impl Into<SharedString>, bg: Hsla) -> impl IntoElement {
    let theme = use_theme();
    let label: SharedString = label.into();

    div()
        .bg(bg)
        .rounded(theme.tokens.radius_md)
        .p(px(16.0))
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.tokens.primary_foreground)
        .font_weight(FontWeight::MEDIUM)
        .child(label)
}

fn clickable_card(
    label: impl Into<SharedString>,
    bg: Hsla,
    id: &'static str,
    clicked_item: Option<String>,
    cx: &mut Context<LayoutDemo>,
) -> impl IntoElement {
    let theme = use_theme();
    let label: SharedString = label.into();
    let is_clicked = clicked_item.as_ref().map(|s| s.as_str()) == Some(id);
    let id_string = id.to_string();

    VStack::new()
        .flex_1()
        .p(px(16.0))
        .bg(if is_clicked { bg } else { theme.tokens.card })
        .border_2()
        .border_color(if is_clicked { bg } else { theme.tokens.border })
        .rounded(px(8.0))
        .spacing(8.0)
        .hover(|style| style.border_color(bg).bg(bg.opacity(0.1)))
        .on_click(cx.listener(move |view, _event, _window, cx| {
            view.clicked_item = Some(id_string.clone());
            println!("[Layout Demo] Clicked: {}", id_string);
            cx.notify();
        }))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(if is_clicked {
                    theme.tokens.primary_foreground
                } else {
                    theme.tokens.foreground
                })
                .child(label),
        )
        .child(
            div()
                .text_size(px(11.0))
                .text_color(if is_clicked {
                    theme.tokens.primary_foreground.opacity(0.8)
                } else {
                    theme.tokens.muted_foreground
                })
                .child("Click me!"),
        )
}

fn database_record_header() -> impl IntoElement {
    let theme = use_theme();

    HStack::new()
        .w_full()
        .p(px(12.0))
        .bg(theme.tokens.muted.opacity(0.5))
        .border_1()
        .border_color(theme.tokens.border)
        .rounded_t(px(8.0))
        .child(
            div()
                .w(px(60.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.tokens.foreground)
                .child("ID"),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.tokens.foreground)
                .child("Name"),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.tokens.foreground)
                .child("Email"),
        )
        .child(
            div()
                .w(px(80.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.tokens.foreground)
                .child("Status"),
        )
}

fn database_record(
    id: impl Into<SharedString>,
    name: impl Into<SharedString>,
    email: impl Into<SharedString>,
    status: impl Into<SharedString>,
    theme: &Theme,
) -> impl IntoElement {
    let id: SharedString = id.into();
    let name: SharedString = name.into();
    let email: SharedString = email.into();
    let status: SharedString = status.into();

    HStack::new()
        .w_full()
        .p(px(12.0))
        .bg(theme.tokens.card)
        .border_1()
        .border_color(theme.tokens.border)
        .border_t(px(0.0))
        .hover(|style| style.bg(theme.tokens.accent.opacity(0.1)))
        .child(
            div()
                .w(px(60.0))
                .text_size(px(13.0))
                .text_color(theme.tokens.muted_foreground)
                .child(id),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.tokens.foreground)
                .child(name),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(13.0))
                .text_color(theme.tokens.muted_foreground)
                .child(email),
        )
        .child(
            div()
                .w(px(80.0))
                .child(
                    Badge::new(status.clone()).variant(if status.as_ref() == "Active" {
                        BadgeVariant::Default
                    } else if status.as_ref() == "Away" {
                        BadgeVariant::Secondary
                    } else {
                        BadgeVariant::Outline
                    }),
                ),
        )
}
