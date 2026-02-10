use adabraka_ui::components::editor::{EditorState, Language};
use adabraka_ui::components::icon::Icon;
use adabraka_ui::theme::use_theme;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::sync::Arc;

#[derive(IntoElement)]
pub struct StatusBarView {
    pub line: usize,
    pub col: usize,
    pub language: Language,
    pub is_modified: bool,
    pub line_count: usize,
    pub terminal_open: bool,
    pub on_toggle_terminal: Option<Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>>,
}

impl StatusBarView {
    pub fn from_editor(state: &EditorState) -> Self {
        let cursor = state.cursor();
        Self {
            line: cursor.line + 1,
            col: cursor.col + 1,
            language: state.language(),
            is_modified: state.is_modified(),
            line_count: state.line_count(),
            terminal_open: false,
            on_toggle_terminal: None,
        }
    }

    pub fn terminal_open(mut self, open: bool) -> Self {
        self.terminal_open = open;
        self
    }

    pub fn on_toggle_terminal(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_toggle_terminal = Some(Arc::new(handler));
        self
    }
}

impl RenderOnce for StatusBarView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let modified_color = theme.tokens.primary;
        let muted_fg = theme.tokens.muted_foreground;
        let terminal_icon_color = if self.terminal_open {
            theme.tokens.primary
        } else {
            muted_fg
        };

        div()
            .w_full()
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_between()
            .px(px(16.0))
            .bg(theme.tokens.muted.opacity(0.5))
            .border_t_1()
            .border_color(theme.tokens.border)
            .text_size(px(12.0))
            .text_color(muted_fg)
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .child(format!("Ln {}, Col {}", self.line, self.col))
                    .child(format!("{} lines", self.line_count))
                    .when(self.is_modified, |el| {
                        el.child(div().text_color(modified_color).child("Modified"))
                    }),
            )
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .items_center()
                    .child("UTF-8")
                    .child(self.language.display_name())
                    .child({
                        let btn = div()
                            .id("terminal-toggle-btn")
                            .w(px(22.0))
                            .h(px(22.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.tokens.muted.opacity(0.8)))
                            .child(
                                Icon::new("terminal")
                                    .size(px(14.0))
                                    .color(terminal_icon_color),
                            );
                        if let Some(handler) = self.on_toggle_terminal {
                            btn.on_click(move |_, window, cx| {
                                handler(window, cx);
                            })
                        } else {
                            btn
                        }
                    }),
            )
    }
}
