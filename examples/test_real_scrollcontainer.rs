// Test using the ACTUAL ScrollContainer from our library

use adabraka_ui::layout::ScrollContainer;
use gpui::{
    div, prelude::*, px, rgb, App, Application, Bounds, Context,
    Window, WindowBounds, WindowOptions, size,
};

struct TestReal {}

impl Render for TestReal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("Raw GPUI (WORKS):"))
            .child(
                div()
                    .id("raw")
                    .h(px(200.))
                    .w_full()
                    .overflow_y_scroll()
                    .border_1()
                    .border_color(rgb(0xff0000))
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xdbeafe))
                            .child("Raw GPUI - scrolls")
                    )
            )
            .child(div().child("ACTUAL ScrollContainer from library:"))
            .child(
                ScrollContainer::vertical()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0x00ff00))
                    .bg(rgb(0xfafafa))
                    .p(px(12.0))
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xd1fae5))
                            .child("REAL ScrollContainer - does this scroll?")
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
            |_, cx| cx.new(|_| TestReal {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

