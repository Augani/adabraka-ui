use adabraka_ui::{
    components::{
        scrollable::scrollable_vertical,
        text::{caption, h1, h2, h3},
        timeline::{timeline, Timeline, TimelineItem, TimelineOrientation, TimelineSize},
    },
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;

struct TimelineDemo;

impl TimelineDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for TimelineDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("Timeline"))
                    .child(caption(
                        "Display events in chronological order with customizable indicators and layouts",
                    )),
            )
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(48.0))
                    .p(px(24.0))
                    .child(Self::basic_timeline_section(&theme))
                    .child(Self::layout_section(&theme))
                    .child(Self::indicator_styles_section(&theme))
                    .child(Self::connector_styles_section(&theme))
                    .child(Self::alternating_section(&theme))
                    .child(Self::variants_section(&theme))
                    .child(Self::sizes_section(&theme))
                    .child(Self::horizontal_section(&theme))
                    .child(Self::activity_feed_section(&theme)),
            ))
    }
}

impl TimelineDemo {
    fn basic_timeline_section(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Basic Timeline"))
            .child(caption("Simple vertical timeline with timestamps"))
            .child(
                div()
                    .p(px(24.0))
                    .rounded(theme.tokens.radius_lg)
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(timeline(vec![
                        TimelineItem::new("Project Created")
                            .description("Initial project setup and configuration")
                            .timestamp("Jan 15, 2024"),
                        TimelineItem::new("Development Started")
                            .description("Started implementing core features")
                            .timestamp("Jan 20, 2024"),
                        TimelineItem::new("Beta Release")
                            .description("Released first beta version to testers")
                            .timestamp("Feb 10, 2024"),
                        TimelineItem::new("Public Launch")
                            .description("Official public release")
                            .timestamp("Mar 1, 2024"),
                    ])),
            )
    }

