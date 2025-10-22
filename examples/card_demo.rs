use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Card Component Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(900.0), px(700.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| CardDemoApp::new(window, cx)),
        )
        .unwrap();
    });
}

struct CardDemoApp {
    notification_count: usize,
    is_editing: bool,
}

impl CardDemoApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());
        Self {
            notification_count: 3,
            is_editing: false,
        }
    }
}

impl Render for CardDemoApp {
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
                        div()
                            .text_size(px(28.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("Dashboard")
                    )
                    .child(
                        HStack::new()
                            .spacing(12.0)
                            .child(Button::new("settings-btn", "Settings").variant(ButtonVariant::Outline))
                            .child(Button::new("signout-btn", "Sign Out").variant(ButtonVariant::Ghost))
                    )
            )
            .child(
                // Main content with scrollable cards
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        scrollable_vertical(
                            div()
                                .p(px(32.0))
                                .child(
                                    Grid::new()
                                        .columns(2)
                                        .gap(24.0)
                                        // User Profile Card
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            div()
                                                                .text_size(px(18.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("User Profile")
                                                        )
                                                        .child(
                                                            Button::new("edit-btn", if self.is_editing { "Save" } else { "Edit" })
                                                                .size(ButtonSize::Sm)
                                                                .variant(ButtonVariant::Ghost)
                                                                .on_click(cx.listener(|view, _event, _window, cx| {
                                                                    view.is_editing = !view.is_editing;
                                                                    println!("[Card Demo] Edit button clicked! is_editing: {}", view.is_editing);
                                                                    cx.notify();
                                                                }))
                                                        )
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(16.0)
                                                        .child(
                                                            HStack::new()
                                                                .spacing(16.0)
                                                                .align(Align::Center)
                                                                .child(
                                                                    div()
                                                                        .size(px(64.0))
                                                                        .rounded(px(32.0))
                                                                        .bg(theme.tokens.primary)
                                                                        .flex()
                                                                        .items_center()
                                                                        .justify_center()
                                                                        .text_size(px(24.0))
                                                                        .font_weight(FontWeight::BOLD)
                                                                        .text_color(theme.tokens.primary_foreground)
                                                                        .child("JD")
                                                                )
                                                                .child(
                                                                    VStack::new()
                                                                        .spacing(4.0)
                                                                        .child(
                                                                            div()
                                                                                .text_size(px(16.0))
                                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                                .text_color(theme.tokens.foreground)
                                                                                .child("John Doe")
                                                                        )
                                                                        .child(
                                                                            div()
                                                                                .text_size(px(14.0))
                                                                                .text_color(theme.tokens.muted_foreground)
                                                                                .child("john.doe@example.com")
                                                                        )
                                                                )
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .child(if self.is_editing {
                                                                    "Editing mode active - make your changes here"
                                                                } else {
                                                                    "Senior Software Engineer with 10+ years of experience in building scalable applications."
                                                                })
                                                        )
                                                )
                                                .footer(
                                                    HStack::new()
                                                        .spacing(8.0)
                                                        .child(Badge::new("Premium").variant(BadgeVariant::Default))
                                                        .child(Badge::new("Verified").variant(BadgeVariant::Secondary))
                                                )
                                        )
                                        // Notifications Card
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            HStack::new()
                                                                .spacing(8.0)
                                                                .align(Align::Center)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(18.0))
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .text_color(theme.tokens.foreground)
                                                                        .child("Notifications")
                                                                )
                                                                .child(
                                                                    Badge::new(format!("{}", self.notification_count))
                                                                        .variant(BadgeVariant::Destructive)
                                                                )
                                                        )
                                                        .child(
                                                            Button::new("clear-all-btn", "Clear All")
                                                                .size(ButtonSize::Sm)
                                                                .variant(ButtonVariant::Ghost)
                                                                .on_click(cx.listener(|view, _event, _window, cx| {
                                                                    view.notification_count = 0;
                                                                    println!("[Card Demo] Clear All clicked! Notifications cleared");
                                                                    cx.notify();
                                                                }))
                                                        )
                                                )
                                                .content({
                                                    let mut vstack = VStack::new()
                                                        .spacing(12.0)
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .child(if self.notification_count > 0 {
                                                                    format!("You have {} unread notifications", self.notification_count)
                                                                } else {
                                                                    "No new notifications".to_string()
                                                                })
                                                        );

                                                    if self.notification_count > 0 {
                                                        vstack = vstack.child(
                                                            VStack::new()
                                                                .spacing(8.0)
                                                                .child(notification_item("New comment on your post", "2 minutes ago", theme.tokens.primary))
                                                                .child(notification_item("Someone liked your photo", "1 hour ago", theme.tokens.secondary))
                                                                .child(notification_item("You have a new follower", "3 hours ago", theme.tokens.accent))
                                                        );
                                                    }

                                                    vstack
                                                })
                                        )
                                        // Analytics Card
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            div()
                                                                .text_size(px(18.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Analytics")
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .spacing(8.0)
                                                                .child(
                                                                    Button::new("export-btn", "Export")
                                                                        .size(ButtonSize::Sm)
                                                                        .variant(ButtonVariant::Outline)
                                                                        .on_click(|_event, _window, _cx| {
                                                                            println!("[Card Demo] Export clicked!");
                                                                        })
                                                                )
                                                                .child(
                                                                    Button::new("refresh-btn", "Refresh")
                                                                        .size(ButtonSize::Sm)
                                                                        .on_click(|_event, _window, _cx| {
                                                                            println!("[Card Demo] Refresh clicked!");
                                                                        })
                                                                )
                                                        )
                                                )
                                                .content(
                                                    Grid::new()
                                                        .columns(2)
                                                        .gap(16.0)
                                                        .child(stat_box("Total Users", "12,543", "+12%", theme.tokens.primary))
                                                        .child(stat_box("Revenue", "$45,231", "+23%", theme.tokens.accent))
                                                        .child(stat_box("Active Sessions", "1,234", "+5%", theme.tokens.secondary))
                                                        .child(stat_box("Conversion Rate", "3.24%", "-2%", theme.tokens.destructive))
                                                )
                                        )
                                        // Recent Activity Card
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            div()
                                                                .text_size(px(18.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Recent Activity")
                                                        )
                                                        .child(
                                                            Button::new("view-all-btn", "View All")
                                                                .size(ButtonSize::Sm)
                                                                .variant(ButtonVariant::Link)
                                                                .on_click(|_event, _window, _cx| {
                                                                    println!("[Card Demo] View All clicked!");
                                                                })
                                                        )
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(12.0)
                                                        .child(activity_item("Deployed to production", "5 minutes ago"))
                                                        .child(activity_item("Merged PR #234", "1 hour ago"))
                                                        .child(activity_item("Created new branch", "3 hours ago"))
                                                        .child(activity_item("Updated dependencies", "5 hours ago"))
                                                )
                                        )
                                )
                        )
                    )
            )
    }
}

