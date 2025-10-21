// Test horizontal scrolling using the ScrollContainer from our library

use adabraka_ui::layout::ScrollContainer;
use gpui::{
    div, prelude::*, px, rgb, App, Application, Bounds, Context,
    Window, WindowBounds, WindowOptions, size,
};

struct TestHorizontalScroll {}

impl Render for TestHorizontalScroll {
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
                    .overflow_x_scroll()
                    .border_1()
                    .border_color(rgb(0xff0000))
                    .bg(rgb(0xfafafa))
                    .p_4()
                    .child(
                        div()
                            .w(px(2000.))
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_nowrap()
                                    .gap_4()
                                    .h_full()
                                    .children(
                                        (0..20).map(|i| {
                                            div()
                                                .w(px(150.))
                                                .flex_shrink_0()
                                                .h_full()
                                                .bg(if i % 2 == 0 { rgb(0xdbeafe) } else { rgb(0xfecaca) })
                                                .border_1()
                                                .rounded(px(4.0))
                                                .p_2()
                                                .child(format!("Item {}", i + 1))
                                        })
                                    )
                            )
                    )
            )
            .child(div().child("ScrollContainer::horizontal() with overlay bars:"))
            .child(
                ScrollContainer::horizontal()
                    .with_scrollbar()
                    .horizontal_bar_top()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0x00ff00))
                    .bg(rgb(0xfafafa))
                    .p(px(12.0))
                    .child(
                        div()
                            .w(px(2000.))
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_nowrap()
                                    .gap_4()
                                    .h_full()
                                    .children(
                                        (0..20).map(|i| {
                                            div()
                                                .w(px(150.))
                                                .flex_shrink_0()
                                                .h_full()
                                                .bg(if i % 2 == 0 { rgb(0xd1fae5) } else { rgb(0xfed7d7) })
                                                .border_1()
                                                .rounded(px(4.0))
                                                .p_2()
                                                .child(format!("Item {}", i + 1))
                                        })
                                    )
                            )
                    )
            )
            .child(div().child("ScrollContainer::horizontal() WITHOUT overlay bars:"))
            .child(
                ScrollContainer::horizontal()
                    .with_scrollbar()
                    .horizontal_bar_bottom()
                    .h(px(200.))
                    .w_full()
                    .border_1()
                    .border_color(rgb(0x0000ff))
                    .bg(rgb(0xfafafa))
                    .p(px(12.0))
                    .child(
                        div()
                            .w(px(2000.))
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_nowrap()
                                    .gap_4()
                                    .h_full()
                                    .children(
                                        (0..20).map(|i| {
                                            div()
                                                .w(px(150.))
                                                .flex_shrink_0()
                                                .h_full()
                                                .bg(if i % 2 == 0 { rgb(0xffe4b5) } else { rgb(0xe6e6fa) })
                                                .border_1()
                                                .rounded(px(4.0))
                                                .p_2()
                                                .child(format!("Item {}", i + 1))
                                        })
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TestHorizontalScroll {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
