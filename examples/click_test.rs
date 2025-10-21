use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Click Test".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point { x: px(100.0), y: px(100.0) },
                    size: Size { width: px(400.0), height: px(300.0) },
                })),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| ClickTestApp::new(window, cx))
            },
        )
        .unwrap();
    });
}

struct ClickTestApp {
    click_count: usize,
}

impl ClickTestApp {
    fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {
            click_count: 0,
        }
    }
}

impl Render for ClickTestApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(20.0))
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .text_size(px(24.0))
                    .text_color(rgb(0xffffff))
                    .child(format!("Clicked {} times", self.click_count))
            )
            .child(
                // Test 1: Simple div with on_click
                div()
                    .id("simple-div")
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(rgb(0x0078d4))
                    .text_color(rgb(0xffffff))
                    .rounded(px(4.0))
                    .cursor(CursorStyle::PointingHand)
                    .child("Simple Div (on_click)")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.click_count += 1;
                        cx.notify();
                        println!("Simple div clicked! Count: {}", view.click_count);
                    }))
            )
            .child(
                // Test 2: Stateful div with on_click
                div()
                    .id("stateful-div")
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(rgb(0x107c10))
                    .text_color(rgb(0xffffff))
                    .rounded(px(4.0))
                    .cursor(CursorStyle::PointingHand)
                    .child("Stateful<Div> (on_click)")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.click_count += 1;
                        cx.notify();
                        println!("Stateful div clicked! Count: {}", view.click_count);
                    }))
            )
            .child(
                // Test 3: gc-style double on_click
                div()
                    .id("gc-style")
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(rgb(0xd83b01))
                    .text_color(rgb(0xffffff))
                    .rounded(px(4.0))
                    .cursor(CursorStyle::PointingHand)
                    .child("gc-style (double on_click)")
                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                        window.prevent_default();
                    })
                    .on_click(cx.listener(|_view, _, _, cx| {
                        cx.stop_propagation();
                    }))
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        view.click_count += 1;
                        cx.notify();
                        println!("gc-style clicked! Count: {}", view.click_count);
                    }))
            )
    }
}
