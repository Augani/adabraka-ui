// Test if creating div inside into_element works

use gpui::{
    div, prelude::*, px, rgb, App, Application, Bounds, Context,
    Window, WindowBounds, WindowOptions, size, AnyElement, Stateful,
    ParentElement, ElementId,
};

struct InlineDiv {
    id_name: String,
    children: Vec<AnyElement>,
}

impl InlineDiv {
    fn new() -> Self {
        Self {
            id_name: "inline-test".to_string(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for InlineDiv {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl IntoElement for InlineDiv {
    type Element = Stateful<gpui::Div>;

    fn into_element(self) -> Self::Element {
        let mut base = div()
            .id(ElementId::Name(self.id_name.into()))
            .overflow_y_scroll();
        
        base.extend(self.children);
        base
    }
}

struct TestInline {}

impl Render for TestInline {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("InlineDiv (creates div in into_element):"))
            .child(
                InlineDiv::new()
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
                            .child("Inline content - does this scroll?")
                    )
            )
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
            |_, cx| cx.new(|_| TestInline {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

