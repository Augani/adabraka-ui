use crate::autosave::AutosaveManager;
use crate::search_bar::SearchBar;
use crate::status_bar::StatusBarView;
use adabraka_ui::components::editor::{Editor, EditorState};
use adabraka_ui::components::input::{Input, InputState};
use adabraka_ui::components::icon::Icon;
use adabraka_ui::theme::{install_theme, use_theme, Theme};
use gpui::prelude::FluentBuilder as _;
use gpui::EntityId;
use gpui::*;
use smol::Timer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

const AUTOSAVE_DELAY: Duration = Duration::from_secs(2);

actions!(
    shiori,
    [
        SaveFile,
        CloseTab,
        OpenFile,
        NewFile,
        NextTab,
        PrevTab,
        ToggleSearch,
        ToggleSearchReplace,
        CloseSearch,
        GotoLine,
        CloseGotoLine,
    ]
);

pub fn init(cx: &mut App) {
    crate::search_bar::init(cx);
    cx.bind_keys([
        KeyBinding::new("cmd-s", SaveFile, Some("ShioriApp")),
        KeyBinding::new("cmd-w", CloseTab, Some("ShioriApp")),
        KeyBinding::new("cmd-o", OpenFile, Some("ShioriApp")),
        KeyBinding::new("cmd-n", NewFile, Some("ShioriApp")),
        KeyBinding::new("ctrl-tab", NextTab, Some("ShioriApp")),
        KeyBinding::new("ctrl-shift-tab", PrevTab, Some("ShioriApp")),
        KeyBinding::new("cmd-f", ToggleSearch, Some("ShioriApp")),
        KeyBinding::new("cmd-h", ToggleSearchReplace, Some("ShioriApp")),
        KeyBinding::new("cmd-g", GotoLine, Some("ShioriApp")),
    ]);
}

pub struct AppState {
    focus_handle: FocusHandle,
    buffers: Vec<Entity<EditorState>>,
    buffer_index: HashMap<EntityId, usize>,
    active_tab: usize,
    autosave: AutosaveManager,
    tab_meta: Vec<TabMeta>,
    search_bar: Entity<SearchBar>,
    search_visible: bool,
    goto_line_visible: bool,
    goto_line_input: Entity<InputState>,
    tab_scroll_offset: usize,
    theme_selector_open: bool,
}

struct TabMeta {
    file_path: Option<PathBuf>,
    file_name: Option<String>,
    modified: bool,
    title: SharedString,
}

