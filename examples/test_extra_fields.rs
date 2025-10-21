// Test if extra fields break scrolling

use gpui::{
    div, prelude::*, px, rgb, AnyElement, App, Application, Bounds, Context, 
    Interactivity, ParentElement, Stateful, StatefulInteractiveElement, 
    Styled, StyleRefinement, Window, WindowBounds, WindowOptions, size, ElementId, ScrollHandle,
};

// Test 1: Single field (SHOULD WORK)
struct Test1 {
    base: Stateful<gpui::Div>,
}

impl Test1 {
    fn new() -> Self {
        Self { base: div().id("test1") }
    }
}

impl Styled for Test1 {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Test1 {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Test1 {}

impl ParentElement for Test1 {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl IntoElement for Test1 {
    type Element = Stateful<gpui::Div>;

    fn into_element(self) -> Self::Element {
        let Test1 { mut base } = self;
        base = base.overflow_y_scroll();
        base
    }
}

enum TestDirection {
    Vertical,
}

// Test 2: Add Option<ScrollHandle> field AND direction with match
struct Test2 {
    base: Stateful<gpui::Div>,
    scroll_handle: Option<ScrollHandle>,
    direction: TestDirection,
}

impl Test2 {
    fn new() -> Self {
        Self { 
            base: div().id("test2"),
            scroll_handle: None,
            direction: TestDirection::Vertical,
        }
    }
}

impl Styled for Test2 {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Test2 {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Test2 {}

impl ParentElement for Test2 {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl IntoElement for Test2 {
    type Element = Stateful<gpui::Div>;

    fn into_element(self) -> Self::Element {
        let Test2 { mut base, scroll_handle: _, direction } = self;
        
        // Use match like ScrollContainer
        base = match direction {
            TestDirection::Vertical => base.overflow_y_scroll(),
        };
        
        base
    }
}

struct TestExtraFields {}

impl Render for TestExtraFields {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("Test1: Single field"))
            .child(
                Test1::new()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0xff0000))
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xdbeafe))
                            .child("Test1")
                    )
            )
            .child(div().child("Test2: With Option<ScrollHandle> field"))
            .child(
                Test2::new()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0x00ff00))
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xd1fae5))
                            .child("Test2")
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
            |_, cx| cx.new(|_| TestExtraFields {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

