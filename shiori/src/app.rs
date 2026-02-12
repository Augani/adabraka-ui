use crate::autosave::AutosaveManager;
use crate::completion::{extract_symbols, CompletionItem, CompletionMenu, CompletionState};
use crate::git_state::GitState;
use crate::git_view::GitView;
use crate::search_bar::SearchBar;
use crate::status_bar::StatusBarView;
use crate::terminal_view::TerminalView;
use adabraka_ui::components::editor::{
    Editor, EditorState, Enter as EditorEnter, MoveDown, MoveUp, Tab as EditorTab,
};
use adabraka_ui::components::icon::Icon;
use adabraka_ui::components::input::{Input, InputState};
use adabraka_ui::components::resizable::{
    h_resizable, resizable_panel, v_resizable, ResizableState,
};
use adabraka_ui::navigation::file_tree::{FileNode, FileTree};
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
        OpenFolder,
        NewFile,
        NextTab,
        PrevTab,
        ToggleSearch,
        ToggleSearchReplace,
        CloseSearch,
        GotoLine,
        CloseGotoLine,
        ToggleSidebar,
        ToggleTerminal,
        ToggleTerminalFullscreen,
        NewTerminal,
        CompletionUp,
        CompletionDown,
        CompletionAccept,
        CompletionDismiss,
        TriggerCompletion,
        ToggleGitView,
        GitNextFile,
        GitPrevFile,
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
        KeyBinding::new("cmd-shift-o", OpenFolder, Some("ShioriApp")),
        KeyBinding::new("cmd-b", ToggleSidebar, Some("ShioriApp")),
        KeyBinding::new("cmd-`", ToggleTerminal, Some("ShioriApp")),
        KeyBinding::new("cmd-shift-enter", ToggleTerminalFullscreen, Some("ShioriApp")),
        KeyBinding::new("cmd-shift-g", ToggleGitView, Some("ShioriApp")),
        KeyBinding::new("ctrl-.", TriggerCompletion, Some("ShioriApp")),
        KeyBinding::new("up", CompletionUp, Some("ShioriApp")),
        KeyBinding::new("down", CompletionDown, Some("ShioriApp")),
        KeyBinding::new("ctrl-p", CompletionUp, Some("ShioriApp")),
        KeyBinding::new("ctrl-n", CompletionDown, Some("ShioriApp")),
        KeyBinding::new("tab", CompletionAccept, Some("ShioriApp")),
        KeyBinding::new("enter", CompletionAccept, Some("ShioriApp")),
        KeyBinding::new("escape", CompletionDismiss, Some("ShioriApp")),
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
    workspace_root: Option<PathBuf>,
    file_tree_nodes: Vec<FileNode>,
    expanded_paths: Vec<PathBuf>,
    selected_tree_path: Option<PathBuf>,
    sidebar_visible: bool,
    resizable_state: Entity<ResizableState>,
    terminals: Vec<Entity<TerminalView>>,
    active_terminal: usize,
    terminal_visible: bool,
    terminal_fullscreen: bool,
    terminal_resizable_state: Entity<ResizableState>,
    completion_state: Entity<CompletionState>,
    cached_symbols: Vec<CompletionItem>,
    last_symbol_update_line: usize,
    suppress_completion: bool,
    last_content_version: u64,
    git_state: Entity<GitState>,
    git_visible: bool,
    ide_theme_selector_open: bool,
}

struct TabMeta {
    file_path: Option<PathBuf>,
    file_name: Option<String>,
    modified: bool,
    title: SharedString,
    is_image: bool,
}

fn is_image_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()).as_deref(),
        Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" | "bmp" | "tiff" | "tif")
    )
}

