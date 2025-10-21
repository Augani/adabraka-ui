// Test different ElementId creation methods

use gpui::{
    div, prelude::*, px, rgb, App, Application, Bounds, Context, ElementId,
    Window, WindowBounds, WindowOptions, size,
};

struct TestElementId {}

impl Render for TestElementId {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Test 1: String literal (what works)
        let test1 = div()
            .id("literal-id")
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
                    .child("Test 1: String literal ID")
            );

        // Test 2: ElementId::Name with format! (what we use)
        let id2 = ElementId::Name(format!("formatted-id-{}", 123).into());
        let test2 = div()
            .id(id2)
            .h(px(200.))
            .w_full()
            .overflow_y_scroll()
            .border_1()
            .border_color(rgb(0x00ff00))
            .bg(rgb(0xfafafa))
            .p_4()
            .child(
                div()
                    .h(px(800.))
                    .bg(rgb(0xd1fae5))
                    .child("Test 2: ElementId::Name with format!")
            );

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("Test 1: String literal"))
            .child(test1)
            .child(div().child("Test 2: ElementId::Name(format!(...).into())"))
            .child(test2)
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
            |_, cx| cx.new(|_| TestElementId {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

