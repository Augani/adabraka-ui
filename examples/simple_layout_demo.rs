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
            |window, cx| cx.new(|cx| SimpleLayoutDemo::new(window, cx)),
        )
        .unwrap();
    });
}

struct SimpleLayoutDemo;

impl SimpleLayoutDemo {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());
        Self
    }
}

impl Render for SimpleLayoutDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .bg(theme.tokens.background)
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Header - Direct child of root flex container
                HStack::new()
                    .padding(24.0)
                    .align(Align::Center)
                    .justify(Justify::Between)
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("Layout System Demo"),
                    )
                    .child(
                        HStack::new()
                            .spacing(8.0)
                            .child(Button::new("docs-btn", "Docs").variant(ButtonVariant::Ghost))
                            .child(Button::new("settings-btn", "Settings").variant(ButtonVariant::Ghost)),
                    ),
            )
            .child(
                // Main content - Direct child of root flex container with .flex_1()
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                                scrollable_vertical(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .p(px(24.0))
                                        .gap(px(32.0))
                                        // VStack Demo
                                        .child(section_card(
                                            "VStack - Vertical Stacking",
                                            VStack::new()
                                                .spacing(12.0)
                                                .child(colored_box("Item 1", theme.tokens.primary))
                                                .child(colored_box("Item 2", theme.tokens.secondary))
                                                .child(colored_box("Item 3", theme.tokens.accent)),
                                        ))
                                        // HStack Demo
                                        .child(section_card(
                                            "HStack - Horizontal Stacking",
                                            VStack::new()
                                                .spacing(16.0)
                                                .child(
                                                    VStack::new()
                                                        .spacing(8.0)
                                                        .child(label("Justify: Between"))
                                                        .child(
                                                            HStack::new()
                                                                .justify(Justify::Between)
                                                                .child(colored_box("Left", theme.tokens.primary))
                                                                .child(colored_box("Right", theme.tokens.primary)),
                                                        ),
                                                )
                                                .child(
                                                    VStack::new()
                                                        .spacing(8.0)
                                                        .child(label("Justify: Center"))
                                                        .child(
                                                            HStack::new()
                                                                .justify(Justify::Center)
                                                                .spacing(12.0)
                                                                .child(colored_box("A", theme.tokens.secondary))
                                                                .child(colored_box("B", theme.tokens.secondary))
                                                                .child(colored_box("C", theme.tokens.secondary)),
                                                        ),
                                                ),
                                        ))
                                        // Grid Demo
                                        .child(section_card(
                                            "Grid - Grid Layout",
                                            Grid::new()
                                                .columns(3)
                                                .gap(16.0)
                                                .child(colored_box("Grid 1", theme.tokens.primary))
                                                .child(colored_box("Grid 2", theme.tokens.secondary))
                                                .child(colored_box("Grid 3", theme.tokens.accent))
                                                .child(colored_box("Grid 4", theme.tokens.destructive))
                                                .child(colored_box("Grid 5", theme.tokens.primary))
                                                .child(colored_box("Grid 6", theme.tokens.secondary)),
                                        ))
                                        // Flow Demo
                                        .child(section_card(
                                            "Flow - Wrapping Layout",
                                            Flow::new()
                                                .spacing(8.0)
                                                .child(tag("React"))
                                                .child(tag("Vue"))
                                                .child(tag("Angular"))
                                                .child(tag("Svelte"))
                                                .child(tag("Solid"))
                                                .child(tag("Qwik"))
                                                .child(tag("Preact"))
                                                .child(tag("Alpine")),
                                        ))
                                        // Cluster Demo
                                        .child(section_card(
                                            "Cluster - Inline Grouping",
                                            VStack::new()
                                                .spacing(16.0)
                                                .child(
                                                    Cluster::new()
                                                        .spacing(8.0)
                                                        .align(Align::Center)
                                                        .child(avatar("JD", theme.tokens.primary))
                                                        .child(label("John Doe"))
                                                        .child(tag("Admin")),
                                                )
                                                .child(
                                                    Cluster::new()
                                                        .spacing(8.0)
                                                        .align(Align::Center)
                                                        .child(avatar("JS", theme.tokens.secondary))
                                                        .child(label("Jane Smith"))
                                                        .child(tag("User")),
                                                ),
                                        ))
                                        // Spacer Demo
                                        .child(section_card(
                                            "Spacer - Flexible Spacing",
                                            VStack::new()
                                                .spacing(16.0)
                                                .child(
                                                    HStack::new()
                                                        .child(Button::new("left-btn", "Left"))
                                                        .child(Spacer::new())
                                                        .child(Button::new("right-btn", "Right")),
                                                )
                                                .child(
                                                    HStack::new()
                                                        .child(Button::new("first-btn", "First"))
                                                        .child(Spacer::new())
                                                        .child(Button::new("middle-btn", "Middle"))
                                                        .child(Spacer::new())
                                                        .child(Button::new("last-btn", "Last")),
                                                ),
                                        ))
                                        // Nested Layouts
                                        .child(section_card(
                                            "Nested Layouts - Complex Compositions",
                                            HStack::new()
                                                .spacing(16.0)
                                                .child(
                                                    VStack::new()
                                                        .spacing(12.0)
                                                        .child(colored_box("Header", theme.tokens.primary))
                                                        .child(colored_box("Content", theme.tokens.muted))
                                                        .child(colored_box("Footer", theme.tokens.accent)),
                                                )
                                                .child(
                                                    Grid::new()
                                                        .columns(2)
                                                        .gap(12.0)
                                                        .child(colored_box("1", theme.tokens.secondary))
                                                        .child(colored_box("2", theme.tokens.secondary))
                                                        .child(colored_box("3", theme.tokens.secondary))
                                                        .child(colored_box("4", theme.tokens.secondary)),
                                                ),
                                        ))
                                        // More examples to ensure scrolling
                                        .child(section_card(
                                            "Custom Scrollbar",
                                            VStack::new()
                                                .spacing(12.0)
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("This demo uses our custom animated scrollbar with:"),
                                                )
                                                .child(colored_box("✓ Hover & drag states", theme.tokens.primary))
                                                .child(colored_box("✓ Auto fade-in/fade-out", theme.tokens.secondary))
                                                .child(colored_box("✓ Click-to-jump support", theme.tokens.accent))
                                                .child(colored_box("✓ Smooth animations", theme.tokens.destructive)),
                                        ))
                                        // Additional Grid examples
                                        .child(section_card(
                                            "Grid - 4 Columns",
                                            Grid::new()
                                                .columns(4)
                                                .gap(12.0)
                                                .child(colored_box("A", theme.tokens.primary))
                                                .child(colored_box("B", theme.tokens.secondary))
                                                .child(colored_box("C", theme.tokens.accent))
                                                .child(colored_box("D", theme.tokens.destructive))
                                                .child(colored_box("E", theme.tokens.primary))
                                                .child(colored_box("F", theme.tokens.secondary))
                                                .child(colored_box("G", theme.tokens.accent))
                                                .child(colored_box("H", theme.tokens.destructive)),
                                        ))
                                        // More VStack examples
                                        .child(section_card(
                                            "VStack with Different Alignments",
                                            HStack::new()
                                                .spacing(16.0)
                                                .child(
                                                    VStack::new()
                                                        .spacing(8.0)
                                                        .align(Align::Start)
                                                        .child(label("Align: Start"))
                                                        .child(colored_box("Item 1", theme.tokens.primary))
                                                        .child(colored_box("Item 2", theme.tokens.secondary)),
                                                )
                                                .child(
                                                    VStack::new()
                                                        .spacing(8.0)
                                                        .align(Align::Center)
                                                        .child(label("Align: Center"))
                                                        .child(colored_box("Item 1", theme.tokens.accent))
                                                        .child(colored_box("Item 2", theme.tokens.destructive)),
                                                ),
                                        ))
                                        // Final section
                                        .child(section_card(
                                            "Scroll to See More!",
                                            VStack::new()
                                                .spacing(12.0)
                                                .child(colored_box("Try scrolling with mouse wheel", theme.tokens.primary))
                                                .child(colored_box("Hover over the scrollbar", theme.tokens.secondary))
                                                .child(colored_box("Click and drag the thumb", theme.tokens.accent))
                                                .child(colored_box("You've reached the end!", theme.tokens.destructive)),
                                        )),
                                ),
                            ),
            )
            .child(
                // Footer - Direct child of root flex container
                HStack::new()
                    .padding(16.0)
                    .justify(Justify::Center)
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(use_theme().tokens.muted_foreground)
                            .child("Layout System: Semantic • Composable • Type-Safe"),
                    ),
            )
    }
}

