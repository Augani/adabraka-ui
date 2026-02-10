use adabraka_ui::components::editor::{EditorState, Language};
use adabraka_ui::theme::use_theme;
use gpui::prelude::FluentBuilder as _;
use gpui::*;

#[derive(IntoElement)]
pub struct StatusBarView {
    pub line: usize,
    pub col: usize,
    pub language: Language,
    pub is_modified: bool,
    pub line_count: usize,
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
        }
    }
}

impl RenderOnce for StatusBarView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let modified_color = theme.tokens.primary;

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
            .text_color(theme.tokens.muted_foreground)
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
                    .child("UTF-8")
                    .child(self.language.display_name()),
            )
    }
}
