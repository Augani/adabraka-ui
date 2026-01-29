use adabraka_ui::prelude::*;
use gpui::*;

struct AvatarGroupDemoApp;

impl Render for AvatarGroupDemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .p(px(40.0))
            .flex()
            .flex_col()
            .gap(px(32.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h2("Avatar Group Component"))
                    .child(muted("Display a group of avatars with overflow handling")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("Small Size"))
                            .child(
                                AvatarGroup::new(vec![
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=1"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=2"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=3"),
                                ])
                                .size(AvatarSize::Sm),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("Medium Size (Default)"))
                            .child(AvatarGroup::new(vec![
                                AvatarItem::new().src("https://i.pravatar.cc/150?u=4"),
                                AvatarItem::new().src("https://i.pravatar.cc/150?u=5"),
                                AvatarItem::new().src("https://i.pravatar.cc/150?u=6"),
                                AvatarItem::new().src("https://i.pravatar.cc/150?u=7"),
                            ])),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("Large Size with Max Display"))
                            .child(
                                AvatarGroup::new(vec![
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=8"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=9"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=10"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=11"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=12"),
                                    AvatarItem::new().src("https://i.pravatar.cc/150?u=13"),
                                ])
                                .size(AvatarSize::Lg)
                                .max_visible(4),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("With Fallback Initials"))
                            .child(
                                AvatarGroup::new(vec![
                                    AvatarItem::new().fallback_text("JD"),
                                    AvatarItem::new().fallback_text("AB"),
                                    AvatarItem::new().fallback_text("XY"),
                                ])
                                .size(AvatarSize::Md),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(500.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AvatarGroupDemoApp),
        )
        .unwrap();
    });
}
