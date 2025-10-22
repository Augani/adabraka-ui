use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Demo Without Scroll".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| DemoApp::new(window, cx))
            },
        )
        .unwrap();
    });
}

struct DemoApp {
    click_count: usize,
}

impl DemoApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());
        Self { click_count: 0 }
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        println!("[DemoApp::render] Rendering with click_count: {}", self.click_count);

        div()
            .bg(theme.tokens.background)
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .px(px(24.0))
                    .py(px(16.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("Demo - Button in ROOT view")
                    )
            )
            .child(
                // Content DIRECTLY in root view - like minimal_button.rs
                div()
                    .flex_1()
                    .w_full()
                    .p(px(24.0))
                    .flex()
                    .flex_col()
                    .gap(px(32.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.tokens.foreground)
                                    .child("Buttons (in ROOT view)")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .child(
                                        Button::new("click-btn", "Click Me!")
                                            .on_click(cx.listener(|view, _event, _window, cx| {
                                                println!("[DemoApp] BUTTON CLICKED! Count: {} -> {}",
                                                         view.click_count, view.click_count + 1);
                                                view.click_count += 1;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(format!("Button clicked {} times", self.click_count))
                            )
                    )
            )
    }
}
