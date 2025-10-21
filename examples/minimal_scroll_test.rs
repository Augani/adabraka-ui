// Absolute minimal test to debug ScrollContainer

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size, SharedString, ElementId,
};

struct MinimalTest {}

impl Render for MinimalTest {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Test 1: Create a Stateful<Div> with ID, then apply styles and overflow
        let test1 = div()
            .id("test-stateful-first")
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
                    .child("Test 1: ID -> styles -> overflow")
            );

        // Test 2: Create ID, apply styles, store in variable, then apply overflow
        let mut test2_base = div().id("test-stored-var");
        test2_base = test2_base.h(px(200.)).w_full();
        test2_base = test2_base.border_1().border_color(rgb(0x00ff00)).bg(rgb(0xfafafa)).p_4();
        test2_base = test2_base.child(
            div()
                .h(px(800.))
                .bg(rgb(0xd1fae5))
                .child("Test 2: Stored in var, styles, then overflow")
        );
        test2_base = test2_base.overflow_y_scroll();

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("Test 1: Direct chain"))
            .child(test1)
            .child(div().child("Test 2: Variable reassignment"))
            .child(test2_base)
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
            |_, cx| cx.new(|_| MinimalTest {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

