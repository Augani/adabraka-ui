//! Test example for editor with scrollbars

use adabraka_ui::components::editor::{Editor, EditorState};
use gpui::*;

struct EditorScrollTestApp {
    editor: Entity<EditorState>,
}

impl EditorScrollTestApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| EditorState::new(cx));
        Self { editor }
    }
}

impl Render for EditorScrollTestApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(600.0))
                    .h(px(400.0))
                    .m(px(50.0))
                    .bg(rgb(0x313244))
                    .rounded_lg()
                    .p(px(20.0))
                    .child(
                        // The Editor already wraps itself with scrollable internally, so just use it directly
                        Editor::new(&self.editor)
                            .content(
                                "SELECT * FROM users WHERE id > 100;\n\n-- This is a long SQL query to test scrolling\n-- Add more lines to see the scrollbar\n\nSELECT name, email, created_at\nFROM users\nJOIN profiles ON users.id = profiles.user_id\nWHERE users.active = true\nORDER BY users.created_at DESC;\n\n-- Even more SQL to test vertical scrolling\n\nCREATE TABLE test_table (\n    id SERIAL PRIMARY KEY,\n    name VARCHAR(255) NOT NULL,\n    email VARCHAR(255) UNIQUE,\n    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n);\n\nINSERT INTO test_table (name, email) VALUES\n('John Doe', 'john@example.com'),\n('Jane Smith', 'jane@example.com'),\n('Bob Johnson', 'bob@example.com');\n\n-- More content for scrolling\nSELECT * FROM test_table;\n\n-- Final comment\n-- This should be enough content to trigger scrolling",
                                cx,
                            )
                    )
            )
            .on_mouse_down(MouseButton::Left, {
                let editor = self.editor.clone();
                move |_event, window, cx| {
                    // Focus the editor when clicking anywhere in the app
                    let focus_handle = editor.read(cx).focus_handle(cx);
                    window.focus(&focus_handle);
                }
            })
    }
}

fn main() {
    Application::new().run(|cx| {
        // Initialize adabraka-ui
        adabraka_ui::init(cx);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Editor with Scrollbars Test".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| EditorScrollTestApp::new(cx)),
        )
        .unwrap();
    });
}
