// Direct test of ScrollContainer vs raw GPUI

use adabraka_ui::prelude::*;
use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size, FontWeight,
};

struct TestScrollContainer {}

impl Render for TestScrollContainer {
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
                    .font_weight(FontWeight::BOLD)
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
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xdbeafe))
                            .p_4()
                            .child("✓ Tall content (800px) - THIS SCROLLS")
                    )
            )
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("ScrollContainer Pattern:")
            )
            .child(
                // OUR ScrollContainer
                ScrollContainer::vertical()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(gpui::blue())
                    .bg(rgb(0xfafafa))
                    .p(px(12.0))
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xd1fae5))
                            .p_4()
                            .child("? Tall content (800px) - DOES THIS SCROLL?")
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TestScrollContainer {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

