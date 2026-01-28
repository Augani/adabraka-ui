use adabraka_ui::{
    components::empty_state::{EmptyState, EmptyStateSize},
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;

struct EmptyStateDemo;

impl Render for EmptyStateDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
                    .p(px(24.0))
                    .gap(px(32.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Empty State Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Display placeholder content when no data is available"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Basic Empty State"),
                            )
                            .child(
                                div()
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .rounded(theme.tokens.radius_md)
                                    .p(px(24.0))
                                    .child(
                                        EmptyState::new("basic", "No results found")
                                            .icon("search")
                                            .description(
                                                "Try adjusting your search or filter to find what you're looking for.",
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("With Action Button"),
                            )
                            .child(
                                div()
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .rounded(theme.tokens.radius_md)
                                    .p(px(24.0))
                                    .child(
                                        EmptyState::new("with-action", "No projects yet")
                                            .icon("folder")
                                            .description("Get started by creating your first project.")
                                            .action("Create Project", |_, _| {}),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("With Primary and Secondary Actions"),
                            )
                            .child(
                                div()
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .rounded(theme.tokens.radius_md)
                                    .p(px(24.0))
                                    .child(
                                        EmptyState::new("with-actions", "Your inbox is empty")
                                            .icon("inbox")
                                            .description(
                                                "When you receive messages, they will appear here.",
                                            )
                                            .action("Compose Message", |_, _| {})
                                            .secondary_action("Learn More", |_, _| {}),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Size Variants"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(24.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .child(
                                                EmptyState::new("size-sm", "Small")
                                                    .icon("file")
                                                    .description("Compact empty state")
                                                    .size(EmptyStateSize::Sm),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .child(
                                                EmptyState::new("size-md", "Medium")
                                                    .icon("file")
                                                    .description("Default empty state")
                                                    .size(EmptyStateSize::Md),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .child(
                                                EmptyState::new("size-lg", "Large")
                                                    .icon("file")
                                                    .description("Large empty state")
                                                    .size(EmptyStateSize::Lg),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Different Icons"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(24.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .child(
                                                EmptyState::new("icon-users", "No team members")
                                                    .icon("users")
                                                    .description("Invite people to collaborate"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .child(
                                                EmptyState::new("icon-image", "No images")
                                                    .icon("image")
                                                    .description("Upload images to get started"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .child(
                                                EmptyState::new("icon-calendar", "No events")
                                                    .icon("calendar")
                                                    .description("Schedule your first event"),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Empty State Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| EmptyStateDemo),
        )
        .unwrap();

        cx.activate(true);
    });
}