// Helper functions
fn section_card(title: impl Into<SharedString>, content: impl IntoElement) -> impl IntoElement {
    let theme = use_theme();
    let title: SharedString = title.into();

    VStack::new()
        .spacing(12.0)
        .child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.tokens.foreground)
                .child(title),
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

fn colored_box(text: impl Into<SharedString>, color: Hsla) -> impl IntoElement {
    let theme = use_theme();
    let text: SharedString = text.into();

    div()
        .bg(color)
        .rounded(theme.tokens.radius_md)
        .p(px(16.0))
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.tokens.primary_foreground)
        .font_weight(FontWeight::MEDIUM)
        .child(text)
}

fn tag(text: impl Into<SharedString>) -> impl IntoElement {
    let theme = use_theme();
    let text: SharedString = text.into();

    div()
        .bg(theme.tokens.secondary)
        .rounded(theme.tokens.radius_md)
        .px(px(12.0))
        .py(px(6.0))
        .text_size(px(12.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.tokens.secondary_foreground)
        .child(text)
}

fn avatar(initials: impl Into<SharedString>, color: Hsla) -> impl IntoElement {
    let theme = use_theme();
    let initials: SharedString = initials.into();

    div()
        .size(px(32.0))
        .rounded(px(16.0))
        .bg(color)
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.tokens.primary_foreground)
        .text_size(px(12.0))
        .font_weight(FontWeight::BOLD)
        .child(initials)
}

fn label(text: impl Into<SharedString>) -> impl IntoElement {
    let theme = use_theme();
    let text: SharedString = text.into();

    div()
        .text_size(px(14.0))
        .text_color(theme.tokens.foreground)
        .child(text)
}
