use gpui::*;
use adabraka_ui::{
    theme::{install_theme, Theme, use_theme},
    components::{
        text::{h1, h2, body, caption, code_small},
        button::{Button, ButtonVariant},
        editor::{Editor, EditorState},
    },
};

struct EditorDemo {
    editor_state: Entity<EditorState>,
    show_line_numbers: bool,
}

impl EditorDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let editor_state = cx.new(|cx| {
            let mut state = EditorState::new(cx);

            // Set initial SQL content
            state.set_content(
                "-- Sample SQL Query\n\
                SELECT \n\
                    users.id,\n\
                    users.name,\n\
                    users.email,\n\
                    COUNT(orders.id) as order_count,\n\
                    SUM(orders.total) as total_spent\n\
                FROM users\n\
                LEFT JOIN orders ON users.id = orders.user_id\n\
                WHERE users.created_at >= '2024-01-01'\n\
                GROUP BY users.id, users.name, users.email\n\
                HAVING COUNT(orders.id) > 0\n\
                ORDER BY total_spent DESC\n\
                LIMIT 100;\n\
                \n\
                -- Try editing this query!\n\
                -- Keyboard shortcuts:\n\
                -- Ctrl+A: Select all\n\
                -- Ctrl+C: Copy\n\
                -- Ctrl+V: Paste",
                cx,
            );
            state
        });

        Self {
            editor_state,
            show_line_numbers: true,
        }
    }
}

impl Render for EditorDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("SQL Editor"))
                    .child(caption("A multi-line code editor with syntax highlighting and keyboard shortcuts"))
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .mt(px(12.0))
                            .child(
                                Button::new("toggle-line-numbers-btn", "Toggle Line Numbers")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.show_line_numbers = !this.show_line_numbers;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("clear-content-btn", "Clear Content")
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.editor_state.update(cx, |state, cx| {
                                            state.set_content("", cx);
                                        });
                                    }))
                            )
                            .child(
                                Button::new("reset-sample-btn", "Reset to Sample")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.editor_state.update(cx, |state, cx| {
                                            state.set_content(
                                                "-- Sample SQL Query\n\
                                                SELECT \n\
                                                    users.id,\n\
                                                    users.name,\n\
                                                    users.email,\n\
                                                    COUNT(orders.id) as order_count,\n\
                                                    SUM(orders.total) as total_spent\n\
                                                FROM users\n\
                                                LEFT JOIN orders ON users.id = orders.user_id\n\
                                                WHERE users.created_at >= '2024-01-01'\n\
                                                GROUP BY users.id, users.name, users.email\n\
                                                HAVING COUNT(orders.id) > 0\n\
                                                ORDER BY total_spent DESC\n\
                                                LIMIT 100;",
                                                cx,
                                            );
                                        });
                                    }))
                            )
                    )
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .overflow_hidden()
                    .child(
                        // Editor area
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .p(px(24.0))
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .rounded(theme.tokens.radius_lg)
                                    .overflow_hidden()
                                    .bg(theme.tokens.card)
                                    .child(
                                        Editor::new(&self.editor_state)
                                            .min_lines(20)
                                            .show_line_numbers(self.show_line_numbers, cx)
                                            .show_border(false)
                                    )
                            )
                    )
                    .child(
                        // Info sidebar
                        div()
                            .w(px(350.0))
                            .p(px(24.0))
                            .border_l_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(16.0))
                                    .child(h2("Features"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(12.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(body("Syntax Highlighting").weight(FontWeight::SEMIBOLD))
                                                    .child(caption("SQL syntax highlighting powered by tree-sitter"))
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(body("Line Numbers").weight(FontWeight::SEMIBOLD))
                                                    .child(caption("Toggle line numbers on/off"))
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(body("Text Selection").weight(FontWeight::SEMIBOLD))
                                                    .child(caption("Mouse drag to select, keyboard shortcuts supported"))
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(body("Cursor Navigation").weight(FontWeight::SEMIBOLD))
                                                    .child(caption("Arrow keys, Home, End, Page Up/Down"))
                                            )
                                    )
                                    .child(
                                        div()
                                            .mt(px(8.0))
                                            .child(h2("Keyboard Shortcuts"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .p(px(12.0))
                                            .rounded(theme.tokens.radius_md)
                                            .bg(theme.tokens.muted.opacity(0.3))
                                            .child(shortcut_row("Ctrl+A", "Select All"))
                                            .child(shortcut_row("Ctrl+C", "Copy"))
                                            .child(shortcut_row("Ctrl+X", "Cut"))
                                            .child(shortcut_row("Ctrl+V", "Paste"))
                                            .child(shortcut_row("Ctrl+Z", "Undo"))
                                            .child(shortcut_row("Ctrl+Shift+Z", "Redo"))
                                            .child(shortcut_row("Ctrl+Home", "Go to Start"))
                                            .child(shortcut_row("Ctrl+End", "Go to End"))
                                    )
                                    .child(
                                        div()
                                            .mt(px(8.0))
                                            .child(h2("Usage"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .p(px(12.0))
                                            .rounded(theme.tokens.radius_md)
                                            .bg(theme.tokens.muted.opacity(0.3))
                                            .child(code_small("let state = cx.new(|cx| {"))
                                            .child(code_small("  EditorState::new(cx)"))
                                            .child(code_small("});"))
                                            .child(code_small(""))
                                            .child(code_small("Editor::new(state)"))
                                            .child(code_small("  .min_lines(20)"))
                                    )
                            )
                    )
            )
    }
}

fn shortcut_row(keys: impl Into<String>, description: impl Into<String>) -> impl IntoElement {
    div()
        .flex()
        .justify_between()
        .items_center()
        .child(code_small(keys.into()))
        .child(caption(description.into()))
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize the UI library
        adabraka_ui::init(cx);

        // Install dark theme
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(1400.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("SQL Editor Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| EditorDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
