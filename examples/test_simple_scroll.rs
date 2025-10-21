// Testing to understand why raw GPUI works but our ScrollContainer doesn't

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};

struct TestScroll {}

impl Render for TestScroll {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(
                div()
                    .child("Raw GPUI Pattern (WORKS):")
            )
            .child(
                // RAW GPUI - This works
                div()
                    .h(px(200.))
                    .w_full()
                    .id("raw-scroll")
                    .overflow_y_scroll()
                    .border_1()
                    .border_color(gpui::red())
                    .bg(gpui::rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(gpui::rgb(0xdbeafe))
                            .child("Tall content (800px)")
                    )
            )
            .child(
                div()
                    .child("Test Pattern 1 - ID then overflow:")
            )
            .child(
                // Test pattern 1: ID first, then overflow
                {
                    let base = div().id("test-1");
                    base
                        .h(px(200.))
                        .w_full()
                        .overflow_y_scroll()
                        .border_1()
                        .border_color(gpui::blue())
                        .bg(gpui::rgb(0xfafafa))
                        .p_4()
                        .child(
                            div()
                                .h(px(800.))
                                .bg(gpui::rgb(0xd1fae5))
                                .child("Tall content (800px)")
                        )
                }
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TestScroll {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