fn scan_directory(path: &Path, depth: usize) -> Vec<FileNode> {
    let mut nodes = Vec::new();
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return nodes,
    };
    for entry in entries.flatten() {
        let entry_path = entry.path();
        let is_hidden = entry_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false);
        if entry_path.is_dir() {
            let mut dir_node = FileNode::directory(&entry_path).hidden(is_hidden);
            if depth > 0 {
                dir_node = dir_node.with_children(scan_directory(&entry_path, depth - 1));
            } else {
                dir_node = dir_node.with_unloaded_children(true);
            }
            nodes.push(dir_node);
        } else if entry_path.is_file() {
            nodes.push(FileNode::file(&entry_path).hidden(is_hidden));
        }
    }
    nodes
}

fn load_children_if_needed(nodes: &mut Vec<FileNode>, target: &Path) {
    for node in nodes.iter_mut() {
        if node.path == target {
            if node.has_unloaded_children && node.children.is_empty() {
                node.children = scan_directory(&node.path, 1);
                node.has_unloaded_children = false;
            }
            return;
        }
        if target.starts_with(&node.path) && !node.children.is_empty() {
            load_children_if_needed(&mut node.children, target);
            return;
        }
    }
}

impl AppState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let completion_state = cx.new(|cx| CompletionState::new(cx));

        let completion_for_check = completion_state.clone();
        let buffer = cx.new(|cx| {
            let mut state = EditorState::new(cx);
            state.set_overlay_active_check(move |cx| completion_for_check.read(cx).is_visible());
            state
        });
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

        let resizable_state = ResizableState::new(cx);
        let terminal_resizable_state = ResizableState::new(cx);
        let git_state = cx.new(|cx| GitState::new(cx));

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
            workspace_root: None,
            file_tree_nodes: Vec::new(),
            expanded_paths: Vec::new(),
            selected_tree_path: None,
            sidebar_visible: false,
            resizable_state,
            terminals: Vec::new(),
            active_terminal: 0,
            terminal_visible: false,
            terminal_fullscreen: false,
            terminal_resizable_state,
            completion_state,
            cached_symbols: Vec::new(),
            last_symbol_update_line: usize::MAX,
            suppress_completion: false,
            last_content_version: 0,
            git_state,
            git_visible: false,
            ide_theme_selector_open: false,
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
        let is_image = file_path.as_ref().map(|p| is_image_file(p)).unwrap_or(false);
        TabMeta {
            file_path,
            file_name,
            modified,
            title,
            is_image,
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

    fn add_buffer(&mut self, buffer: Entity<EditorState>, cx: &mut Context<Self>) {
        let idx = self.buffers.len();
        self.buffer_index.insert(buffer.entity_id(), idx);
        self.tab_meta.push(Self::build_tab_meta(&buffer, idx, cx));
        self.buffers.push(buffer.clone());
        self.autosave.push();
        self.active_tab = idx;
        self.setup_overlay_check(&buffer, cx);
    }

    fn setup_overlay_check(&self, buffer: &Entity<EditorState>, cx: &mut Context<Self>) {
        let completion_state = self.completion_state.clone();
        buffer.update(cx, |state, _| {
            state.set_overlay_active_check(move |cx| completion_state.read(cx).is_visible());
        });
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
            if is_image_file(&path) {
                self.open_image_tab(path, cx);
            } else {
                let buffer = cx.new(|cx| {
                    let mut state = EditorState::new(cx);
                    state.load_file(&path, cx);
                    state
                });
                cx.observe(&buffer, Self::on_buffer_changed).detach();
                self.add_buffer(buffer, cx);
            }
        }
        self.clamp_tab_scroll();
        self.update_search_editor(cx);
        cx.notify();
    }

    fn open_image_tab(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let idx = self.buffers.len();
        let buffer = cx.new(|cx| EditorState::new(cx));
        let file_name = path.file_name().map(|n| n.to_string_lossy().to_string());
        let title = Self::compose_tab_title(file_name.as_deref(), idx, false);
        self.buffer_index.insert(buffer.entity_id(), idx);
        self.tab_meta.push(TabMeta {
            file_path: Some(path),
            file_name,
            modified: false,
            title,
            is_image: true,
        });
        self.buffers.push(buffer);
        self.autosave.push();
        self.active_tab = idx;
    }

    fn on_buffer_changed(&mut self, buffer: Entity<EditorState>, cx: &mut Context<Self>) {
        if let Some(&idx) = self.buffer_index.get(&buffer.entity_id()) {
            if self.tab_meta.get(idx).map(|m| m.is_image).unwrap_or(false) {
                return;
            }
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

            if idx == self.active_tab {
                self.update_completion_for_typing(&buffer, cx);
            }
        }
        cx.notify();
    }

    fn update_completion_for_typing(&mut self, buffer: &Entity<EditorState>, cx: &mut Context<Self>) {
        if self.suppress_completion {
            self.suppress_completion = false;
            return;
        }

        let state = buffer.read(cx);
        let content_version = state.content_version();

        if content_version == self.last_content_version {
            return;
        }
        self.last_content_version = content_version;

        let completion_visible = self.completion_state.read(cx).is_visible();
        let cursor = state.cursor();
        let word_info = state.word_at_cursor();
        let anchor = state.cursor_screen_position(px(20.0));

        if completion_visible {
            let trigger_line = self.completion_state.read(cx).trigger_line();

            if let Some((word, _word_start)) = word_info {
                if cursor.line != trigger_line {
                    self.completion_state.update(cx, |s, cx| s.dismiss(cx));
                    return;
                }
                self.completion_state.update(cx, |s, cx| {
                    s.set_filter(&word, cx);
                });
                if let Some(anchor) = anchor {
                    self.completion_state.update(cx, |s, _| {
                        s.update_anchor(anchor);
                    });
                }
            } else {
                self.completion_state.update(cx, |s, cx| s.dismiss(cx));
            }
        } else if let Some((word, word_start)) = word_info {
            let state = buffer.read(cx);
            let tree_exists = state.syntax_tree().is_some();

            if word.len() >= 2 && tree_exists {
                if self.last_symbol_update_line != cursor.line {
                    if let Some(tree) = state.syntax_tree() {
                        let content = state.content();
                        let language = state.language();
                        let symbols = extract_symbols(tree, &content, language);
                        self.cached_symbols =
                            symbols.into_iter().map(CompletionItem::from).collect();
                        self.last_symbol_update_line = cursor.line;
                    }
                }

                if !self.cached_symbols.is_empty() {
                    if let Some(anchor) = anchor {
                        let items = self.cached_symbols.clone();
                        self.completion_state.update(cx, |s, cx| {
                            s.show(items, cursor.line, word_start, anchor, cx);
                            s.set_filter(&word, cx);
                        });
                    }
                }
            }
        }
    }

    fn trigger_completion(&mut self, cx: &mut Context<Self>) {
        let buffer = match self.buffers.get(self.active_tab) {
            Some(b) => b.clone(),
            None => return,
        };

        let state = buffer.read(cx);
        let cursor = state.cursor();
        let language = state.language();
        let content = state.content();

        if self.last_symbol_update_line != cursor.line {
            if let Some(tree) = state.syntax_tree() {
                let symbols = extract_symbols(tree, &content, language);
                self.cached_symbols = symbols.into_iter().map(CompletionItem::from).collect();
                self.last_symbol_update_line = cursor.line;
            }
        }

        if self.cached_symbols.is_empty() {
            return;
        }

        let anchor = match state.cursor_screen_position(px(20.0)) {
            Some(p) => p,
            None => return,
        };

        let (filter_prefix, trigger_col) = if let Some((word, word_start)) = state.word_at_cursor() {
            (word, word_start)
        } else {
            (String::new(), cursor.col)
        };

        let items: Vec<CompletionItem> = self.cached_symbols.clone();

        self.completion_state.update(cx, |s, cx| {
            s.show(items, cursor.line, trigger_col, anchor, cx);
            if !filter_prefix.is_empty() {
                s.set_filter(&filter_prefix, cx);
            }
        });
    }

    fn apply_completion(&mut self, cx: &mut Context<Self>) {
        let item = match self.completion_state.read(cx).selected_item() {
            Some(i) => i.clone(),
            None => return,
        };

        let trigger_col = self.completion_state.read(cx).trigger_col();

        self.suppress_completion = true;

        if let Some(buffer) = self.buffers.get(self.active_tab).cloned() {
            buffer.update(cx, |state, ecx| {
                state.apply_completion(trigger_col, &item.insert_text, ecx);
            });
        }

        self.completion_state.update(cx, |s, cx| s.dismiss(cx));
    }

    fn completion_move_up(&mut self, cx: &mut Context<Self>) {
        self.completion_state.update(cx, |s, cx| s.move_up(cx));
    }

    fn completion_move_down(&mut self, cx: &mut Context<Self>) {
        self.completion_state.update(cx, |s, cx| s.move_down(cx));
    }

    fn completion_dismiss(&mut self, cx: &mut Context<Self>) {
        self.completion_state.update(cx, |s, cx| s.dismiss(cx));
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

    fn render_ide_theme_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let muted_fg = theme.tokens.muted_foreground;

        div()
            .id("ide-theme-anchor")
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
                this.ide_theme_selector_open = !this.ide_theme_selector_open;
                cx.notify();
            }))
            .child(Icon::new("palette").size(px(14.0)).color(muted_fg))
    }

    fn render_ide_theme_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let all = crate::ide_theme::all_ide_themes();
        let current_name = crate::ide_theme::use_ide_theme().name;

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
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(theme.tokens.muted_foreground)
                    .mr(px(4.0))
                    .child("IDE:"),
            )
            .children(all.into_iter().enumerate().map(|(i, t)| {
                let name = t.name;
                let is_current = name == current_name;
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

                div()
                    .id(ElementId::Name(format!("ide-theme-{}", i).into()))
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
                        let themes = crate::ide_theme::all_ide_themes();
                        if let Some(chosen) = themes.into_iter().nth(i) {
                            crate::ide_theme::install_ide_theme(chosen);
                        }
                        for terminal in &this.terminals {
                            terminal.update(cx, |tv, _cx| {
                                tv.apply_ide_theme();
                            });
                        }
                        this.ide_theme_selector_open = false;
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

    pub fn open_folder(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let nodes = scan_directory(&path, 2);
        self.expanded_paths = vec![path.clone()];
        let git_path = path.clone();
        self.workspace_root = Some(path);
        self.file_tree_nodes = nodes;
        self.sidebar_visible = true;
        self.selected_tree_path = None;
        self.git_state
            .update(cx, |s, cx| s.set_workspace(git_path, cx));
        cx.notify();
    }

    fn open_folder_dialog(&mut self, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: None,
        });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await {
                if let Some(path) = paths.into_iter().next() {
                    let _ = cx.update(|cx| {
                        let _ = this.update(cx, |this, cx| {
                            this.open_folder(path, cx);
                        });
                    });
                }
            }
        })
        .detach();
    }

    fn toggle_terminal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.terminal_visible {
            self.terminal_visible = false;
        } else {
            if self.terminals.is_empty() {
                self.new_terminal(window, cx);
                return;
            }
            self.terminal_visible = true;
        }
        cx.notify();
    }

    fn new_terminal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let working_dir = self.current_working_directory();
        let terminal =
            cx.new(|cx| TerminalView::new(cx).with_working_directory(working_dir));
        terminal.update(cx, |t, cx| {
            let _ = t.start_with_polling(window, cx);
        });
        self.terminals.push(terminal);
        self.active_terminal = self.terminals.len() - 1;
        self.terminal_visible = true;
        cx.notify();
    }

    fn close_terminal_at(&mut self, idx: usize, cx: &mut Context<Self>) {
        if idx >= self.terminals.len() {
            return;
        }
        self.terminals[idx].update(cx, |t, _| t.stop());
        self.terminals.remove(idx);
        if self.terminals.is_empty() {
            self.terminal_visible = false;
            self.terminal_fullscreen = false;
            self.active_terminal = 0;
        } else if self.active_terminal >= self.terminals.len() {
            self.active_terminal = self.terminals.len() - 1;
        }
        cx.notify();
    }

    fn toggle_terminal_fullscreen(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.terminals.is_empty() {
            self.new_terminal(window, cx);
        }
        self.terminal_fullscreen = !self.terminal_fullscreen;
        if self.terminal_fullscreen {
            self.terminal_visible = true;
        }
        cx.notify();
    }

    fn render_terminal_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let border = theme.tokens.border;
        let muted_fg = theme.tokens.muted_foreground;
        let muted_bg = theme.tokens.muted;
        let active_fg = theme.tokens.foreground;
        let active_bg = theme.tokens.background;

        let any_running = self
            .terminals
            .iter()
            .any(|t| t.read(cx).is_running());

        div()
            .id("terminal-tab-bar")
            .w_full()
            .h(px(36.0))
            .bg(theme.tokens.muted.opacity(0.3))
            .border_b_1()
            .border_color(border)
            .flex()
            .items_center()
            .child(
                div()
                    .h_full()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .px(px(12.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(muted_fg)
                            .child("TERMINAL"),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .items_center()
                    .overflow_x_hidden()
                    .children(self.terminals.iter().enumerate().map(|(idx, term)| {
                        let is_active = idx == self.active_terminal;
                        let title = term.read(cx).title();

                        div()
                            .id(ElementId::Name(format!("term-tab-{}", idx).into()))
                            .h_full()
                            .flex()
                            .flex_shrink_0()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(12.0))
                            .cursor_pointer()
                            .text_size(px(13.0))
                            .border_r_1()
                            .border_color(border.opacity(0.5))
                            .when(is_active, |el| el.bg(active_bg).text_color(active_fg))
                            .when(!is_active, |el| {
                                el.text_color(muted_fg)
                                    .hover(|s| s.bg(muted_bg.opacity(0.5)))
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active_terminal = idx;
                                cx.notify();
                            }))
                            .child(title)
                            .child(
                                div()
                                    .id(ElementId::Name(
                                        format!("term-tab-close-{}", idx).into(),
                                    ))
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(3.0))
                                    .text_color(muted_fg)
                                    .hover(|s| s.bg(muted_bg).text_color(active_fg))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.close_terminal_at(idx, cx);
                                    }))
                                    .child(
                                        Icon::new("x").size(px(12.0)).color(muted_fg),
                                    ),
                            )
                    }))
                    .child(
                        div()
                            .id("new-terminal-btn")
                            .h_full()
                            .flex()
                            .flex_shrink_0()
                            .items_center()
                            .px(px(6.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(muted_bg.opacity(0.5)))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.new_terminal(window, cx);
                            }))
                            .child(Icon::new("plus").size(px(14.0)).color(muted_fg)),
                    ),
            )
            .child(
                div()
                    .id("terminal-fullscreen-btn")
                    .flex_shrink_0()
                    .h_full()
                    .flex()
                    .items_center()
                    .px(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(muted_bg.opacity(0.5)))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_terminal_fullscreen(window, cx);
                    }))
                    .child(
                        Icon::new(if self.terminal_fullscreen {
                            "minimize-2"
                        } else {
                            "maximize-2"
                        })
                        .size(px(14.0))
                        .color(muted_fg),
                    ),
            )
            .child(
                div()
                    .flex_shrink_0()
                    .px(px(10.0))
                    .child(
                        div().w(px(8.0)).h(px(8.0)).rounded_full().bg(
                            if any_running {
                                gpui::rgb(0x4ade80)
                            } else {
                                gpui::rgb(0x8b7b6b)
                            },
                        ),
                    ),
            )
    }

    fn current_working_directory(&self) -> PathBuf {
        if let Some(meta) = self.tab_meta.get(self.active_tab) {
            if let Some(path) = &meta.file_path {
                if let Some(parent) = path.parent() {
                    return parent.to_path_buf();
                }
            }
        }
        if let Some(root) = &self.workspace_root {
            return root.clone();
        }
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
    }

    fn render_image_preview(path: &Path, theme: &Theme) -> Div {
        let path_str: SharedString = path.to_string_lossy().into_owned().into();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_size = std::fs::metadata(path)
            .map(|m| {
                let bytes = m.len();
                if bytes < 1024 {
                    format!("{} B", bytes)
                } else if bytes < 1024 * 1024 {
                    format!("{:.1} KB", bytes as f64 / 1024.0)
                } else {
                    format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
                }
            })
            .unwrap_or_default();

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.tokens.background)
            .child(
                div()
                    .max_w(px(800.0))
                    .max_h_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .p(px(24.0))
                    .child(
                        img(path_str)
                            .max_w(px(760.0))
                            .max_h(px(600.0))
                            .object_fit(ObjectFit::Contain),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.foreground)
                                    .child(file_name),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(file_size),
                            ),
                    ),
            )
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let folder_name = self
            .workspace_root
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Workspace".to_string());

        let app_entity = cx.entity().clone();
        let app_entity2 = cx.entity().clone();

        let mut tree = FileTree::new()
            .nodes(self.file_tree_nodes.clone())
            .expanded_paths(self.expanded_paths.clone());
        if let Some(path) = &self.selected_tree_path {
            tree = tree.selected_path(path.clone());
        }
        tree = tree
            .on_select({
                move |path, _, cx| {
                    let path = path.clone();
                    let _ = app_entity.update(cx, |this, cx| {
                        this.selected_tree_path = Some(path.clone());
                        if path.is_file() {
                            let already_open = this
                                .tab_meta
                                .iter()
                                .position(|meta| meta.file_path.as_ref() == Some(&path));
                            if let Some(idx) = already_open {
                                this.active_tab = idx;
                                this.update_search_editor(cx);
                            } else {
                                this.open_paths(vec![path], cx);
                            }
                        }
                        cx.notify();
                    });
                }
            })
            .on_toggle({
                move |path, expanding, _, cx| {
                    let path = path.clone();
                    let _ = app_entity2.update(cx, |this, cx| {
                        if expanding {
                            if !this.expanded_paths.contains(&path) {
                                this.expanded_paths.push(path.clone());
                            }
                            load_children_if_needed(&mut this.file_tree_nodes, &path);
                        } else {
                            this.expanded_paths.retain(|p| p != &path);
                        }
                        cx.notify();
                    });
                }
            });

        div()
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
                    .justify_between()
                    .px(px(12.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .overflow_x_hidden()
                            .text_ellipsis()
                            .child(folder_name),
                    )
                    .child(
                        div()
                            .id("toggle-sidebar-close")
                            .w(px(22.0))
                            .h(px(22.0))
                            .flex()
                            .flex_shrink_0()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.tokens.muted.opacity(0.5)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.sidebar_visible = false;
                                cx.notify();
                            }))
                            .child(
                                Icon::new("chevron-left")
                                    .size(px(14.0))
                                    .color(theme.tokens.muted_foreground),
                            ),
                    ),
            )
            .child(div().id("sidebar-tree").flex_1().overflow_y_scroll().child(tree))
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

        let terminal_visible = self.terminal_visible;
        let app_entity = cx.entity().clone();
        let app_entity_git = cx.entity().clone();
        let git_summary = {
            let gs = self.git_state.read(cx);
            if gs.repo_path.is_some() && gs.summary.changed_files > 0 {
                Some(gs.summary.clone())
            } else if gs.repo_path.is_some() {
                Some(gs.summary.clone())
            } else {
                None
            }
        };
        let status = self
            .buffers
            .get(self.active_tab)
            .map(|b| {
                StatusBarView::from_editor(b.read(cx))
                    .terminal_open(terminal_visible)
                    .on_toggle_terminal(move |window, cx| {
                        let _ = app_entity.update(cx, |this, cx| {
                            this.toggle_terminal(window, cx);
                        });
                    })
                    .git_summary(git_summary)
                    .on_toggle_git(move |_window, cx| {
                        let _ = app_entity_git.update(cx, |this, cx| {
                            this.git_visible = !this.git_visible;
                            cx.notify();
                        });
                    })
            });

        let search_visible = self.search_visible;
        let goto_visible = self.goto_line_visible;

        let ide = crate::ide_theme::use_ide_theme();
        let build_editor = |buffer: &Entity<EditorState>, cx: &mut App| {
            let syn = ide.syntax.clone();
            Editor::new(buffer)
                .show_line_numbers(true, cx)
                .show_border(false)
                .cursor_color(ide.editor.cursor)
                .selection_color(ide.editor.selection)
                .search_match_colors(ide.editor.search_match, ide.editor.search_match_active)
                .syntax_color_fn(move |name| syn.color_for_capture(name))
        };

        let active_is_image = self.tab_meta.get(self.active_tab).map(|m| m.is_image).unwrap_or(false);
        let active_image_path = if active_is_image {
            self.tab_meta.get(self.active_tab).and_then(|m| m.file_path.clone())
        } else {
            None
        };

        let editor_area = if self.sidebar_visible {
            let sidebar = self.render_sidebar(cx);
            let editor_el: Option<AnyElement> = if let Some(image_path) = &active_image_path {
                Some(Self::render_image_preview(image_path, &theme).into_any_element())
            } else {
                self.buffers.get(self.active_tab).map(|buffer| {
                    build_editor(buffer, cx).into_any_element()
                })
            };
            div()
                .size_full()
                .child(
                    h_resizable("main-layout", self.resizable_state.clone())
                        .child(
                            resizable_panel()
                                .size(px(250.0))
                                .min_size(px(150.0))
                                .max_size(px(500.0))
                                .child(sidebar),
                        )
                        .child(
                            resizable_panel().child(
                                div()
                                    .size_full()
                                    .children(editor_el),
                            ),
                        ),
                )
                .into_any_element()
        } else {
            let editor_el: Option<AnyElement> = if let Some(image_path) = &active_image_path {
                Some(Self::render_image_preview(image_path, &theme).into_any_element())
            } else {
                self.buffers.get(self.active_tab).map(|buffer| {
                    build_editor(buffer, cx).into_any_element()
                })
            };
            div()
                .size_full()
                .children(editor_el)
                .into_any_element()
        };

        let main_content = if self.git_visible {
            div()
                .flex_1()
                .overflow_hidden()
                .child(GitView::new(self.git_state.clone()))
                .into_any_element()
        } else if self.terminal_fullscreen {
            let terminal_tab_bar = self.render_terminal_tab_bar(cx);
            let active_terminal = self.terminals.get(self.active_terminal).cloned();
            div()
                .flex_1()
                .overflow_hidden()
                .flex()
                .flex_col()
                .child(terminal_tab_bar)
                .child(
                    div()
                        .flex_1()
                        .overflow_hidden()
                        .children(active_terminal),
                )
                .into_any_element()
        } else if terminal_visible {
            let terminal_tab_bar = self.render_terminal_tab_bar(cx);
            let active_terminal = self.terminals.get(self.active_terminal).cloned();
            div()
                .flex_1()
                .overflow_hidden()
                .child(
                    v_resizable(
                        "editor-terminal",
                        self.terminal_resizable_state.clone(),
                    )
                    .child(resizable_panel().child(editor_area))
                    .child(
                        resizable_panel()
                            .size(px(300.0))
                            .min_size(px(100.0))
                            .max_size(px(600.0))
                            .child(
                                div()
                                    .size_full()
                                    .flex()
                                    .flex_col()
                                    .child(terminal_tab_bar)
                                    .child(
                                        div()
                                            .flex_1()
                                            .overflow_hidden()
                                            .children(active_terminal),
                                    ),
                            ),
                    ),
                )
                .into_any_element()
        } else {
            div()
                .flex_1()
                .overflow_hidden()
                .child(editor_area)
                .into_any_element()
        };

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
            .on_action(cx.listener(|this, _: &OpenFolder, _, cx| {
                this.open_folder_dialog(cx);
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
            .on_action(cx.listener(|this, _: &ToggleSidebar, _, cx| {
                if this.workspace_root.is_some() {
                    this.sidebar_visible = !this.sidebar_visible;
                    cx.notify();
                }
            }))
            .on_action(cx.listener(|this, _: &ToggleTerminal, window, cx| {
                this.toggle_terminal(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleTerminalFullscreen, window, cx| {
                this.toggle_terminal_fullscreen(window, cx);
            }))
            .on_action(cx.listener(|this, _: &NewTerminal, window, cx| {
                this.new_terminal(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleGitView, _, cx| {
                this.git_visible = !this.git_visible;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &GitNextFile, _, cx| {
                if this.git_visible {
                    this.git_state.update(cx, |s, cx| s.select_next_file(cx));
                }
            }))
            .on_action(cx.listener(|this, _: &GitPrevFile, _, cx| {
                if this.git_visible {
                    this.git_state.update(cx, |s, cx| s.select_prev_file(cx));
                }
            }))
            .on_action(cx.listener(|this, _: &TriggerCompletion, _, cx| {
                this.trigger_completion(cx);
            }))
            .on_action(cx.listener(|this, _: &CompletionUp, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.completion_move_up(cx);
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &CompletionDown, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.completion_move_down(cx);
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &CompletionAccept, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.apply_completion(cx);
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &CompletionDismiss, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.completion_dismiss(cx);
                } else if this.git_visible {
                    this.git_visible = false;
                    cx.notify();
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &MoveUp, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.completion_move_up(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &MoveDown, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.completion_move_down(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &EditorTab, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.apply_completion(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &EditorEnter, _, cx| {
                if this.completion_state.read(cx).is_visible() {
                    this.apply_completion(cx);
                }
            }))
            .on_drop::<ExternalPaths>(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                let mut file_paths = Vec::new();
                let mut folder_path = None;
                for p in paths.paths() {
                    if p.is_dir() {
                        folder_path = Some(p.clone());
                    } else if p.is_file() {
                        file_paths.push(p.clone());
                    }
                }
                if let Some(folder) = folder_path {
                    this.open_folder(folder, cx);
                }
                if !file_paths.is_empty() {
                    this.open_paths(file_paths, cx);
                }
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .when(!self.terminal_fullscreen, |el| {
                el.child(
                    div()
                        .w_full()
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .bg(theme.tokens.muted.opacity(0.3))
                        .border_b_1()
                        .border_color(theme.tokens.border)
                        .child(self.render_tab_bar(cx))
                        .child(self.render_ide_theme_button(cx))
                        .child(self.render_theme_button(cx)),
                )
            })
            .when(self.theme_selector_open && !self.terminal_fullscreen, |el| {
                el.child(self.render_theme_panel(cx))
            })
            .when(self.ide_theme_selector_open && !self.terminal_fullscreen, |el| {
                el.child(self.render_ide_theme_panel(cx))
            })
            .when(search_visible && !self.terminal_fullscreen, |el| {
                el.child(self.search_bar.clone())
            })
            .when(goto_visible && !self.terminal_fullscreen, |el| {
                el.child(self.render_goto_line(cx))
            })
            .child(main_content)
            .when(!self.terminal_fullscreen, |el| {
                el.children(status)
            })
            .child({
                let app_entity = cx.entity().clone();
                let mut menu = CompletionMenu::new(self.completion_state.clone());
                if let Some(buffer) = self.buffers.get(self.active_tab) {
                    menu = menu.editor_state(buffer.clone());
                }
                menu.on_accept(move |_, cx| {
                    let _ = app_entity.update(cx, |this, cx| {
                        this.apply_completion(cx);
                    });
                })
            })
    }
}