    fn layout_section(theme: &Theme) -> impl IntoElement {
        let items = || {
            vec![
                TimelineItem::new("First Event")
                    .description("Event description here")
                    .timestamp("9:00 AM"),
                TimelineItem::new("Second Event")
                    .description("Another description")
                    .timestamp("10:00 AM"),
                TimelineItem::new("Third Event")
                    .description("Final description")
                    .timestamp("11:00 AM"),
            ]
        };

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Layout Options"))
            .child(caption("Left, right, and center layouts"))
            .child(
                div()
                    .flex()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Left (Default)"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).left_layout()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Right"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).right_layout()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Center"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).center_layout()),
                            ),
                    ),
            )
    }

    fn indicator_styles_section(theme: &Theme) -> impl IntoElement {
        let items = || {
            vec![
                TimelineItem::new("First Step")
                    .description("Getting started")
                    .success(),
                TimelineItem::new("Second Step")
                    .description("In progress")
                    .info(),
                TimelineItem::new("Third Step").description("Pending"),
            ]
        };

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Indicator Styles"))
            .child(caption("Dot, icon, and numbered indicators"))
            .child(
                div()
                    .flex()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Dots (Default)"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).dot_indicators()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Icons"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).icon_indicators()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Numbers"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).number_indicators()),
                            ),
                    ),
            )
    }

    fn connector_styles_section(theme: &Theme) -> impl IntoElement {
        let items = || {
            vec![
                TimelineItem::new("Start")
                    .description("Beginning")
                    .success(),
                TimelineItem::new("Middle").description("Processing").info(),
                TimelineItem::new("End").description("Complete"),
            ]
        };

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Connector Styles"))
            .child(caption("Solid, dashed, or no connector lines"))
            .child(
                div()
                    .flex()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Solid (Default)"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).solid_connector()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Dashed"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).dashed_connector()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("No Connector"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).no_connector()),
                            ),
                    ),
            )
    }

    fn alternating_section(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Alternating Layout"))
            .child(caption(
                "Items alternate between left and right sides of a center line",
            ))
            .child(
                div()
                    .p(px(24.0))
                    .rounded(theme.tokens.radius_lg)
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Timeline::vertical(vec![
                            TimelineItem::new("Q1 2024")
                                .description("Research and planning phase completed")
                                .icon("search")
                                .info(),
                            TimelineItem::new("Q2 2024")
                                .description("Development and prototyping started")
                                .icon("code")
                                .success(),
                            TimelineItem::new("Q3 2024")
                                .description("Testing and refinement in progress")
                                .icon("check-square")
                                .warning(),
                            TimelineItem::new("Q4 2024")
                                .description("Launch and marketing campaign")
                                .icon("rocket")
                                .success(),
                        ])
                        .alternating(true)
                        .icon_indicators(),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h3("With Custom Positions"))
                    .child(caption(
                        "Override per-item position with .left() or .right()",
                    ))
                    .child(
                        div()
                            .p(px(24.0))
                            .rounded(theme.tokens.radius_lg)
                            .bg(theme.tokens.card)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .child(
                                Timeline::vertical(vec![
                                    TimelineItem::new("Always Left")
                                        .description("This item stays on the left")
                                        .left(),
                                    TimelineItem::new("Always Right")
                                        .description("This item stays on the right")
                                        .right(),
                                    TimelineItem::new("Also Left")
                                        .description("Forced to left side")
                                        .left(),
                                    TimelineItem::new("Also Right")
                                        .description("Forced to right side")
                                        .right(),
                                ])
                                .center_layout(),
                            ),
                    ),
            )
    }

    fn variants_section(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Item Variants"))
            .child(caption("Different visual states for timeline items"))
            .child(
                div()
                    .p(px(24.0))
                    .rounded(theme.tokens.radius_lg)
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Timeline::vertical(vec![
                            TimelineItem::new("Build Started")
                                .description("CI/CD pipeline triggered")
                                .timestamp("10:00 AM")
                                .info(),
                            TimelineItem::new("Tests Passed")
                                .description("All 156 tests passed successfully")
                                .timestamp("10:05 AM")
                                .success(),
                            TimelineItem::new("Warning: Deprecated API")
                                .description("Using deprecated method in auth module")
                                .timestamp("10:06 AM")
                                .warning(),
                            TimelineItem::new("Build Failed")
                                .description("Compilation error in parser module")
                                .timestamp("10:08 AM")
                                .error(),
                            TimelineItem::new("Retrying Build")
                                .description("Attempting rebuild with fixes")
                                .timestamp("10:15 AM"),
                        ])
                        .icon_indicators(),
                    ),
            )
    }

    fn sizes_section(theme: &Theme) -> impl IntoElement {
        let items = || {
            vec![
                TimelineItem::new("First Event")
                    .description("Event description")
                    .timestamp("9:00 AM"),
                TimelineItem::new("Second Event")
                    .description("Another description")
                    .timestamp("10:00 AM"),
                TimelineItem::new("Third Event")
                    .description("Final description")
                    .timestamp("11:00 AM"),
            ]
        };

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Size Variants"))
            .child(caption("Small, medium, and large timeline sizes"))
            .child(
                div()
                    .flex()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Small"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).sm()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Medium (Default)"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).md()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .flex_1()
                            .child(h3("Large"))
                            .child(
                                div()
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(Timeline::vertical(items()).lg()),
                            ),
                    ),
            )
    }

    fn horizontal_section(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Horizontal Timeline"))
            .child(caption("Timeline displayed in horizontal orientation"))
            .child(
                div()
                    .p(px(24.0))
                    .rounded(theme.tokens.radius_lg)
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Timeline::horizontal(vec![
                            TimelineItem::new("Step 1")
                                .description("Planning")
                                .icon("clipboard"),
                            TimelineItem::new("Step 2")
                                .description("Design")
                                .icon("pen-tool")
                                .success(),
                            TimelineItem::new("Step 3")
                                .description("Development")
                                .icon("code")
                                .success(),
                            TimelineItem::new("Step 4")
                                .description("Testing")
                                .icon("check-circle")
                                .info(),
                            TimelineItem::new("Step 5")
                                .description("Deploy")
                                .icon("upload"),
                        ])
                        .icon_indicators()
                        .size(TimelineSize::Sm),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h3("With Numbers"))
                    .child(
                        div()
                            .p(px(24.0))
                            .rounded(theme.tokens.radius_lg)
                            .bg(theme.tokens.card)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .child(
                                Timeline::new(vec![
                                    TimelineItem::new("2020").description("Founded"),
                                    TimelineItem::new("2021").description("Series A").success(),
                                    TimelineItem::new("2022").description("Expansion").success(),
                                    TimelineItem::new("2023").description("IPO").success(),
                                    TimelineItem::new("2024").description("Global").info(),
                                ])
                                .orientation(TimelineOrientation::Horizontal)
                                .number_indicators(),
                            ),
                    ),
            )
    }

    fn activity_feed_section(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(h2("Activity Feed Example"))
            .child(caption("Real-world use case: user activity feed"))
            .child(
                div()
                    .max_w(px(500.0))
                    .p(px(24.0))
                    .rounded(theme.tokens.radius_lg)
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Timeline::vertical(vec![
                            TimelineItem::new("John commented on your post")
                                .description(
                                    "\"Great article! Really helped me understand the concept.\"",
                                )
                                .timestamp("2 minutes ago")
                                .icon("message-circle"),
                            TimelineItem::new("New follower")
                                .description("Sarah Chen started following you")
                                .timestamp("1 hour ago")
                                .icon("user-plus")
                                .info(),
                            TimelineItem::new("Post published")
                                .description(
                                    "Your article \"Getting Started with Rust\" is now live",
                                )
                                .timestamp("3 hours ago")
                                .icon("check-circle")
                                .success(),
                            TimelineItem::new("Draft saved")
                                .description("Auto-saved draft of \"Advanced Patterns\"")
                                .timestamp("5 hours ago")
                                .icon("save"),
                            TimelineItem::new("Account verified")
                                .description("Your email has been verified successfully")
                                .timestamp("Yesterday")
                                .icon("shield-check")
                                .success(),
                            TimelineItem::new("Password changed")
                                .description("Security update applied to your account")
                                .timestamp("2 days ago")
                                .icon("lock")
                                .warning(),
                        ])
                        .sm()
                        .icon_indicators(),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(1100.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Timeline Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TimelineDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
