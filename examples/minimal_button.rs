use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Minimal Button Test".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| MinimalButtonTest::new(window, cx))
            },
        )
        .unwrap();
    });
}

struct MinimalButtonTest {
    click_count: usize,
}

impl MinimalButtonTest {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());
        Self { click_count: 0 }
    }
}

impl Render for MinimalButtonTest {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("[MinimalButtonTest::render] Rendering with click_count: {}", self.click_count);

        div()
            .bg(use_theme().tokens.background)
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .flex_col()
            .gap(px(20.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .text_color(use_theme().tokens.foreground)
                    .child(format!("Clicked {} times", self.click_count))
            )
            .child(
                Button::new("click-btn", "Click Me!")
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        println!("[MinimalButtonTest] BUTTON CLICKED! Incrementing from {} to {}",
                                 view.click_count, view.click_count + 1);
                        view.click_count += 1;
                        cx.notify();
                    }))
            )
    }
}
