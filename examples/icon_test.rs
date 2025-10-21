use adabraka_ui::{
    prelude::*,
    components::icon::Icon,
};
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        adabraka_ui::init(cx);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Icon Test".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(600.0), px(400.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| IconTestApp::new(window, cx)),
        )
        .unwrap();
    });
}

struct IconTestApp {}

impl IconTestApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::light();
        install_theme(cx, theme.clone());
        Self {}
    }
}

impl Render for IconTestApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(theme.tokens.background)
            .gap_8()
            .child(
                div()
                    .text_2xl()
                    .text_color(theme.tokens.foreground)
                    .child("Icon Test - If you see icons below, it works!")
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    .items_center()
                    // Test 1: Simple named icon
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new("heart")
                                    .size(px(64.0))
                                    .color(rgb(0xff0000).into())
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.tokens.foreground)
                                    .child("heart (red)")
                            )
                    )
                    // Test 2: Arrow down
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new("arrow-down")
                                    .size(px(64.0))
                                    .color(rgb(0x0000ff).into())
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.tokens.foreground)
                                    .child("arrow-down (blue)")
                            )
                    )
                    // Test 3: Check mark
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new("check")
                                    .size(px(64.0))
                                    .color(rgb(0x00ff00).into())
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.tokens.foreground)
                                    .child("check (green)")
                            )
                    )
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.tokens.muted_foreground)
                    .child("Icons should be rendered above in different colors")
            )
    }
}