impl AppState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let buffer = cx.new(|cx| EditorState::new(cx));
        let goto_line_input = cx.new(|cx| InputState::new(cx));

        cx.observe(&buffer, Self::on_buffer_changed).detach();

        let app_entity = cx.entity().clone();
        let search_bar = cx.new(|cx| {
            let mut bar = SearchBar::new(cx);
            bar.set_dismiss(move |cx| {
                app_entity.update(cx, |this, cx| {
                    this.close_search_internal(cx);
                });
            });
            bar
        });

        let mut buffer_index = HashMap::new();
        buffer_index.insert(buffer.entity_id(), 0);
        let tab_meta = vec![Self::build_tab_meta(&buffer, 0, cx)];

        Self {
            focus_handle,
            buffers: vec![buffer],
            buffer_index,
            active_tab: 0,
            autosave: AutosaveManager::new(1),
            tab_meta,
            search_bar,
            search_visible: false,
            goto_line_visible: false,
            goto_line_input,
            tab_scroll_offset: 0,
            theme_selector_open: false,
        }
    }

    fn build_tab_meta(buffer: &Entity<EditorState>, idx: usize, cx: &App) -> TabMeta {
        let state = buffer.read(cx);
        let file_path = state.file_path().cloned();
        let file_name = file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string());
        let modified = state.is_modified();
        let title = Self::compose_tab_title(file_name.as_deref(), idx, modified);
        TabMeta {
            file_path,
            file_name,
            modified,
            title,
        }
    }

    fn compose_tab_title(name: Option<&str>, idx: usize, modified: bool) -> SharedString {
        let base = match name {
            Some(name) => name.to_string(),
            None => format!("Untitled {}", idx + 1),
        };
        let title = if modified {
            format!("{} \u{2022}", base)
        } else {
            base
        };
        SharedString::from(title)
    }

    fn update_tab_meta_at(&mut self, idx: usize, cx: &App) {
        if idx >= self.buffers.len() || idx >= self.tab_meta.len() {
            return;
        }
        let state = self.buffers[idx].read(cx);
        let file_path = state.file_path();
        let modified = state.is_modified();

        let meta = &mut self.tab_meta[idx];
        let mut changed = false;

        let file_path_changed = match (&meta.file_path, file_path) {
            (Some(prev), Some(current)) => prev != current,
            (None, None) => false,
            _ => true,
        };

        if file_path_changed {
            meta.file_path = file_path.cloned();
            meta.file_name = meta
                .file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string());
            changed = true;
        }

        if meta.modified != modified {
            meta.modified = modified;
            changed = true;
        }

        if changed {
            meta.title = Self::compose_tab_title(meta.file_name.as_deref(), idx, meta.modified);
        }
    }

    fn refresh_untitled_titles_from(&mut self, start: usize) {
        for idx in start..self.tab_meta.len() {
            if self.tab_meta[idx].file_path.is_none() {
                let modified = self.tab_meta[idx].modified;
                self.tab_meta[idx].title = Self::compose_tab_title(None, idx, modified);
            }
        }
    }

    fn add_buffer(&mut self, buffer: Entity<EditorState>, cx: &App) {
        let idx = self.buffers.len();
        self.buffer_index.insert(buffer.entity_id(), idx);
        self.tab_meta.push(Self::build_tab_meta(&buffer, idx, cx));
        self.buffers.push(buffer);
        self.autosave.push();
        self.active_tab = idx;
    }

    fn remove_buffer_at(&mut self, idx: usize) {
        if idx >= self.buffers.len() {
            return;
        }
        let buffer = self.buffers.remove(idx);
        self.tab_meta.remove(idx);
        self.autosave.remove(idx);
        self.buffer_index.remove(&buffer.entity_id());
        for i in idx..self.buffers.len() {
            let id = self.buffers[i].entity_id();
            self.buffer_index.insert(id, i);
        }
        self.refresh_untitled_titles_from(idx);
    }

    pub fn open_paths(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        for path in paths {
            let buffer = cx.new(|cx| {
                let mut state = EditorState::new(cx);
                state.load_file(&path, cx);
                state
            });
            cx.observe(&buffer, Self::on_buffer_changed).detach();
            self.add_buffer(buffer, cx);
        }
        self.clamp_tab_scroll();
        self.update_search_editor(cx);
        cx.notify();
    }

    fn on_buffer_changed(&mut self, buffer: Entity<EditorState>, cx: &mut Context<Self>) {
        if let Some(&idx) = self.buffer_index.get(&buffer.entity_id()) {
            self.update_tab_meta_at(idx, cx);
            let buf = buffer.clone();
            let task = cx.spawn(async move |_, cx| {
                Timer::after(AUTOSAVE_DELAY).await;
                let _ = cx.update(|cx| {
                    buf.update(cx, |state, cx| {
                        if let Some(path) = state.file_path().cloned() {
                            if state.is_modified() {
                                state.save_to_file(path, cx);
                            }
                        }
                    });
                });
            });
            self.autosave.set(idx, task);
        }
        cx.notify();
    }

    fn save_active(&mut self, cx: &mut Context<Self>) {
        if let Some(buffer) = self.buffers.get(self.active_tab) {
            let has_path = buffer.read(cx).file_path().is_some();
            if has_path {
                let buffer = buffer.clone();
                buffer.update(cx, |state, cx| {
                    if let Some(path) = state.file_path().cloned() {
                        state.save_to_file(path, cx);
                    }
                });
            } else {
                let buffer = buffer.clone();
                let rx = cx.prompt_for_new_path(Path::new(""), Some("untitled.txt"));
                cx.spawn(async move |this, cx| {
                    if let Ok(Ok(Some(path))) = rx.await {
                        let _ = cx.update(|cx| {
                            buffer.update(cx, |state, cx| {
                                state.save_to_file(path, cx);
                            });
                            let _ = this.update(cx, |_, cx| cx.notify());
                        });
                    }
                })
                .detach();
            }
        }
    }

    fn close_active_tab(&mut self, cx: &mut Context<Self>) {
        if self.buffers.len() <= 1 {
            return;
        }
        let idx = self.active_tab;
        self.autosave.cancel(idx);
        self.remove_buffer_at(idx);
        if self.active_tab >= self.buffers.len() {
            self.active_tab = self.buffers.len().saturating_sub(1);
        }
        self.clamp_tab_scroll();
        self.update_search_editor(cx);
        cx.notify();
    }

    fn open_file_dialog(&mut self, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await {
                let _ = cx.update(|cx| {
                    let _ = this.update(cx, |this, cx| {
                        this.open_paths(paths, cx);
                    });
                });
            }
        })
        .detach();
    }

    fn new_file(&mut self, cx: &mut Context<Self>) {
        let buffer = cx.new(|cx| EditorState::new(cx));
        cx.observe(&buffer, Self::on_buffer_changed).detach();
        self.add_buffer(buffer, cx);
        self.clamp_tab_scroll();
        self.update_search_editor(cx);
        cx.notify();
    }

    fn update_search_editor(&self, cx: &mut Context<Self>) {
        if let Some(buffer) = self.buffers.get(self.active_tab) {
            let buffer = buffer.clone();
            self.search_bar.update(cx, |bar, cx| {
                bar.set_editor(buffer, cx);
            });
        }
    }

    fn apply_prefill_to_search(
        &self,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let find_input = self.search_bar.read(cx).find_input_entity();
        let editor = self.search_bar.read(cx).editor_entity();
        find_input.update(cx, |state, cx| {
            state.set_value(SharedString::from(text.to_string()), window, cx);
        });
        if let Some(editor) = editor {
            editor.update(cx, |state, ecx| {
                state.find_all(text, ecx);
            });
        }
    }

    fn close_search_internal(&mut self, cx: &mut Context<Self>) {
        self.search_visible = false;
        self.goto_line_visible = false;
        if let Some(buffer) = self.buffers.get(self.active_tab) {
            let buffer = buffer.clone();
            buffer.update(cx, |state, ecx| state.clear_search(ecx));
        }
        cx.notify();
    }

    fn clamp_tab_scroll(&mut self) {
        let max = self.buffers.len().saturating_sub(1);
        if self.tab_scroll_offset > max {
            self.tab_scroll_offset = max;
        }
        if self.active_tab >= self.buffers.len() {
            return;
        }
        if self.active_tab < self.tab_scroll_offset {
            self.tab_scroll_offset = self.active_tab;
        }
    }

    fn close_tab_at(&mut self, idx: usize, cx: &mut Context<Self>) {
        if self.buffers.len() <= 1 {
            return;
        }
        self.autosave.cancel(idx);
        self.remove_buffer_at(idx);
        if self.active_tab >= self.buffers.len() {
            self.active_tab = self.buffers.len().saturating_sub(1);
        } else if self.active_tab > idx {
            self.active_tab -= 1;
        }
        self.clamp_tab_scroll();
        self.update_search_editor(cx);
        cx.notify();
    }

    fn render_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let offset = self.tab_scroll_offset;
        let total = self.buffers.len();
        let show_left = offset > 0;
        let show_right = total > 0 && offset < total.saturating_sub(1);
        let muted_fg = theme.tokens.muted_foreground;
        let active_fg = theme.tokens.foreground;
        let muted_bg = theme.tokens.muted;
        let border = theme.tokens.border;

        div()
            .flex_1()
            .h_full()
            .flex()
            .items_center()
            .overflow_x_hidden()
            .child(
                div()
                    .id("tab-scroll-left")
                    .h_full()
                    .w(px(28.0))
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .border_r_1()
                    .border_color(border.opacity(0.5))
                    .when(show_left, |el| {
                        el.cursor_pointer()
                            .hover(|s| s.bg(muted_bg.opacity(0.5)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.tab_scroll_offset =
                                    this.tab_scroll_offset.saturating_sub(1);
                                cx.notify();
                            }))
                            .child(
                                Icon::new("chevron-left")
                                    .size(px(14.0))
                                    .color(muted_fg),
                            )
                    })
                    .when(!show_left, |el| {
                        el.child(
                            Icon::new("chevron-left")
                                .size(px(14.0))
                                .color(muted_fg.opacity(0.2)),
                        )
                    }),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .overflow_x_hidden()
                    .children(
                        self.buffers
                            .iter()
                            .enumerate()
                            .skip(offset)
                            .map(|(idx, _)| {
                                let is_active = idx == self.active_tab;
                                let title = self
                                    .tab_meta
                                    .get(idx)
                                    .map(|meta| meta.title.clone())
                                    .unwrap_or_else(|| SharedString::from("Untitled"));
                                let can_close = self.buffers.len() > 1;
                                let active_bg = theme.tokens.background;

                                div()
                                    .id(ElementId::Name(format!("tab-{}", idx).into()))
                                    .h_full()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(14.0))
                                    .cursor_pointer()
                                    .text_size(px(13.0))
                                    .border_r_1()
                                    .border_color(border.opacity(0.5))
                                    .when(is_active, |el| {
                                        el.bg(active_bg).text_color(active_fg)
                                    })
                                    .when(!is_active, |el| {
                                        el.text_color(muted_fg)
                                            .hover(|s| s.bg(muted_bg.opacity(0.5)))
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.active_tab = idx;
                                        this.update_search_editor(cx);
                                        cx.notify();
                                    }))
                                    .child(title)
                                    .when(can_close, |el| {
                                        el.child(
                                            div()
                                                .id(ElementId::Name(
                                                    format!("tab-close-{}", idx).into(),
                                                ))
                                                .w(px(16.0))
                                                .h(px(16.0))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded(px(3.0))
                                                .text_color(muted_fg)
                                                .hover(|s| {
                                                    s.bg(muted_bg).text_color(active_fg)
                                                })
                                                .on_click(cx.listener(
                                                    move |this, _, _, cx| {
                                                        this.close_tab_at(idx, cx);
                                                    },
                                                ))
                                                .child(
                                                    Icon::new("x")
                                                        .size(px(12.0))
                                                        .color(muted_fg),
                                                ),
                                        )
                                    })
                            })
                    )
                    .child(
                        div()
                            .id("new-tab-btn")
                            .h_full()
                            .flex()
                            .flex_shrink_0()
                            .items_center()
                            .px(px(6.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(muted_bg.opacity(0.5)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.new_file(cx);
                            }))
                            .child(Icon::new("plus").size(px(14.0)).color(muted_fg)),
                    ),
            )
            .child(
                div()
                    .id("tab-scroll-right")
                    .h_full()
                    .w(px(28.0))
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .border_l_1()
                    .border_color(border.opacity(0.5))
                    .when(show_right, |el| {
                        el.cursor_pointer()
                            .hover(|s| s.bg(muted_bg.opacity(0.5)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                let max = this.buffers.len().saturating_sub(1);
                                if this.tab_scroll_offset < max {
                                    this.tab_scroll_offset += 1;
                                }
                                cx.notify();
                            }))
                            .child(
                                Icon::new("chevron-right")
                                    .size(px(14.0))
                                    .color(muted_fg),
                            )
                    })
                    .when(!show_right, |el| {
                        el.child(
                            Icon::new("chevron-right")
                                .size(px(14.0))
                                .color(muted_fg.opacity(0.2)),
                        )
                    }),
            )
    }

    fn render_theme_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let muted_fg = theme.tokens.muted_foreground;

        div()
            .id("theme-menu-anchor")
            .h_full()
            .flex()
            .flex_shrink_0()
            .items_center()
            .px(px(8.0))
            .border_l_1()
            .border_color(theme.tokens.border.opacity(0.5))
            .cursor_pointer()
            .hover(|s| s.bg(theme.tokens.muted.opacity(0.5)))
            .on_click(cx.listener(|this, _, _, cx| {
                this.theme_selector_open = !this.theme_selector_open;
                cx.notify();
            }))
            .child(Icon::new("sun").size(px(14.0)).color(muted_fg))
    }

    fn render_theme_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let all_themes = Theme::all();
        let current = theme.variant;

        div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap(px(6.0))
            .px(px(12.0))
            .py(px(8.0))
            .bg(theme.tokens.muted.opacity(0.3))
            .border_b_1()
            .border_color(theme.tokens.border)
            .children(all_themes.into_iter().enumerate().map(|(i, t)| {
                let variant = t.variant;
                let name = variant.display_name();
                let is_current = variant == current;
                let pill_bg = if is_current {
                    theme.tokens.primary
                } else {
                    theme.tokens.muted
                };
                let pill_fg = if is_current {
                    theme.tokens.primary_foreground
                } else {
                    theme.tokens.foreground
                };
                let chosen = t.clone();

                div()
                    .id(ElementId::Name(format!("theme-{}", i).into()))
                    .flex()
                    .items_center()
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(12.0))
                    .text_size(px(12.0))
                    .bg(pill_bg)
                    .text_color(pill_fg)
                    .cursor_pointer()
                    .when(!is_current, |el| {
                        el.hover(|s| s.bg(theme.tokens.muted.opacity(0.8)))
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        install_theme(cx, chosen.clone());
                        this.theme_selector_open = false;
                        cx.notify();
                    }))
                    .child(name)
            }))
    }

    fn render_goto_line(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let line_count = self
            .buffers
            .get(self.active_tab)
            .map(|b| b.read(cx).line_count())
            .unwrap_or(0);

        div()
            .w_full()
            .flex()
            .items_center()
            .bg(theme.tokens.muted.opacity(0.3))
            .border_b_1()
            .border_color(theme.tokens.border)
            .px(px(12.0))
            .py(px(6.0))
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Go to Line:"),
            )
            .child(
                div()
                    .w(px(100.0))
                    .child(
                        Input::new(&self.goto_line_input)
                            .placeholder("Line #")
                            .h(px(28.0))
                            .text_size(px(13.0))
                            .on_enter({
                                let goto_input = self.goto_line_input.clone();
                                let app_entity = cx.entity().clone();
                                move |_, cx| {
                                    let text = goto_input.read(cx).content().to_string();
                                    if let Ok(line) = text.trim().parse::<usize>() {
                                        let _ = app_entity.update(cx, |this, cx| {
                                            if let Some(buffer) =
                                                this.buffers.get(this.active_tab)
                                            {
                                                buffer.update(cx, |state, ecx| {
                                                    state.goto_line(line, ecx);
                                                });
                                            }
                                        });
                                    }
                                }
                            }),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child(format!("/ {}", line_count)),
            )
    }
}

