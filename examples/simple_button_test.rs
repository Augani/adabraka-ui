use adabraka_ui::prelude::*;
use gpui::*;
use std::sync::{Arc, Mutex};

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Simple Button Test".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point { x: px(100.0), y: px(100.0) },
                    size: Size { width: px(400.0), height: px(300.0) },
                })),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| SimpleButtonApp::new(window, cx))
            },
        )
        .unwrap();
    });
}

struct SimpleButtonApp {
    click_count: Arc<Mutex<usize>>,
}

impl SimpleButtonApp {
    fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {
            click_count: Arc::new(Mutex::new(0)),
        }
    }
}

impl Render for SimpleButtonApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let count = *self.click_count.lock().unwrap();
        let click_count = Arc::clone(&self.click_count);

        div()
            .bg(theme.tokens.background)
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(20.0))
            .child(
                div()
                    .text_size(px(24.0))
                    .text_color(theme.tokens.foreground)
                    .child(format!("Clicked {} times", count))
            )
            .child(
                // Using Button with a regular closure (gc pattern)
                Button::new("click-btn", "Click Me!")
                    .on_click(move |_event, _window, _cx| {
                        let mut count = click_count.lock().unwrap();
                        *count += 1;
                        println!("Button clicked! Count: {}", *count);
                        // Can't call cx.notify() here because we don't have view access
                    })
            )
    }
}
