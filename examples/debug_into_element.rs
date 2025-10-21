// Debug to see if into_element is being called

use adabraka_ui::layout::{ScrollContainer, ScrollDirection};
use gpui::{
    div, prelude::*, px, rgb, App, Application, Bounds, Context,
    Window, WindowBounds, WindowOptions, size, AnyElement, Stateful,
    ParentElement, Styled, StyleRefinement, InteractiveElement, Interactivity,
    StatefulInteractiveElement, ElementId,
};

// Create our own version WITH debug prints
struct DebugScrollContainer {
    base: Stateful<gpui::Div>,
    direction: ScrollDirection,
}

impl DebugScrollContainer {
    fn vertical() -> Self {
        eprintln!("DebugScrollContainer::vertical() called");
        Self {
            base: div().id("debug-scroll"),
            direction: ScrollDirection::Vertical,
        }
    }
}

impl Styled for DebugScrollContainer {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for DebugScrollContainer {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for DebugScrollContainer {}

impl ParentElement for DebugScrollContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl IntoElement for DebugScrollContainer {
    type Element = Stateful<gpui::Div>;

    fn into_element(self) -> Self::Element {
        eprintln!("DebugScrollContainer::into_element() CALLED!");
        let DebugScrollContainer { mut base, direction } = self;
        
        base = match direction {
            ScrollDirection::Vertical => {
                eprintln!("Applying overflow_y_scroll()");
                base.overflow_y_scroll()
            }
            ScrollDirection::Horizontal => base.overflow_x_scroll(),
            ScrollDirection::Both => base.overflow_scroll(),
        };
        
        eprintln!("Returning base from into_element");
        base
    }
}

struct DebugTest {}

impl Render for DebugTest {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        eprintln!("\n=== RENDER CALLED ===");
        
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .child(div().child("Debug ScrollContainer:"))
            .child({
                eprintln!("Building DebugScrollContainer...");
                DebugScrollContainer::vertical()
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
                            .child("Debug content")
                    )
            })
            .child(div().child("Actual library ScrollContainer:"))
            .child({
                eprintln!("Building ScrollContainer from library...");
                ScrollContainer::vertical()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0xff0000))
                    .bg(rgb(0xfafafa))
                    .p(px(12.0))
                    .child(
                        div()
                            .h(px(800.))
                            .bg(rgb(0xdbeafe))
                            .child("Library content")
                    )
            })
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
            |_, cx| cx.new(|_| DebugTest {}),
        )
        .unwrap();
        cx.activate(true);
    });
}