impl Focusable for AppState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AppState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let status = self
            .buffers
            .get(self.active_tab)
            .map(|b| StatusBarView::from_editor(b.read(cx)));

        let search_visible = self.search_visible;
        let goto_visible = self.goto_line_visible;

        div()
            .key_context("ShioriApp")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &SaveFile, _, cx| {
                this.save_active(cx);
            }))
            .on_action(cx.listener(|this, _: &CloseTab, _, cx| {
                this.close_active_tab(cx);
            }))
            .on_action(cx.listener(|this, _: &OpenFile, _, cx| {
                this.open_file_dialog(cx);
            }))
            .on_action(cx.listener(|this, _: &NewFile, _, cx| {
                this.new_file(cx);
            }))
            .on_action(cx.listener(|this, _: &NextTab, _, cx| {
                if !this.buffers.is_empty() {
                    this.active_tab = (this.active_tab + 1) % this.buffers.len();
                    this.update_search_editor(cx);
                    cx.notify();
                }
            }))
            .on_action(cx.listener(|this, _: &PrevTab, _, cx| {
                if !this.buffers.is_empty() {
                    this.active_tab = if this.active_tab == 0 {
                        this.buffers.len() - 1
                    } else {
                        this.active_tab - 1
                    };
                    this.update_search_editor(cx);
                    cx.notify();
                }
            }))
            .on_action(cx.listener(|this, _: &ToggleSearch, window, cx| {
                this.goto_line_visible = false;
                this.search_visible = true;
                this.update_search_editor(cx);
                let prefill = this.search_bar.read(cx).get_prefill_text(cx);
                this.search_bar.update(cx, |bar, _cx| {
                    bar.show_replace = false;
                });
                if let Some(text) = prefill {
                    this.apply_prefill_to_search(&text, window, cx);
                }
                let fh = this.search_bar.read(cx).focus_handle(cx);
                window.focus(&fh);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleSearchReplace, window, cx| {
                this.goto_line_visible = false;
                this.search_visible = true;
                this.update_search_editor(cx);
                let prefill = this.search_bar.read(cx).get_prefill_text(cx);
                this.search_bar.update(cx, |bar, _cx| {
                    bar.show_replace = true;
                });
                if let Some(text) = prefill {
                    this.apply_prefill_to_search(&text, window, cx);
                }
                let fh = this.search_bar.read(cx).focus_handle(cx);
                window.focus(&fh);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &CloseSearch, _, cx| {
                if this.search_visible || this.goto_line_visible {
                    this.close_search_internal(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &GotoLine, window, cx| {
                this.search_visible = false;
                this.goto_line_visible = true;
                if let Some(buffer) = this.buffers.get(this.active_tab) {
                    let line_str = (buffer.read(cx).cursor().line + 1).to_string();
                    this.goto_line_input.update(cx, |state, cx| {
                        state.set_value(SharedString::from(line_str), window, cx);
                    });
                }
                let fh = this.goto_line_input.read(cx).focus_handle(cx);
                window.focus(&fh);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &CloseGotoLine, _, cx| {
                this.goto_line_visible = false;
                cx.notify();
            }))
            .on_drop::<ExternalPaths>(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                let file_paths: Vec<PathBuf> = paths
                    .paths()
                    .iter()
                    .filter(|p: &&PathBuf| p.is_file())
                    .cloned()
                    .collect();
                if !file_paths.is_empty() {
                    this.open_paths(file_paths, cx);
                }
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .child(
                div()
                    .w_full()
                    .h(px(36.0))
                    .flex()
                    .items_center()
                    .bg(theme.tokens.muted.opacity(0.3))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(self.render_tab_bar(cx))
                    .child(self.render_theme_button(cx)),
            )
            .when(self.theme_selector_open, |el| {
                el.child(self.render_theme_panel(cx))
            })
            .when(search_visible, |el| {
                el.child(self.search_bar.clone())
            })
            .when(goto_visible, |el| {
                el.child(self.render_goto_line(cx))
            })
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .children(self.buffers.get(self.active_tab).map(|buffer| {
                        Editor::new(buffer)
                            .show_line_numbers(true, cx)
                            .show_border(false)
                    })),
            )
            .children(status)
    }
}
