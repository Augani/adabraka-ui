// Debug version to understand what's happening with ScrollContainer

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct DebugScroll {}

impl Render for DebugScroll {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Let's manually inline what ScrollContainer::vertical() does
        let scroll_id = "debug-scroll-1";

        let mut container = div().id(scroll_id);
        
        // User applies styling
        container = container.h(px(200.)).w_full().border_1().border_color(rgb(0x3b82f6)).bg(rgb(0xfafafa)).p_4();
        
        // Then into_element applies overflow
        container = container.overflow_y_scroll();
        
        // Add child
        container = container.child(
            div()
                .h(px(800.))
                .bg(rgb(0xdbeafe))
                .p_4()
                .child("Manually inlined ScrollContainer logic")
        );
        
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(
                div()
                    .child("Manually Inlined ScrollContainer:")
            )
            .child(container)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(400.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| DebugScroll {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

