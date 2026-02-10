use adabraka_ui::components::editor::EditorState;
use adabraka_ui::components::icon::Icon;
use adabraka_ui::components::input::{Input, InputEvent, InputState};
use adabraka_ui::theme::use_theme;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use smol::Timer;
use std::time::Duration;

const SEARCH_DEBOUNCE: Duration = Duration::from_millis(150);

actions!(
    search_bar,
    [
        FindNext,
        FindPrevious,
        ToggleCaseSensitive,
        ToggleRegex,
        ReplaceOne,
        ReplaceAllMatches,
        DismissSearch,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("enter", FindNext, Some("SearchBar")),
        KeyBinding::new("shift-enter", FindPrevious, Some("SearchBar")),
        KeyBinding::new("escape", DismissSearch, Some("SearchBar")),
    ]);
}

pub struct SearchBar {
    find_input: Entity<InputState>,
    replace_input: Entity<InputState>,
    editor: Option<Entity<EditorState>>,
    pub show_replace: bool,
    #[allow(dead_code)]
    pub visible: bool,
    dismiss_callback: Option<Box<dyn Fn(&mut App)>>,
    search_task: Option<Task<()>>,
    last_query: SharedString,
}

impl SearchBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let find_input = cx.new(|cx| InputState::new(cx));
        let replace_input = cx.new(|cx| InputState::new(cx));

        cx.subscribe(&find_input, |this, _input, event, cx| {
            if matches!(event, InputEvent::Change) {
                let query = this.find_input.read(cx).content.clone();
                if query == this.last_query {
                    return;
                }
                this.last_query = query.clone();
                this.search_task = None;
                if query.is_empty() {
                    if let Some(editor) = &this.editor {
                        let editor = editor.clone();
                        editor.update(cx, |state, ecx| {
                            state.find_all("", ecx);
                        });
                    }
                    cx.notify();
                    return;
                }

                let bar = cx.entity().clone();
                let task = cx.spawn(async move |_, cx| {
                    Timer::after(SEARCH_DEBOUNCE).await;
                    let _ = bar.update(cx, |this, cx| {
                        let current = this.find_input.read(cx).content.clone();
                        if current != query {
                            return;
                        }
                        if let Some(editor) = &this.editor {
                            let editor = editor.clone();
                            editor.update(cx, |state, ecx| {
                                state.find_all(current.as_ref(), ecx);
                            });
                        }
                        cx.notify();
                    });
                });
                this.search_task = Some(task);
            }
        })
        .detach();

        Self {
            find_input,
            replace_input,
            editor: None,
            show_replace: false,
            visible: false,
            dismiss_callback: None,
            search_task: None,
            last_query: SharedString::from(""),
        }
    }

    pub fn set_editor(&mut self, editor: Entity<EditorState>, cx: &mut Context<Self>) {
        self.editor = Some(editor.clone());
        let query = self.find_input.read(cx).content.clone();
        self.last_query = query.clone();
        if query.is_empty() {
            editor.update(cx, |state, ecx| {
                state.clear_search(ecx);
            });
            cx.notify();
            return;
        }
        editor.update(cx, |state, ecx| {
            state.find_all(query.as_ref(), ecx);
        });
        cx.notify();
    }

    pub fn set_dismiss<F: Fn(&mut App) + 'static>(&mut self, callback: F) {
        self.dismiss_callback = Some(Box::new(callback));
    }

    #[allow(dead_code)]
    pub fn focus_find(&self, window: &mut Window, cx: &App) {
        window.focus(&self.find_input.read(cx).focus_handle(cx));
    }

    pub fn get_prefill_text(&self, cx: &App) -> Option<String> {
        self.editor
            .as_ref()
            .and_then(|e| e.read(cx).selection_text())
    }

    pub fn find_input_entity(&self) -> Entity<InputState> {
        self.find_input.clone()
    }

    pub fn editor_entity(&self) -> Option<Entity<EditorState>> {
        self.editor.clone()
    }

    fn find_next(&mut self, _: &FindNext, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = &self.editor {
            let editor = editor.clone();
            editor.update(cx, |state, ecx| state.find_next(ecx));
        }
        cx.notify();
    }

    fn find_previous(&mut self, _: &FindPrevious, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = &self.editor {
            let editor = editor.clone();
            editor.update(cx, |state, ecx| state.find_previous(ecx));
        }
        cx.notify();
    }

    fn dismiss(&mut self, _: &DismissSearch, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(cb) = &self.dismiss_callback {
            cb(cx);
        }
    }
}

impl Focusable for SearchBar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.find_input.read(cx).focus_handle(cx)
    }
}

