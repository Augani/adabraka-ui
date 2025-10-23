//! Demo showing styled Editor component examples
//!
//! This example demonstrates various styling capabilities of the Editor component
//! using the Styled trait to customize appearance while maintaining functionality.

use adabraka_ui::{
    prelude::*,
    components::editor::{Editor, EditorState},
    components::scrollable::scrollable_vertical,
};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Editor Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| EditorStyledDemo::new()),
            )
            .unwrap();
        });
}

struct EditorStyledDemo;

impl EditorStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for EditorStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .p_6()
                        .gap_6()
                    // Example 1: Default styled editor
                    .child(demo_section(
                        "Default Editor",
                        "Standard editor with default styling",
                        {
                            let editor = cx.new(|cx| EditorState::new(cx));
                            Editor::new(&editor)
                                .content("-- Default SQL Editor\nSELECT id, name, email\nFROM users\nWHERE active = true\nORDER BY created_at DESC;", cx)
                                .min_lines(5)
                        },
                    ))
                    // Example 2: Custom background color
                    .child(demo_section(
                        "Custom Background",
                        "Editor with custom background color and padding",
                        {
                            let editor = cx.new(|cx| EditorState::new(cx));
                            Editor::new(&editor)
                                .content("-- Custom Background\nSELECT * FROM products\nWHERE price > 100;", cx)
                                .min_lines(4)
                                .bg(rgb(0x1e1e2e))
                                .p_4()
                        },
                    ))
                    // Example 3: Custom border and rounded corners
                    .child(demo_section(
                        "Custom Border & Radius",
                        "Editor with thick colored border and large border radius",
                        {
                            let editor = cx.new(|cx| EditorState::new(cx));
                            Editor::new(&editor)
                                .content("-- Styled Border\nINSERT INTO logs (message, level)\nVALUES ('System started', 'INFO');", cx)
                                .min_lines(4)
                                .border_2()
                                .border_color(rgb(0x89b4fa))
                                .rounded_lg()
                        },
                    ))
                    // Example 4: Shadow and elevation
                    .child(demo_section(
                        "Shadow & Elevation",
                        "Editor with shadow for elevated appearance",
                        {
                            let editor = cx.new(|cx| EditorState::new(cx));
                            Editor::new(&editor)
                                .content("-- With Shadow\nUPDATE settings\nSET theme = 'dark'\nWHERE user_id = 123;", cx)
                                .min_lines(4)
                                .shadow_lg()
                        },
                    ))
                    // Example 5: Custom width constraints
                    .child(demo_section(
                        "Width Constraints",
                        "Editor with custom width and centered layout",
                        {
                            let editor = cx.new(|cx| EditorState::new(cx));
                            div()
                                .flex()
                                .justify_center()
                                .child(
                                    Editor::new(&editor)
                                        .content("-- Constrained Width\nSELECT COUNT(*) FROM orders;", cx)
                                        .min_lines(3)
                                        .w(px(500.0))
                                )
                        },
                    ))
                    // Example 6: Combined custom styling
                    .child(demo_section(
                        "Combined Styles",
                        "Editor with multiple custom styles combined",
                        {
                            let editor = cx.new(|cx| EditorState::new(cx));
                            Editor::new(&editor)
                                .content("-- Multiple Custom Styles\nDELETE FROM cache\nWHERE expires_at < NOW();", cx)
                                .min_lines(4)
                                .bg(rgb(0x2d2d44))
                                .border_2()
                                .border_color(rgb(0xf38ba8))
                                .rounded_xl()
                                .p_6()
                                .shadow_md()
                        },
                    ))
                )
            )
    }
}

fn demo_section(
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    content: impl IntoElement,
) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.tokens.foreground)
                        .child(title.into())
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.tokens.muted_foreground)
                        .child(description.into())
                )
        )
        .child(content)
}
