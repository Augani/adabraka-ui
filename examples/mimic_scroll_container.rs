// Exactly mimic what ScrollContainer does

use gpui::{
    div, prelude::*, px, rgb, AnyElement, App, Application, Bounds, Context, 
    Interactivity, ParentElement, Stateful, StatefulInteractiveElement, 
    Styled, StyleRefinement, Window, WindowBounds, WindowOptions, size, ElementId,
};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_test_scroll_id() -> ElementId {
    let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    ElementId::Name(format!("test-scroll-container-{}", id).into())
}

// Mimic ScrollContainer structure
struct MyScrollContainer {
    base: Stateful<gpui::Div>,
}

impl MyScrollContainer {
    fn new() -> Self {
        let base = div().id(next_test_scroll_id());  // Use dynamic ID like ScrollContainer
        Self { base }
    }
}

// Implement the same traits as ScrollContainer
impl Styled for MyScrollContainer {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for MyScrollContainer {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for MyScrollContainer {}

impl ParentElement for MyScrollContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl IntoElement for MyScrollContainer {
    type Element = Stateful<gpui::Div>;

    fn into_element(self) -> Self::Element {
        let MyScrollContainer { mut base } = self;
        base = base.overflow_y_scroll();
        base
    }
}

struct TestMimic {}

impl Render for TestMimic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("MyScrollContainer (WORKS):"))
            .child(
                MyScrollContainer::new()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0x0000ff))
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xdbeafe))
                            .p_4()
                            .child("Tall content - does this scroll?")
                    )
            )
            .child(div().child("Raw GPUI (for comparison):"))
            .child(
                div()
                    .h(px(200.))
                    .w_full()
                    .id("raw-gpui")
                    .overflow_y_scroll()
                    .border_1()
                    .border_color(rgb(0xff0000))
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xd1fae5))
                            .p_4()
                            .child("Tall content - this DOES scroll")
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
            |_, cx| cx.new(|_| TestMimic {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

