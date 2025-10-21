//! Test example for the scroll component

use adabraka_ui::components::scrollable::scrollable_vertical;
use gpui::*;

struct ScrollTestApp;

impl Render for ScrollTestApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(400.0))
                    .h(px(300.0))
                    .m(px(50.0))
                    .bg(rgb(0x313244))
                    .rounded_lg()
                    .p(px(20.0))
                    .child(
                        scrollable_vertical(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(10.0))
                                .children((0..50).map(|i| {
                                    div()
                                        .p(px(15.0))
                                        .bg(rgb(0x45475a))
                                        .rounded_md()
                                        .child(format!("Item {}", i + 1))
                                }))
                        )
                        .always_show_scrollbars()
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        // Initialize adabraka-ui
        adabraka_ui::init(cx);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Scroll Component Test".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| ScrollTestApp),
        )
        .unwrap();
    });
}