// Helper functions for rendering card content
fn notification_item(message: impl Into<SharedString>, time: impl Into<SharedString>, color: Hsla) -> impl IntoElement {
    let theme = use_theme();
    let message: SharedString = message.into();
    let time: SharedString = time.into();

    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .p(px(8.0))
        .rounded(theme.tokens.radius_md)
        .hover(|style| style.bg(theme.tokens.muted.opacity(0.5)))
        .child(
            div()
                .size(px(8.0))
                .rounded(px(4.0))
                .bg(color)
        )
        .child(
            VStack::new()
                .spacing(2.0)
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(theme.tokens.foreground)
                        .child(message)
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(time)
                )
        )
}

fn stat_box(label: impl Into<SharedString>, value: impl Into<SharedString>, change: impl Into<SharedString>, color: Hsla) -> impl IntoElement {
    let theme = use_theme();
    let label: SharedString = label.into();
    let value: SharedString = value.into();
    let change: SharedString = change.into();

    div()
        .p(px(16.0))
        .rounded(theme.tokens.radius_md)
        .border_1()
        .border_color(theme.tokens.border)
        .bg(theme.tokens.card)
        .child(
            VStack::new()
                .spacing(8.0)
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(label)
                )
                .child(
                    div()
                        .text_size(px(24.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.tokens.foreground)
                        .child(value)
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(color)
                        .child(change)
                )
        )
}

fn activity_item(message: impl Into<SharedString>, time: impl Into<SharedString>) -> impl IntoElement {
    let theme = use_theme();
    let message: SharedString = message.into();
    let time: SharedString = time.into();

    HStack::new()
        .spacing(12.0)
        .align(Align::Start)
        .child(
            div()
                .size(px(8.0))
                .rounded(px(4.0))
                .bg(theme.tokens.primary)
                .mt(px(6.0))
        )
        .child(
            VStack::new()
                .spacing(2.0)
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(theme.tokens.foreground)
                        .child(message)
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(time)
                )
        )
}