impl Render for SearchBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let (match_count, current_idx, case_sensitive, use_regex) =
            if let Some(editor) = &self.editor {
                let state = editor.read(cx);
                (
                    state.search_match_count(),
                    state.current_match_index(),
                    state.search_case_sensitive(),
                    state.search_use_regex(),
                )
            } else {
                (0, None, false, false)
            };

        let match_info = if match_count > 0 {
            format!(
                "{} of {}",
                current_idx.map(|i| i + 1).unwrap_or(0),
                match_count
            )
        } else if !self.find_input.read(cx).content().is_empty() {
            "No results".to_string()
        } else {
            String::new()
        };

        let show_replace = self.show_replace;
        let case_bg = if case_sensitive {
            theme.tokens.primary.opacity(0.2)
        } else {
            theme.tokens.muted.opacity(0.3)
        };
        let case_fg = if case_sensitive {
            theme.tokens.primary
        } else {
            theme.tokens.muted_foreground
        };
        let regex_bg = if use_regex {
            theme.tokens.primary.opacity(0.2)
        } else {
            theme.tokens.muted.opacity(0.3)
        };
        let regex_fg = if use_regex {
            theme.tokens.primary
        } else {
            theme.tokens.muted_foreground
        };
        let btn_bg = theme.tokens.muted.opacity(0.3);
        let btn_fg = theme.tokens.muted_foreground;
        let hover_bg = theme.tokens.muted.opacity(0.5);

        div()
            .key_context("SearchBar")
            .on_action(cx.listener(Self::find_next))
            .on_action(cx.listener(Self::find_previous))
            .on_action(cx.listener(Self::dismiss))
            .w_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.muted.opacity(0.3))
            .border_b_1()
            .border_color(theme.tokens.border)
            .px(px(12.0))
            .pt(px(8.0))
            .pb(px(12.0))
            .gap(px(10.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .id("toggle-replace-btn")
                            .w(px(20.0))
                            .h(px(20.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(hover_bg))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.show_replace = !this.show_replace;
                                cx.notify();
                            }))
                            .child(
                                Icon::new(if show_replace {
                                    "chevron-down"
                                } else {
                                    "chevron-right"
                                })
                                .size(px(14.0))
                                .color(btn_fg),
                            ),
                    )
                    .child(
                        div().flex_1().max_w(px(400.0)).child(
                            Input::new(&self.find_input)
                                .placeholder("Find")
                                .h(px(28.0))
                                .text_size(px(13.0)),
                        ),
                    )
                    .child(
                        div()
                            .id("case-btn")
                            .h(px(24.0))
                            .px(px(8.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .bg(case_bg)
                            .text_color(case_fg)
                            .text_size(px(12.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(hover_bg))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(editor) = &this.editor {
                                    let editor = editor.clone();
                                    editor.update(cx, |state, ecx| {
                                        let val = !state.search_case_sensitive();
                                        state.set_search_case_sensitive(val, ecx);
                                    });
                                }
                                cx.notify();
                            }))
                            .child("Aa"),
                    )
                    .child(
                        div()
                            .id("regex-btn")
                            .h(px(24.0))
                            .px(px(8.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .bg(regex_bg)
                            .text_color(regex_fg)
                            .text_size(px(12.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(hover_bg))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(editor) = &this.editor {
                                    let editor = editor.clone();
                                    editor.update(cx, |state, ecx| {
                                        let val = !state.search_use_regex();
                                        state.set_search_regex(val, ecx);
                                    });
                                }
                                cx.notify();
                            }))
                            .child(".*"),
                    )
                    .child(
                        div()
                            .id("prev-btn")
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(btn_fg)
                            .hover(|s| s.bg(hover_bg))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(editor) = &this.editor {
                                    let editor = editor.clone();
                                    editor.update(cx, |state, ecx| state.find_previous(ecx));
                                }
                                cx.notify();
                            }))
                            .child("\u{2191}"),
                    )
                    .child(
                        div()
                            .id("next-btn")
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(btn_fg)
                            .hover(|s| s.bg(hover_bg))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(editor) = &this.editor {
                                    let editor = editor.clone();
                                    editor.update(cx, |state, ecx| state.find_next(ecx));
                                }
                                cx.notify();
                            }))
                            .child("\u{2193}"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .min_w(px(70.0))
                            .child(match_info),
                    )
                    .child(
                        div()
                            .id("close-search-btn")
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(btn_fg)
                            .hover(|s| s.bg(hover_bg))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(cb) = &this.dismiss_callback {
                                    cb(cx);
                                }
                            }))
                            .child("\u{00D7}"),
                    ),
            )
            .when(show_replace, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(div().w(px(20.0)).flex_shrink_0())
                        .child(
                            div().flex_1().max_w(px(400.0)).child(
                                Input::new(&self.replace_input)
                                    .placeholder("Replace")
                                    .h(px(28.0))
                                    .text_size(px(13.0)),
                            ),
                        )
                        .child(
                            div()
                                .id("replace-btn")
                                .h(px(24.0))
                                .px(px(8.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(4.0))
                                .bg(btn_bg)
                                .text_color(btn_fg)
                                .text_size(px(12.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(hover_bg))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    let replacement =
                                        this.replace_input.read(cx).content().to_string();
                                    if let Some(editor) = &this.editor {
                                        let editor = editor.clone();
                                        editor.update(cx, |state, ecx| {
                                            state.replace_current(&replacement, ecx);
                                        });
                                    }
                                    cx.notify();
                                }))
                                .child("Replace"),
                        )
                        .child(
                            div()
                                .id("replace-all-btn")
                                .h(px(24.0))
                                .px(px(8.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(4.0))
                                .bg(btn_bg)
                                .text_color(btn_fg)
                                .text_size(px(12.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(hover_bg))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    let replacement =
                                        this.replace_input.read(cx).content().to_string();
                                    if let Some(editor) = &this.editor {
                                        let editor = editor.clone();
                                        editor.update(cx, |state, ecx| {
                                            state.replace_all(&replacement, ecx);
                                        });
                                    }
                                    cx.notify();
                                }))
                                .child("All"),
                        ),
                )
            })
    }
}
