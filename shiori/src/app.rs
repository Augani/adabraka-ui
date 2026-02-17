use crate::autosave::AutosaveManager;
use crate::completion::{extract_symbols, CompletionItem, CompletionMenu, CompletionState};
use crate::git_service::FileStatusKind;
use crate::git_state::GitState;
use crate::git_view::GitView;
use crate::ide_theme::{all_ide_themes, install_ide_theme, sync_adabraka_theme_from_ide, use_ide_theme, IdeTheme};
use crate::search_bar::SearchBar;
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
use adabraka_ui::overlays::command_palette::{
    CloseCommand, Command, CommandPalette, NavigateDown as CmdNavDown, NavigateUp as CmdNavUp,
    SelectCommand,
};
use gpui::prelude::FluentBuilder as _;
use gpui::EntityId;
use gpui::*;
use smol::Timer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
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
        ToggleSymbolOutline,
        ToggleCommandPalette,
        FoldToggle,
        FoldAll,
        UnfoldAll,
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Explorer,
    Git,
    Terminal,
    Settings,
}

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
        KeyBinding::new("cmd-shift-k", OpenFolder, Some("ShioriApp")),
        KeyBinding::new("cmd-b", ToggleSidebar, Some("ShioriApp")),
        KeyBinding::new("cmd-`", ToggleTerminal, Some("ShioriApp")),
        KeyBinding::new(
            "cmd-shift-enter",
            ToggleTerminalFullscreen,
            Some("ShioriApp"),
        ),
        KeyBinding::new("cmd-shift-g", ToggleGitView, Some("ShioriApp")),
        KeyBinding::new("cmd-shift-p", ToggleCommandPalette, Some("ShioriApp")),
        KeyBinding::new("cmd-shift-o", ToggleSymbolOutline, Some("ShioriApp")),
        KeyBinding::new("cmd-shift-[", FoldToggle, Some("ShioriApp")),
        KeyBinding::new("cmd-k cmd-0", FoldAll, Some("ShioriApp")),
        KeyBinding::new("cmd-k cmd-j", UnfoldAll, Some("ShioriApp")),
        KeyBinding::new("ctrl-.", TriggerCompletion, Some("ShioriApp")),
        KeyBinding::new("up", CompletionUp, Some("ShioriApp")),
        KeyBinding::new("down", CompletionDown, Some("ShioriApp")),
        KeyBinding::new("ctrl-p", CompletionUp, Some("ShioriApp")),
        KeyBinding::new("ctrl-n", CompletionDown, Some("ShioriApp")),
        KeyBinding::new("tab", CompletionAccept, Some("ShioriApp")),
        KeyBinding::new("enter", CompletionAccept, Some("ShioriApp")),
        KeyBinding::new("escape", CompletionDismiss, Some("ShioriApp")),
        KeyBinding::new("up", CmdNavUp, Some("CommandPalette")),
        KeyBinding::new("down", CmdNavDown, Some("CommandPalette")),
        KeyBinding::new("enter", SelectCommand, Some("CommandPalette")),
        KeyBinding::new("escape", CloseCommand, Some("CommandPalette")),
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
    active_mode: ViewMode,
    panel_visible: bool,
    workspace_root: Option<PathBuf>,
    file_tree_nodes: Vec<FileNode>,
    expanded_paths: Vec<PathBuf>,
    selected_tree_path: Option<PathBuf>,
    terminals: Vec<Entity<TerminalView>>,
    active_terminal: usize,
    terminal_visible: bool,
    terminal_fullscreen: bool,
    terminal_resizable_state: Entity<ResizableState>,
    sidebar_resizable_state: Entity<ResizableState>,
    completion_state: Entity<CompletionState>,
    cached_symbols: Vec<CompletionItem>,
    last_symbol_update_line: usize,
    suppress_completion: bool,
    last_content_version: u64,
    git_state: Entity<GitState>,
    symbol_outline_visible: bool,
    symbol_outline_filter: String,
    command_palette: Option<Entity<CommandPalette>>,
    command_palette_open: bool,
    file_search_input: Entity<InputState>,
    file_search_query: String,
    file_search_results: Vec<ContentSearchResult>,
    file_index: Arc<Vec<(PathBuf, String, String)>>,
    search_version: u64,
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
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .as_deref(),
        Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" | "bmp" | "tiff" | "tif")
    )
}

#[derive(Clone, Debug)]
struct ContentSearchResult {
    path: PathBuf,
    file_name: String,
    dir_path: String,
    line_number: usize,
    line_content: String,
    col_start: usize,
    col_end: usize,
}

fn is_binary_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()).as_deref(),
        Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" | "bmp" | "tiff" | "tif"
            | "mp3" | "wav" | "ogg" | "flac" | "mp4" | "mov" | "avi" | "webm"
            | "zip" | "tar" | "gz" | "rar" | "7z" | "woff" | "woff2" | "ttf" | "otf"
            | "exe" | "dll" | "so" | "dylib" | "o" | "a" | "rlib" | "rmeta" | "d"
            | "lock" | "bin" | "dat" | "db" | "sqlite")
    )
}

fn search_content(
    query: &str,
    file_index: &[(PathBuf, String, String)],
) -> Vec<ContentSearchResult> {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    if query.is_empty() || query.len() < 2 {
        return Vec::new();
    }

    let max_results = 100;
    let max_file_size: u64 = 2 * 1024 * 1024; // Skip files > 2MB
    let query_lower = query.to_lowercase();
    let query_bytes = query_lower.as_bytes();

    // Parallel search: split file list into chunks and search concurrently
    let result_count = AtomicUsize::new(0);
    let done = AtomicBool::new(false);

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get().min(8))
        .unwrap_or(4);
    let chunk_size = (file_index.len() / num_threads).max(1);

    let results: Vec<Vec<ContentSearchResult>> = std::thread::scope(|s| {
        let handles: Vec<_> = file_index
            .chunks(chunk_size)
            .map(|chunk| {
                let query_lower = &query_lower;
                let query_bytes = query_bytes;
                let result_count = &result_count;
                let done = &done;
                s.spawn(move || {
                    let mut local_results = Vec::new();
                    for (path, file_name, dir_path) in chunk {
                        if done.load(Ordering::Relaxed) {
                            break;
                        }
                        if is_binary_file(path) {
                            continue;
                        }
                        // Skip large files by checking metadata (cheap syscall)
                        if let Ok(meta) = std::fs::metadata(path) {
                            if meta.len() > max_file_size {
                                continue;
                            }
                        }
                        // Read file and search — streaming line by line
                        let content = match std::fs::read_to_string(path) {
                            Ok(c) => c,
                            Err(_) => continue,
                        };
                        // Fast byte-level pre-check: does the file contain the query at all?
                        let content_lower = content.to_lowercase();
                        if !memchr_find(content_lower.as_bytes(), query_bytes) {
                            continue;
                        }
                        for (line_idx, line) in content.lines().enumerate() {
                            if result_count.load(Ordering::Relaxed) >= max_results {
                                done.store(true, Ordering::Relaxed);
                                break;
                            }
                            let line_lower = line.to_lowercase();
                            if let Some(col) = line_lower.find(query_lower) {
                                let trimmed = line.trim();
                                if trimmed.is_empty() {
                                    continue;
                                }
                                let display_line = if trimmed.len() > 120 {
                                    format!("{}...", &trimmed[..120])
                                } else {
                                    trimmed.to_string()
                                };
                                let trim_offset = line.find(trimmed).unwrap_or(0);
                                let adjusted_col = col.saturating_sub(trim_offset);

                                local_results.push(ContentSearchResult {
                                    path: path.clone(),
                                    file_name: file_name.clone(),
                                    dir_path: dir_path.clone(),
                                    line_number: line_idx + 1,
                                    line_content: display_line,
                                    col_start: adjusted_col,
                                    col_end: adjusted_col + query.len(),
                                });
                                result_count.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                    local_results
                })
            })
            .collect();

        handles.into_iter().map(|h| h.join().unwrap_or_default()).collect()
    });

    let mut merged: Vec<ContentSearchResult> = results.into_iter().flatten().collect();
    merged.truncate(max_results);
    merged
}

/// Fast substring check using first-byte scanning
fn memchr_find(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    let first = needle[0];
    let mut i = 0;
    while i <= haystack.len() - needle.len() {
        if haystack[i] == first && &haystack[i..i + needle.len()] == needle {
            return true;
        }
        i += 1;
    }
    false
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

        let goto_line_input = cx.new(|cx| InputState::new(cx));
        let file_search_input = cx.new(|cx| InputState::new(cx));

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

        let buffer_index = HashMap::new();
        let tab_meta = Vec::new();

        let terminal_resizable_state = ResizableState::new(cx);
        let sidebar_resizable_state = ResizableState::new(cx);
        let git_state = cx.new(|cx| GitState::new(cx));

        Self {
            focus_handle,
            buffers: Vec::new(),
            buffer_index,
            active_tab: 0,
            autosave: AutosaveManager::new(1),
            tab_meta,
            search_bar,
            search_visible: false,
            goto_line_visible: false,
            goto_line_input,
            tab_scroll_offset: 0,
            active_mode: ViewMode::Explorer,
            panel_visible: false,
            workspace_root: None,
            file_tree_nodes: Vec::new(),
            expanded_paths: Vec::new(),
            selected_tree_path: None,
            terminals: Vec::new(),
            active_terminal: 0,
            terminal_visible: false,
            terminal_fullscreen: false,
            terminal_resizable_state,
            sidebar_resizable_state,
            completion_state,
            cached_symbols: Vec::new(),
            last_symbol_update_line: usize::MAX,
            suppress_completion: false,
            last_content_version: 0,
            git_state,
            symbol_outline_visible: false,
            symbol_outline_filter: String::new(),
            command_palette: None,
            command_palette_open: false,
            file_search_input,
            file_search_query: String::new(),
            file_search_results: Vec::new(),
            file_index: Arc::new(Vec::new()),
            search_version: 0,
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
        let is_image = file_path
            .as_ref()
            .map(|p| is_image_file(p))
            .unwrap_or(false);
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
                let completion_check = self.completion_state.clone();
                let buffer = cx.new(|cx| {
                    let mut state = EditorState::new(cx);
                    state
                        .set_overlay_active_check(move |cx| completion_check.read(cx).is_visible());
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

    
  
        fn update_completion_for_typing(
        &mut self,
        buffer: &Entity<EditorState>,
        cx: &mut Context<Self>,
    ) {
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

        let (filter_prefix, trigger_col) = if let Some((word, word_start)) = state.word_at_cursor()
        {
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
                let saved_path = buffer.read(cx).file_path().cloned();
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
        if self.buffers.is_empty() {
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
        let completion_check = self.completion_state.clone();
        let buffer = cx.new(|cx| {
            let mut state = EditorState::new(cx);
            state.set_overlay_active_check(move |cx| completion_check.read(cx).is_visible());
            state
        });
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

    fn apply_prefill_to_search(&self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
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
        if self.buffers.is_empty() {
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
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let offset = self.tab_scroll_offset;
        let total = self.buffers.len();
        let show_left = offset > 0;
        let show_right = total > 0 && offset < total.saturating_sub(1);
        let muted_fg = chrome.text_secondary;
        let active_fg = chrome.bright;
        let editor_bg = chrome.editor_bg;
        let border_color = hsla(0.0, 0.0, 1.0, 0.05);

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
                    .border_color(border_color)
                    .when(show_left, |el| {
                        el.cursor_pointer()
                            .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.tab_scroll_offset = this.tab_scroll_offset.saturating_sub(1);
                                cx.notify();
                            }))
                            .child(Icon::new("chevron-left").size(px(14.0)).color(muted_fg))
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
                                    .border_color(border_color)
                                    .when(is_active, |el| {
                                        el.bg(editor_bg).text_color(active_fg)
                                    })
                                    .when(!is_active, |el| {
                                        el.text_color(muted_fg)
                                            .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.active_tab = idx;
                                        this.update_search_editor(cx);
                                        cx.notify();
                                    }))
                                    .child(title)
                                    .child(
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
                                                s.bg(hsla(0.0, 0.0, 1.0, 0.1))
                                                    .text_color(active_fg)
                                            })
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.close_tab_at(idx, cx);
                                            }))
                                            .child(
                                                Icon::new("x").size(px(12.0)).color(muted_fg),
                                            ),
                                    )
                            }),
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
                            .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
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
                    .border_color(border_color)
                    .when(show_right, |el| {
                        el.cursor_pointer()
                            .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                let max = this.buffers.len().saturating_sub(1);
                                if this.tab_scroll_offset < max {
                                    this.tab_scroll_offset += 1;
                                }
                                cx.notify();
                            }))
                            .child(Icon::new("chevron-right").size(px(14.0)).color(muted_fg))
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

    fn render_goto_line(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let line_count = self
            .buffers
            .get(self.active_tab)
            .map(|b| b.read(cx).line_count())
            .unwrap_or(0);

        div()
            .w_full()
            .flex()
            .items_center()
            .bg(chrome.panel_bg)
            .border_b_1()
            .border_color(chrome.header_border)
            .px(px(12.0))
            .py(px(6.0))
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(chrome.text_secondary)
                    .child("Go to Line:"),
            )
            .child(
                div().w(px(100.0)).child(
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
                                        if let Some(buffer) = this.buffers.get(this.active_tab) {
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
                    .text_color(chrome.text_secondary)
                    .child(format!("/ {}", line_count)),
            )
    }
    
    

    pub fn open_folder(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let nodes = scan_directory(&path, 2);
        self.expanded_paths = vec![path.clone()];
        let git_path = path.clone();
        self.workspace_root = Some(path.clone());
        self.file_tree_nodes = nodes;
        self.active_mode = ViewMode::Explorer;
        self.panel_visible = true;
        self.selected_tree_path = None;
        self.rebuild_file_index(&path);
        self.git_state
            .update(cx, |s, cx| s.set_workspace(git_path, cx));
        cx.notify();
    }

    fn rebuild_file_index(&mut self, root: &Path) {
        let mut index = Vec::new();
        fn walk_dir(dir: &Path, root: &Path, out: &mut Vec<(PathBuf, String, String)>, depth: usize) {
            if depth > 12 {
                return;
            }
            let entries = match std::fs::read_dir(dir) {
                Ok(e) => e,
                Err(_) => return,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                if name.starts_with('.') {
                    continue;
                }
                if path.is_file() {
                    let rel_dir = path.parent()
                        .and_then(|p| p.strip_prefix(root).ok())
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    out.push((path, name, rel_dir));
                } else if path.is_dir() {
                    if matches!(name.as_str(), "node_modules" | "target" | ".git" | "dist" | "build" | "__pycache__" | ".next") {
                        continue;
                    }
                    walk_dir(&path, root, out, depth + 1);
                }
            }
        }
        walk_dir(root, root, &mut index, 0);
        self.file_index = Arc::new(index);
    }

    fn trigger_content_search(&mut self, cx: &mut Context<Self>) {
        self.search_version += 1;
        let version = self.search_version;
        let query = self.file_search_query.clone();
        let index = self.file_index.clone();
        cx.notify();

        cx.spawn(async move |this, cx| {
            Timer::after(Duration::from_millis(200)).await;

            let still_current = cx.update(|cx| {
                this.update(cx, |this, _| this.search_version == version).unwrap_or(false)
            }).unwrap_or(false);
            if !still_current {
                return;
            }

            let results =
                smol::unblock(move || search_content(&query, &index)).await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    if this.search_version == version {
                        this.file_search_results = results;
                        cx.notify();
                    }
                });
            });
        })
        .detach();
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
        if self.active_mode == ViewMode::Terminal {
            self.panel_visible = !self.panel_visible;
        } else {
            if self.terminal_visible {
                self.terminal_visible = false;
            } else {
                if self.terminals.is_empty() {
                    self.new_terminal(window, cx);
                    return;
                }
                self.terminal_visible = true;
            }
        }
        cx.notify();
    }

    fn new_terminal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let working_dir = self.current_working_directory();
        let terminal = cx.new(|cx| TerminalView::new(cx).with_working_directory(working_dir));
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
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let border = chrome.header_border;
        let muted_fg = chrome.text_secondary;
        let active_fg = chrome.bright;
        let editor_bg = chrome.editor_bg;

        let any_running = self.terminals.iter().any(|t| t.read(cx).is_running());

        div()
            .id("terminal-tab-bar")
            .w_full()
            .h(px(36.0))
            .bg(chrome.panel_bg)
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
                            .border_color(hsla(0.0, 0.0, 1.0, 0.05))
                            .when(is_active, |el| el.bg(editor_bg).text_color(active_fg))
                            .when(!is_active, |el| {
                                el.text_color(muted_fg)
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active_terminal = idx;
                                cx.notify();
                            }))
                            .child(title)
                            .child(
                                div()
                                    .id(ElementId::Name(format!("term-tab-close-{}", idx).into()))
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(3.0))
                                    .text_color(muted_fg)
                                    .hover(|s| {
                                        s.bg(hsla(0.0, 0.0, 1.0, 0.1)).text_color(active_fg)
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.close_terminal_at(idx, cx);
                                    }))
                                    .child(Icon::new("x").size(px(12.0)).color(muted_fg)),
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
                            .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
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
                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
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
                div().flex_shrink_0().px(px(10.0)).child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded_full()
                        .bg(if any_running {
                            gpui::rgb(0x4ade80)
                        } else {
                            gpui::rgb(0x8b7b6b)
                        }),
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

    fn render_image_preview(path: &Path, ide: &IdeTheme) -> Div {
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
            .bg(ide.chrome.editor_bg)
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
                                    .text_color(ide.chrome.bright)
                                    .child(file_name),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(ide.chrome.text_secondary)
                                    .child(file_size),
                            ),
                    ),
            )
    }
  

    fn render_symbol_outline(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();

        let symbols: Vec<(String, String, usize)> =
            if let Some(buffer) = self.buffers.get(self.active_tab) {
                let state = buffer.read(cx);
                if let (Some(tree), content) = (state.syntax_tree(), state.content()) {
                    let syms = extract_symbols(tree, &content, state.language());
                    syms.into_iter()
                        .map(|s| {
                            let kind_label = format!("{:?}", s.kind);
                            (s.name, kind_label, 0)
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

        let filter = self.symbol_outline_filter.to_lowercase();
        let filtered: Vec<_> = symbols
            .into_iter()
            .filter(|(name, _, _)| filter.is_empty() || name.to_lowercase().contains(&filter))
            .collect();

        let app_entity = cx.entity().clone();

        let mut list = div().flex_col().gap(px(1.0));
        for (name, kind, _line) in filtered {
            let name_clone = name.clone();
            let app_e = app_entity.clone();
            list = list.child(
                div()
                    .px(px(8.0))
                    .py(px(3.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .rounded(px(3.0))
                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                    .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                        let search_name = name_clone.clone();
                        let _ = app_e.update(cx, |this, cx| {
                            if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                                let target_line = {
                                    let state = buffer.read(cx);
                                    let content = state.content();
                                    content.find(&search_name).map(|pos| {
                                        content[..pos].chars().filter(|&c| c == '\n').count()
                                    })
                                };
                                if let Some(line) = target_line {
                                    buffer.update(cx, |s, cx| s.goto_line(line, cx));
                                }
                            }
                            this.symbol_outline_visible = false;
                            cx.notify();
                        });
                    })
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(ide.syntax.keyword.opacity(0.7))
                            .child(kind),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(ide.chrome.bright)
                            .child(name),
                    ),
            );
        }

        div()
            .id("symbol-outline-panel")
            .absolute()
            .top(px(62.0))
            .right(px(16.0))
            .w(px(280.0))
            .max_h(px(400.0))
            .overflow_y_scroll()
            .bg(ide.chrome.panel_bg)
            .border_1()
            .border_color(hsla(0.0, 0.0, 1.0, 0.05))
            .rounded(px(6.0))
            .shadow_lg()
            .p(px(8.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .text_size(px(13.0))
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(ide.chrome.text_secondary)
                    .pb(px(4.0))
                    .child("Symbol Outline"),
            )
            .child(list)
    }

    fn render_welcome(&self, ide: &IdeTheme) -> impl IntoElement {
        use adabraka_ui::animations::easings;
        use adabraka_ui::components::gradient_text::GradientText;

        let title = div()
            .id("welcome-title")
            .child(
                GradientText::new("Shiori")
                    .text_size(px(48.0))
                    .font_weight(FontWeight::BOLD)
                    .start_color(ide.chrome.accent)
                    .end_color(ide.chrome.bright),
            )
            .with_animation(
                "welcome-title-anim",
                Animation::new(Duration::from_millis(600)).with_easing(easings::ease_out_cubic),
                |el, delta| {
                    let offset = (1.0 - delta) * 20.0;
                    el.opacity(delta).mt(px(-offset))
                },
            );

        let subtitle = div()
            .id("welcome-subtitle")
            .text_size(px(14.0))
            .text_color(ide.chrome.text_secondary)
            .child("A lightweight code editor")
            .with_animation(
                "welcome-subtitle-anim",
                Animation::new(Duration::from_millis(800)).with_easing(easings::ease_out_cubic),
                |el, delta| {
                    let delay_frac = 0.3;
                    let t = ((delta - delay_frac) / (1.0 - delay_frac)).clamp(0.0, 1.0);
                    el.opacity(t)
                },
            );

        let shortcuts = div()
            .id("welcome-shortcuts")
            .mt(px(24.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .items_center()
            .text_size(px(12.0))
            .text_color(ide.chrome.text_secondary.opacity(0.7))
            .child("Cmd+O  Open file")
            .child("Cmd+Shift+O  Open folder")
            .child("Cmd+N  New file")
            .with_animation(
                "welcome-shortcuts-anim",
                Animation::new(Duration::from_millis(1000)).with_easing(easings::ease_out_cubic),
                |el, delta| {
                    let delay_frac = 0.5;
                    let t = ((delta - delay_frac) / (1.0 - delay_frac)).clamp(0.0, 1.0);
                    let offset = (1.0 - t) * 12.0;
                    el.opacity(t).mt(px(24.0 + offset))
                },
            );

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(16.0))
            .child(title)
            .child(subtitle)
            .child(shortcuts)
    }

    fn render_icon_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let active_mode = self.active_mode;
        let panel_visible = self.panel_visible;
        let border_color = hsla(0.0, 0.0, 1.0, 0.05);

        let logo = div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(3.0))
            .py(px(16.0))
            .px(px(12.0))
            .child(
                div()
                    .w(px(24.0))
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .bg(chrome.bright),
            )
            .child(
                div()
                    .w(px(17.0))
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .bg(chrome.accent)
                    .shadow(smallvec::smallvec![gpui::BoxShadow {
                        color: hsla(chrome.accent.h, chrome.accent.s, chrome.accent.l, 0.6),
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(10.0),
                        spread_radius: px(2.0),
                        inset: false,
                    }]),
            )
            .child(
                div()
                    .w(px(17.0))
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .ml(px(7.0))
                    .bg(chrome.bright),
            )
            .child(
                div()
                    .w(px(24.0))
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .bg(chrome.bright),
            );

        let icon_button =
            move |id: &'static str,
                  icon_name: &'static str,
                  mode: ViewMode,
                  active_mode: ViewMode,
                  panel_visible: bool,
                  accent: Hsla,
                  bright: Hsla,
                  dim: Hsla| {
                let is_active = active_mode == mode && panel_visible;
                div()
                    .id(ElementId::Name(id.into()))
                    .w_full()
                    .h(px(44.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .relative()
                    .when(is_active, |el| {
                        el.bg(hsla(0.0, 0.0, 1.0, 0.1)).child(
                            div()
                                .absolute()
                                .left_0()
                                .top_0()
                                .bottom_0()
                                .w(px(3.0))
                                .bg(accent),
                        )
                    })
                    .when(!is_active, |el| {
                        el.hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                    })
                    .child(
                        Icon::new(icon_name)
                            .size(px(20.0))
                            .color(if is_active { bright } else { dim }),
                    )
            };

        let accent = chrome.accent;
        let bright = chrome.bright;
        let dim = chrome.dim;

        div()
            .w(px(64.0))
            .h_full()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .rounded(px(16.0))
            .bg(chrome.panel_bg)
            .border_1()
            .border_color(border_color)
            .shadow_lg()
            .overflow_hidden()
            .child(logo)
            .child(
                icon_button(
                    "mode-explorer",
                    "folder",
                    ViewMode::Explorer,
                    active_mode,
                    panel_visible,
                    accent,
                    bright,
                    dim,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    if this.active_mode == ViewMode::Explorer && this.panel_visible {
                        this.panel_visible = false;
                    } else {
                        this.active_mode = ViewMode::Explorer;
                        this.panel_visible = true;
                    }
                    cx.notify();
                })),
            )
            .child(
                icon_button(
                    "mode-git",
                    "git-branch",
                    ViewMode::Git,
                    active_mode,
                    panel_visible,
                    accent,
                    bright,
                    dim,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    if this.active_mode == ViewMode::Git && this.panel_visible {
                        this.panel_visible = false;
                    } else {
                        this.active_mode = ViewMode::Git;
                        this.panel_visible = true;
                    }
                    cx.notify();
                })),
            )
            .child(
                icon_button(
                    "mode-terminal",
                    "terminal",
                    ViewMode::Terminal,
                    active_mode,
                    panel_visible,
                    accent,
                    bright,
                    dim,
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    if this.active_mode == ViewMode::Terminal && this.panel_visible {
                        this.panel_visible = false;
                    } else {
                        this.active_mode = ViewMode::Terminal;
                        this.panel_visible = true;
                        if this.terminals.is_empty() {
                            this.new_terminal(window, cx);
                        }
                    }
                    cx.notify();
                })),
            )
            .child(div().flex_1())
            .child(
                icon_button(
                    "mode-settings",
                    "settings",
                    ViewMode::Settings,
                    active_mode,
                    panel_visible,
                    accent,
                    bright,
                    dim,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    if this.active_mode == ViewMode::Settings {
                        this.active_mode = ViewMode::Explorer;
                    } else {
                        this.active_mode = ViewMode::Settings;
                    }
                    cx.notify();
                })),
            )
    }

    fn render_left_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let border_color = hsla(0.0, 0.0, 1.0, 0.05);

        let panel_content: AnyElement = match self.active_mode {
            ViewMode::Explorer => self.render_explorer_panel(cx).into_any_element(),
            ViewMode::Git => self.render_git_panel(cx).into_any_element(),
            ViewMode::Terminal => self.render_terminal_panel(cx).into_any_element(),
            ViewMode::Settings => div().into_any_element(),
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(16.0))
            .bg(chrome.panel_bg)
            .border_1()
            .border_color(border_color)
            .shadow_lg()
            .overflow_hidden()
            .child(panel_content)
    }

    fn render_explorer_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;

        let app_entity = cx.entity().clone();
        let app_entity2 = cx.entity().clone();
        let app_entity_search = cx.entity().clone();
        let app_entity_clear = cx.entity().clone();

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
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .px(px(16.0))
                    .pt(px(16.0))
                    .pb(px(8.0))
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 1.0, 0.05))
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(chrome.text_secondary)
                                    .child("EXPLORER"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .w(px(8.0))
                                            .h(px(8.0))
                                            .rounded_full()
                                            .bg(hsla(0.01, 1.0, 0.67, 0.5)),
                                    )
                                    .child(
                                        div()
                                            .w(px(8.0))
                                            .h(px(8.0))
                                            .rounded_full()
                                            .bg(hsla(0.11, 0.98, 0.59, 0.5)),
                                    )
                                    .child(
                                        div()
                                            .w(px(8.0))
                                            .h(px(8.0))
                                            .rounded_full()
                                            .bg(hsla(0.37, 0.73, 0.52, 0.5)),
                                    ),
                            ),
                    )
                    .child({
                        let app_search = app_entity_search;
                        let app_clear = app_entity_clear;
                        Input::new(&self.file_search_input)
                            .placeholder("Search files...")
                            .prefix(
                                Icon::new("search")
                                    .size(px(14.0))
                                    .color(chrome.text_secondary)
                                    .into_any_element(),
                            )
                            .when(!self.file_search_query.is_empty(), |input| {
                                input.suffix(
                                    div()
                                        .id("clear-search")
                                        .cursor_pointer()
                                        .child(
                                            Icon::new("x")
                                                .size(px(12.0))
                                                .color(chrome.text_secondary),
                                        )
                                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                            let _ = app_clear.update(cx, |this, cx| {
                                                this.file_search_query.clear();
                                                this.file_search_results.clear();
                                                this.file_search_input.update(cx, |input, cx| {
                                                    input.content = SharedString::default();
                                                    cx.notify();
                                                });
                                                cx.notify();
                                            });
                                        })
                                        .into_any_element(),
                                )
                            })
                            .bg(chrome.editor_bg)
                            .rounded(px(8.0))
                            .text_size(px(12.0))
                            .on_change(move |text: SharedString, cx: &mut App| {
                                let _ = app_search.update(cx, |this, cx| {
                                    this.file_search_query = text.to_string();
                                    if this.file_search_query.is_empty() {
                                        this.file_search_results.clear();
                                        this.search_version += 1;
                                        cx.notify();
                                    } else {
                                        this.trigger_content_search(cx);
                                    }
                                });
                            })
                    }),
            )
            .child(
                div()
                    .id("sidebar-tree")
                    .flex_1()
                    .overflow_y_scroll()
                    .when(self.file_search_query.is_empty(), |el| {
                        el.child(tree)
                    })
                    .when(!self.file_search_query.is_empty(), |el| {
                        el.child(self.render_file_search_results(cx))
                    }),
            )
    }

    fn render_file_search_results(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let theme = adabraka_ui::theme::use_theme();
        let app_entity = cx.entity().clone();
        let accent = chrome.accent;

        let results = &self.file_search_results;
        let searching = !self.file_search_query.is_empty() && results.is_empty();

        let mut grouped: Vec<(PathBuf, String, String, Vec<(usize, String, usize, usize)>)> = Vec::new();
        for r in results {
            if let Some(group) = grouped.last_mut() {
                if group.0 == r.path {
                    group.3.push((r.line_number, r.line_content.clone(), r.col_start, r.col_end));
                    continue;
                }
            }
            grouped.push((
                r.path.clone(),
                r.file_name.clone(),
                r.dir_path.clone(),
                vec![(r.line_number, r.line_content.clone(), r.col_start, r.col_end)],
            ));
        }

        div()
            .w_full()
            .flex()
            .flex_col()
            .py(px(4.0))
            .children(grouped.into_iter().map(move |(path, file_name, dir_path, lines)| {
                let app_e = app_entity.clone();
                let node = FileNode::file(path.clone());
                let icon_name = node.file_icon(false);
                let icon_color = node.file_icon_color(&theme);
                let match_count = lines.len();

                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .mb(px(2.0))
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .h(px(26.0))
                            .px(px(12.0))
                            .child(
                                Icon::new(icon_name)
                                    .size(px(14.0))
                                    .color(icon_color),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(chrome.bright)
                                    .child(file_name),
                            )
                            .when(!dir_path.is_empty(), |el| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(chrome.text_secondary.opacity(0.5))
                                        .text_ellipsis()
                                        .child(dir_path),
                                )
                            })
                            .child(
                                div()
                                    .ml_auto()
                                    .text_size(px(10.0))
                                    .px(px(5.0))
                                    .py(px(1.0))
                                    .rounded(px(8.0))
                                    .bg(hsla(0.0, 0.0, 1.0, 0.1))
                                    .text_color(chrome.text_secondary)
                                    .child(format!("{}", match_count)),
                            ),
                    )
                    .children(lines.into_iter().enumerate().map({
                        let path = path.clone();
                        let app_e = app_e.clone();
                        move |(i, (line_num, line_content, _col_start, _col_end))| {
                            let path = path.clone();
                            let app_e = app_e.clone();
                            div()
                                .id(SharedString::from(format!("sr-{}-{}", path.display(), i)))
                                .w_full()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .h(px(22.0))
                                .pl(px(32.0))
                                .pr(px(12.0))
                                .cursor_pointer()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(accent.opacity(0.7))
                                        .w(px(32.0))
                                        .flex_shrink_0()
                                        .child(format!("{}", line_num)),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .text_size(px(11.0))
                                        .text_color(chrome.text_secondary)
                                        .text_ellipsis()
                                        .overflow_x_hidden()
                                        .child(line_content),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                    let path = path.clone();
                                    let _ = app_e.update(cx, |this, cx| {
                                        this.selected_tree_path = Some(path.clone());
                                        if path.is_file() {
                                            let already_open = this
                                                .tab_meta
                                                .iter()
                                                .position(|meta| meta.file_path.as_ref() == Some(&path));
                                            if let Some(idx) = already_open {
                                                this.active_tab = idx;
                                            } else {
                                                this.open_paths(vec![path], cx);
                                            }
                                            if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                                                cx.spawn({
                                                    let buffer = buffer.clone();
                                                    async move |_, cx| {
                                                        Timer::after(Duration::from_millis(50)).await;
                                                        let _ = cx.update(|cx| {
                                                            buffer.update(cx, |state, cx| {
                                                                state.goto_line(line_num, cx);
                                                            });
                                                        });
                                                    }
                                                }).detach();
                                            }
                                        }
                                        cx.notify();
                                    });
                                })
                        }
                    }))
            }))
            .when(searching, |el| {
                el.child(
                    div()
                        .w_full()
                        .py(px(20.0))
                        .flex()
                        .justify_center()
                        .text_size(px(12.0))
                        .text_color(chrome.text_secondary.opacity(0.5))
                        .child("Searching..."),
                )
            })
    }

    fn render_git_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let gs = self.git_state.read(cx);

        let mut staged: Vec<(usize, String, FileStatusKind)> = gs
            .file_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.staged)
            .map(|(i, e)| (i, e.path.clone(), e.status))
            .collect();
        staged.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));

        let mut changes: Vec<(usize, String, FileStatusKind)> = gs
            .file_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.staged)
            .map(|(i, e)| (i, e.path.clone(), e.status))
            .collect();
        changes.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));

        let branch = gs.summary.branch.clone();
        let commit_editor = gs.commit_editor.clone();

        let status_letter = |status: FileStatusKind| -> &'static str {
            match status {
                FileStatusKind::Modified => "M",
                FileStatusKind::Added => "A",
                FileStatusKind::Deleted => "D",
                FileStatusKind::Renamed => "R",
                FileStatusKind::Untracked => "U",
            }
        };

        let status_color = |status: FileStatusKind| -> Hsla {
            match status {
                FileStatusKind::Modified => hsla(0.12, 0.9, 0.65, 1.0),
                FileStatusKind::Added | FileStatusKind::Untracked => ide.chrome.diff_add_text,
                FileStatusKind::Deleted => ide.chrome.diff_del_text,
                FileStatusKind::Renamed => hsla(0.58, 0.7, 0.65, 1.0),
            }
        };

        let file_icon_for_path = |path: &str| -> &'static str {
            let ext = Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            match ext {
                "json" | "yaml" | "yml" | "toml" | "xml" => "file-json",
                "md" | "txt" | "doc" | "docx" | "pdf" => "file-text",
                "sh" | "bash" | "zsh" => "hash",
                "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => "image",
                "mp3" | "wav" | "ogg" | "flac" => "music",
                "mp4" | "mov" | "avi" | "webm" => "video",
                "zip" | "tar" | "gz" | "rar" | "7z" => "archive",
                _ => "file-code",
            }
        };

        let file_icon_color_for_path = |path: &str| -> Hsla {
            let ext = Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            match ext {
                "json" | "yaml" | "yml" | "toml" | "xml" => Hsla::from(rgb(0xfbbf24)),
                "md" | "txt" | "doc" | "docx" | "pdf" => Hsla::from(rgb(0xa78bfa)),
                "sh" | "bash" | "zsh" => Hsla::from(rgb(0x4ade80)),
                "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => {
                    Hsla::from(rgb(0x22c55e))
                }
                "mp3" | "wav" | "ogg" | "flac" => Hsla::from(rgb(0xf472b6)),
                "mp4" | "mov" | "avi" | "webm" => Hsla::from(rgb(0xf472b6)),
                "zip" | "tar" | "gz" | "rar" | "7z" => Hsla::from(rgb(0xfbbf24)),
                _ => Hsla::from(rgb(0x9ca3af)),
            }
        };

        let staged_count = staged.len();
        let changes_count = changes.len();

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .h(px(44.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 1.0, 0.05))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                Icon::new("git-commit-horizontal")
                                    .size(px(16.0))
                                    .color(chrome.accent),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(chrome.text_secondary)
                                    .child("SOURCE CONTROL"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .id("git-refresh-btn")
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                    .child(
                                        Icon::new("refresh-cw")
                                            .size(px(14.0))
                                            .color(chrome.text_secondary),
                                    ),
                            )
                            .child(
                                div()
                                    .id("git-more-btn")
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                    .child(
                                        Icon::new("ellipsis")
                                            .size(px(14.0))
                                            .color(chrome.text_secondary),
                                    ),
                            ),
                    ),
            )
            .when(!branch.is_empty(), |el| {
                el.child(
                    div()
                        .w_full()
                        .h(px(28.0))
                        .flex()
                        .items_center()
                        .px(px(12.0))
                        .gap(px(6.0))
                        .child(
                            Icon::new("git-branch")
                                .size(px(12.0))
                                .color(chrome.text_secondary),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(chrome.text_secondary)
                                .child(branch),
                        ),
                )
            })
            .child(
                div()
                    .w_full()
                    .px(px(8.0))
                    .py(px(6.0))
                    .child(
                        div()
                            .w_full()
                            .h(px(60.0))
                            .rounded(px(12.0))
                            .bg(chrome.editor_bg)
                            .border_1()
                            .border_color(hsla(0.0, 0.0, 1.0, 0.1))
                            .overflow_hidden()
                            .cursor(CursorStyle::IBeam)
                            .child(
                                Editor::new(&commit_editor)
                                    .show_line_numbers(false, cx)
                                    .show_border(false),
                            ),
                    )
                    .child(
                        div()
                            .id("git-commit-btn")
                            .w_full()
                            .h(px(30.0))
                            .mt(px(6.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .rounded(px(8.0))
                            .bg(chrome.accent)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.git_state.update(cx, |gs, cx| gs.do_commit(cx));
                            }))
                            .child(
                                Icon::new("check")
                                    .size(px(14.0))
                                    .color(hsla(0.0, 0.0, 1.0, 1.0)),
                            )
                            .child("Commit"),
                    ),
            )
            .child(
                div()
                    .id("git-file-list")
                    .flex_1()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .when(!staged.is_empty(), |el| {
                        let mut section = div().flex().flex_col().child(
                            div()
                                .w_full()
                                .h(px(32.0))
                                .flex()
                                .items_center()
                                .justify_between()
                                .px(px(12.0))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(6.0))
                                        .child(
                                            Icon::new("chevron-down")
                                                .size(px(12.0))
                                                .color(chrome.text_secondary),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(chrome.text_secondary)
                                                .child("STAGED CHANGES"),
                                        )
                                        .child(
                                            div()
                                                .px(px(6.0))
                                                .py(px(1.0))
                                                .rounded_full()
                                                .bg(hsla(0.0, 0.0, 1.0, 0.1))
                                                .text_size(px(10.0))
                                                .text_color(chrome.text_secondary)
                                                .child(format!("{}", staged_count)),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("unstage-all-btn")
                                        .w(px(22.0))
                                        .h(px(22.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded(px(4.0))
                                        .cursor_pointer()
                                        .opacity(0.5)
                                        .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.1)).opacity(1.0))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.git_state.update(cx, |gs, cx| {
                                                gs.unstage_all(cx);
                                            });
                                        }))
                                        .child(
                                            Icon::new("minus")
                                                .size(px(14.0))
                                                .color(chrome.text_secondary),
                                        ),
                                ),
                        );
                        for (idx, path, status) in &staged {
                            let file_idx = *idx;
                            let letter = status_letter(*status);
                            let color = status_color(*status);
                            let icon_name = file_icon_for_path(path);
                            let icon_color = file_icon_color_for_path(path);

                            let short_name = Path::new(path)
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| path.clone());
                            let dir_path = Path::new(path)
                                .parent()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();
                            let unstage_color = chrome.text_secondary;
                            section = section.child(
                                div()
                                    .id(ElementId::Name(
                                        format!("git-staged-{}", file_idx).into(),
                                    ))
                                    .w_full()
                                    .h(px(30.0))
                                    .flex()
                                    .items_center()
                                    .mx(px(8.0))
                                    .px(px(8.0))
                                    .gap(px(8.0))
                                    .rounded(px(8.0))
                                    .cursor_pointer()
                                    .text_size(px(12.0))
                                    .text_color(chrome.text_secondary)
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.git_state.update(cx, |gs, cx| {
                                            gs.select_file(file_idx, cx);
                                        });
                                    }))
                                    .child(
                                        div()
                                            .w(px(14.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(color)
                                            .child(letter),
                                    )
                                    .child(
                                        Icon::new(icon_name)
                                            .size(px(16.0))
                                            .color(icon_color),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .min_w_0()
                                            .overflow_x_hidden()
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(chrome.bright)
                                                    .flex_shrink_0()
                                                    .child(short_name),
                                            )
                                            .when(!dir_path.is_empty(), |el| {
                                                el.child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(chrome.text_secondary)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_ellipsis()
                                                        .child(dir_path),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .id(ElementId::Name(
                                                format!("git-unstage-btn-{}", file_idx).into(),
                                            ))
                                            .flex_shrink_0()
                                            .w(px(20.0))
                                            .h(px(20.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .opacity(0.5)
                                            .hover(|s| {
                                                s.bg(hsla(0.0, 0.0, 1.0, 0.1)).opacity(1.0)
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, _, cx| {
                                                    this.git_state.update(cx, |gs, cx| {
                                                        gs.toggle_stage_file(file_idx, cx);
                                                    });
                                                }),
                                            )
                                            .child(
                                                Icon::new("minus")
                                                    .size(px(14.0))
                                                    .color(unstage_color),
                                            ),
                                    ),
                            );
                        }
                        el.child(section)
                    })
                    .when(!changes.is_empty(), |el| {
                        let mut section = div().flex().flex_col().child(
                            div()
                                .w_full()
                                .h(px(32.0))
                                .flex()
                                .items_center()
                                .justify_between()
                                .px(px(12.0))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(6.0))
                                        .child(
                                            Icon::new("chevron-down")
                                                .size(px(12.0))
                                                .color(chrome.text_secondary),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(chrome.text_secondary)
                                                .child("CHANGES"),
                                        )
                                        .child(
                                            div()
                                                .px(px(6.0))
                                                .py(px(1.0))
                                                .rounded_full()
                                                .bg(hsla(0.0, 0.0, 1.0, 0.1))
                                                .text_size(px(10.0))
                                                .text_color(chrome.text_secondary)
                                                .child(format!("{}", changes_count)),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("stage-all-btn")
                                        .w(px(22.0))
                                        .h(px(22.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded(px(4.0))
                                        .cursor_pointer()
                                        .opacity(0.5)
                                        .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.1)).opacity(1.0))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.git_state.update(cx, |gs, cx| {
                                                gs.stage_all(cx);
                                            });
                                        }))
                                        .child(
                                            Icon::new("plus")
                                                .size(px(14.0))
                                                .color(chrome.text_secondary),
                                        ),
                                ),
                        );
                        for (idx, path, status) in &changes {
                            let file_idx = *idx;
                            let letter = status_letter(*status);
                            let color = status_color(*status);
                            let icon_name = file_icon_for_path(path);
                            let icon_color = file_icon_color_for_path(path);

                            let short_name = Path::new(path)
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| path.clone());
                            let dir_path = Path::new(path)
                                .parent()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();
                            let stage_color = chrome.text_secondary;
                            section = section.child(
                                div()
                                    .id(ElementId::Name(
                                        format!("git-change-{}", file_idx).into(),
                                    ))
                                    .w_full()
                                    .h(px(30.0))
                                    .flex()
                                    .items_center()
                                    .mx(px(8.0))
                                    .px(px(8.0))
                                    .gap(px(8.0))
                                    .rounded(px(8.0))
                                    .cursor_pointer()
                                    .text_size(px(12.0))
                                    .text_color(chrome.text_secondary)
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.git_state.update(cx, |gs, cx| {
                                            gs.select_file(file_idx, cx);
                                        });
                                    }))
                                    .child(
                                        div()
                                            .w(px(14.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(color)
                                            .child(letter),
                                    )
                                    .child(
                                        Icon::new(icon_name)
                                            .size(px(16.0))
                                            .color(icon_color),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .min_w_0()
                                            .overflow_x_hidden()
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(chrome.bright)
                                                    .flex_shrink_0()
                                                    .child(short_name),
                                            )
                                            .when(!dir_path.is_empty(), |el| {
                                                el.child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(chrome.text_secondary)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_ellipsis()
                                                        .child(dir_path),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .id(ElementId::Name(
                                                format!("git-stage-btn-{}", file_idx).into(),
                                            ))
                                            .flex_shrink_0()
                                            .w(px(20.0))
                                            .h(px(20.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .opacity(0.5)
                                            .hover(|s| {
                                                s.bg(hsla(0.0, 0.0, 1.0, 0.1)).opacity(1.0)
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, _, cx| {
                                                    this.git_state.update(cx, |gs, cx| {
                                                        gs.toggle_stage_file(file_idx, cx);
                                                    });
                                                }),
                                            )
                                            .child(
                                                Icon::new("plus")
                                                    .size(px(14.0))
                                                    .color(stage_color),
                                            ),
                                    ),
                            );
                        }
                        el.child(section)
                    }),
            )
    }

    fn render_terminal_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .h(px(44.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 1.0, 0.05))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(chrome.text_secondary)
                            .child("TERMINALS"),
                    )
                    .child(
                        div()
                            .id("new-terminal-panel-btn")
                            .w(px(22.0))
                            .h(px(22.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.new_terminal(window, cx);
                            }))
                            .child(
                                Icon::new("plus")
                                    .size(px(14.0))
                                    .color(chrome.text_secondary),
                            ),
                    ),
            )
            .child(
                div()
                    .id("terminal-session-list")
                    .flex_1()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .children(self.terminals.iter().enumerate().map(|(idx, term)| {
                        let is_active = idx == self.active_terminal;
                        let title = term.read(cx).title();
                        let running = term.read(cx).is_running();
                        let status_text = if running { "Running" } else { "Stopped" };

                        div()
                            .id(ElementId::Name(format!("term-session-{}", idx).into()))
                            .w_full()
                            .h(px(44.0))
                            .flex()
                            .items_center()
                            .pl(px(0.0))
                            .pr(px(12.0))
                            .gap(px(8.0))
                            .cursor_pointer()
                            .border_l_2()
                            .when(is_active, |el| {
                                el.border_color(chrome.accent)
                                    .bg(hsla(0.0, 0.0, 1.0, 0.05))
                            })
                            .when(!is_active, |el| {
                                el.border_color(transparent_black())
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.03)))
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active_terminal = idx;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .pl(px(10.0))
                                    .child(
                                        Icon::new("terminal")
                                            .size(px(16.0))
                                            .color(if is_active {
                                                chrome.bright
                                            } else {
                                                chrome.dim
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .overflow_x_hidden()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(if is_active {
                                                chrome.bright
                                            } else {
                                                chrome.text_secondary
                                            })
                                            .text_ellipsis()
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(
                                                div()
                                                    .w(px(6.0))
                                                    .h(px(6.0))
                                                    .rounded_full()
                                                    .bg(if running {
                                                        gpui::rgb(0x4ade80)
                                                    } else {
                                                        gpui::rgb(0x6b7280)
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(chrome.text_secondary)
                                                    .child(status_text),
                                            ),
                                    ),
                            )
                    })),
            )
    }

    fn render_settings_view(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;
        let all_themes = all_ide_themes();
        let current_name = ide.name;

        let mut grid = div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap(px(12.0))
            .px(px(24.0));

        for (i, theme) in all_themes.iter().enumerate() {
            let is_current = theme.name == current_name;
            let theme_clone = theme.clone();

            let preview = div()
                .w_full()
                .h(px(48.0))
                .rounded_t(px(8.0))
                .flex()
                .items_end()
                .gap(px(4.0))
                .p(px(8.0))
                .bg(theme.chrome.bg)
                .child(
                    div()
                        .w(px(16.0))
                        .h(px(24.0))
                        .rounded(px(3.0))
                        .bg(theme.chrome.panel_bg),
                )
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(24.0))
                        .rounded(px(3.0))
                        .bg(theme.chrome.editor_bg),
                )
                .child(
                    div()
                        .w(px(12.0))
                        .h(px(8.0))
                        .rounded(px(2.0))
                        .bg(theme.chrome.accent),
                )
                .child(
                    div()
                        .w(px(20.0))
                        .h(px(4.0))
                        .rounded(px(2.0))
                        .bg(theme.chrome.bright),
                );

            let card = div()
                .id(ElementId::Name(format!("theme-card-{}", i).into()))
                .w(px(180.0))
                .rounded(px(10.0))
                .border_1()
                .overflow_hidden()
                .cursor_pointer()
                .when(is_current, |el| el.border_color(chrome.accent))
                .when(!is_current, |el| {
                    el.border_color(hsla(0.0, 0.0, 1.0, 0.1))
                        .hover(|s| s.border_color(hsla(0.0, 0.0, 1.0, 0.2)))
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    install_ide_theme(theme_clone.clone());
                    sync_adabraka_theme_from_ide(cx);
                    for buffer in &this.buffers {
                        buffer.update(cx, |state, cx| {
                            state.invalidate_line_layouts(cx);
                        });
                    }
                    for terminal in &this.terminals {
                        terminal.update(cx, |tv, _cx| {
                            tv.apply_ide_theme();
                        });
                    }
                    cx.notify();
                }))
                .child(preview)
                .child(
                    div()
                        .w_full()
                        .px(px(10.0))
                        .py(px(8.0))
                        .bg(chrome.panel_bg)
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(chrome.bright)
                                        .child(theme.name),
                                )
                                .when(is_current, |el| {
                                    el.child(
                                        Icon::new("check")
                                            .size(px(14.0))
                                            .color(chrome.accent),
                                    )
                                }),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(chrome.text_secondary)
                                .child(theme.description),
                        ),
                );

            grid = grid.child(card);
        }

        div()
            .id("settings-view")
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .p(px(24.0))
            .gap(px(20.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(chrome.bright)
                            .child("Appearance"),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(chrome.text_secondary)
                            .child("Customize the look and feel of the editor"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(chrome.text_secondary)
                            .child("COLOR THEMES"),
                    )
                    .child(grid),
            )
    }

    fn toggle_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.command_palette_open {
            self.command_palette_open = false;
            self.command_palette = None;
            cx.notify();
            return;
        }

        let commands = self.create_commands(cx);
        let app_entity = cx.entity().clone();
        let palette = cx.new(|palette_cx| {
            CommandPalette::new(window, palette_cx, commands).on_close(move |_, cx| {
                let _ = app_entity.update(cx, |this, cx| {
                    this.command_palette_open = false;
                    this.command_palette = None;
                    cx.notify();
                });
            })
        });
        let focus = palette.read(cx).focus_handle(cx);
        self.command_palette = Some(palette);
        self.command_palette_open = true;
        window.focus(&focus);
        cx.notify();
    }

    fn create_commands(&self, cx: &Context<Self>) -> Vec<Command> {
        let app = cx.entity().clone();

        let mut commands = Vec::new();

        let a = app.clone();
        commands.push(
            Command::new("fold-all", "Fold All")
                .category("Editor")
                .shortcut("⌘K ⌘0")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                            buffer.update(cx, |state, cx| state.fold_all(cx));
                        }
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("unfold-all", "Unfold All")
                .category("Editor")
                .shortcut("⌘K ⌘J")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                            buffer.update(cx, |state, cx| state.unfold_all(cx));
                        }
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("toggle-fold", "Toggle Fold at Cursor")
                .category("Editor")
                .shortcut("⌘⇧[")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                            let line = buffer.read(cx).cursor().line;
                            buffer.update(cx, |state, cx| state.toggle_fold_at_line(line, cx));
                        }
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("new-file", "New File")
                .category("File")
                .shortcut("⌘N")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.new_file(cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("open-file", "Open File")
                .category("File")
                .shortcut("⌘O")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.open_file_dialog(cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("open-folder", "Open Folder")
                .category("File")
                .shortcut("⌘⇧K")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.open_folder_dialog(cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("save-file", "Save File")
                .category("File")
                .shortcut("⌘S")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.save_active(cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("close-tab", "Close Tab")
                .category("File")
                .shortcut("⌘W")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.close_active_tab(cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("goto-line", "Go to Line")
                .category("Navigation")
                .shortcut("⌘G")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.goto_line_visible = true;
                        cx.notify();
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("next-tab", "Next Tab")
                .category("Navigation")
                .shortcut("⌃Tab")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if !this.buffers.is_empty() {
                            this.active_tab = (this.active_tab + 1) % this.buffers.len();
                            this.clamp_tab_scroll();
                            cx.notify();
                        }
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("prev-tab", "Previous Tab")
                .category("Navigation")
                .shortcut("⌃⇧Tab")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if !this.buffers.is_empty() {
                            this.active_tab = if this.active_tab == 0 {
                                this.buffers.len() - 1
                            } else {
                                this.active_tab - 1
                            };
                            this.clamp_tab_scroll();
                            cx.notify();
                        }
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("symbol-outline", "Symbol Outline")
                .category("Navigation")
                .shortcut("⌘⇧O")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.symbol_outline_visible = !this.symbol_outline_visible;
                        this.symbol_outline_filter.clear();
                        cx.notify();
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("toggle-sidebar", "Toggle Sidebar")
                .category("View")
                .shortcut("⌘B")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if this.panel_visible && this.active_mode == ViewMode::Explorer {
                            this.panel_visible = false;
                        } else {
                            this.active_mode = ViewMode::Explorer;
                            this.panel_visible = true;
                        }
                        cx.notify();
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("toggle-terminal", "Toggle Terminal")
                .category("View")
                .shortcut("⌘`")
                .on_select(move |window, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.toggle_terminal(window, cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("new-terminal", "New Terminal")
                .category("View")
                .on_select(move |window, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.new_terminal(window, cx);
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("toggle-search", "Toggle Search")
                .category("View")
                .shortcut("⌘F")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.search_visible = !this.search_visible;
                        cx.notify();
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("search-replace", "Search & Replace")
                .category("View")
                .shortcut("⌘H")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        this.search_visible = true;
                        cx.notify();
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("toggle-git", "Toggle Git View")
                .category("View")
                .shortcut("⌘⇧G")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if this.active_mode == ViewMode::Git && this.panel_visible {
                            this.panel_visible = false;
                        } else {
                            this.active_mode = ViewMode::Git;
                            this.panel_visible = true;
                        }
                        cx.notify();
                    });
                }),
        );

        let a = app.clone();
        commands.push(
            Command::new("settings", "Settings")
                .category("Appearance")
                .on_select(move |_, cx| {
                    let _ = a.update(cx, |this, cx| {
                        if this.active_mode == ViewMode::Settings {
                            this.active_mode = ViewMode::Explorer;
                        } else {
                            this.active_mode = ViewMode::Settings;
                        }
                        cx.notify();
                    });
                }),
        );

        commands
    }
}

impl Focusable for AppState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AppState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ide = use_ide_theme();
        let chrome = &ide.chrome;

        let search_visible = self.search_visible;
        let goto_visible = self.goto_line_visible;
        let is_settings = self.active_mode == ViewMode::Settings;
        let is_git_mode = self.active_mode == ViewMode::Git;
        let is_terminal_mode = self.active_mode == ViewMode::Terminal;
        let show_left_panel =
            self.panel_visible && self.active_mode != ViewMode::Settings;

        let build_editor = |buffer: &Entity<EditorState>, cx: &mut App| {
            let syn = ide.syntax.clone();
            div()
                .size_full()
                .cursor(CursorStyle::IBeam)
                .child(
                    Editor::new(buffer)
                        .show_line_numbers(true, cx)
                        .show_border(false)
                        .cursor_color(ide.editor.cursor)
                        .selection_color(ide.editor.selection)
                        .line_number_color(ide.editor.line_number)
                        .line_number_active_color(ide.editor.line_number_active)
                        .gutter_bg(ide.editor.gutter_bg)
                        .search_match_colors(ide.editor.search_match, ide.editor.search_match_active)
                        .current_line_color(ide.editor.current_line)
                        .bracket_match_color(ide.editor.bracket_match)
                        .word_highlight_color(ide.editor.word_highlight)
                        .indent_guide_colors(ide.editor.indent_guide, ide.editor.indent_guide_active)
                        .fold_marker_color(ide.editor.fold_marker)
                        .syntax_color_fn(move |name| syn.color_for_capture(name))
                        .bg(gpui::transparent_black()),
                )
        };

        let has_tabs = !self.buffers.is_empty();
        let active_is_image = self
            .tab_meta
            .get(self.active_tab)
            .map(|m| m.is_image)
            .unwrap_or(false);
        let active_image_path = if active_is_image {
            self.tab_meta
                .get(self.active_tab)
                .and_then(|m| m.file_path.clone())
        } else {
            None
        };

        let breadcrumbs: Option<Vec<(String, usize)>> = if has_tabs && !active_is_image {
            self.buffers
                .get(self.active_tab)
                .map(|b| b.read(cx).scope_breadcrumbs())
        } else {
            None
        };

        let border_color = hsla(0.0, 0.0, 1.0, 0.05);

        let main_content_element: AnyElement = if is_settings {
            self.render_settings_view(cx).into_any_element()
        } else if is_git_mode {
            div()
                .size_full()
                .child(GitView::new(self.git_state.clone()).diff_only(true))
                .into_any_element()
        } else if is_terminal_mode {
            let active_terminal = self.terminals.get(self.active_terminal).cloned();
            if let Some(term) = active_terminal {
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .child(div().flex_1().overflow_hidden().child(term))
                    .into_any_element()
            } else {
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(chrome.text_secondary)
                    .text_size(px(14.0))
                    .child("No terminal sessions")
                    .into_any_element()
            }
        } else {
            let right_pane_content: AnyElement = if !has_tabs {
                self.render_welcome(&ide).into_any_element()
            } else if let Some(image_path) = &active_image_path {
                Self::render_image_preview(image_path, &ide).into_any_element()
            } else if let Some(buffer) = self.buffers.get(self.active_tab) {
                build_editor(buffer, cx).into_any_element()
            } else {
                self.render_welcome(&ide).into_any_element()
            };

            let tab_bar_row = if !self.terminal_fullscreen {
                let row = div()
                    .w_full()
                    .h(px(36.0))
                    .flex()
                    .items_center()
                    .border_b_1()
                    .border_color(border_color);
                if has_tabs {
                    Some(row.child(self.render_tab_bar(cx)))
                } else {
                    Some(row.child(div().flex_1()))
                }
            } else {
                None
            };

            let breadcrumb_bar = if let Some(crumbs) = &breadcrumbs {
                if !crumbs.is_empty() {
                    let app_entity_bc = cx.entity().clone();
                    let mut row = div()
                        .w_full()
                        .h(px(24.0))
                        .flex()
                        .items_center()
                        .px(px(12.0))
                        .gap(px(4.0))
                        .bg(chrome.panel_bg.opacity(0.5))
                        .border_b_1()
                        .border_color(border_color)
                        .text_size(px(12.0))
                        .text_color(chrome.text_secondary);
                    for (i, (name, line)) in crumbs.iter().enumerate() {
                        if i > 0 {
                            row = row.child(
                                div()
                                    .text_color(chrome.text_secondary.opacity(0.5))
                                    .child("\u{203A}"),
                            );
                        }
                        let target_line = *line;
                        let app_bc = app_entity_bc.clone();
                        row = row.child(
                            div()
                                .cursor_pointer()
                                .text_color(chrome.text_secondary)
                                .hover(|s| s.text_color(chrome.bright))
                                .child(name.clone())
                                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                    let _ = app_bc.update(cx, |this, cx| {
                                        if let Some(buffer) =
                                            this.buffers.get(this.active_tab).cloned()
                                        {
                                            buffer.update(cx, |state, cx| {
                                                state.goto_line(target_line, cx);
                                            });
                                        }
                                    });
                                }),
                        );
                    }
                    Some(row)
                } else {
                    None
                }
            } else {
                None
            };

            let editor_pane = div()
                .size_full()
                .flex()
                .flex_col()
                .children(tab_bar_row)
                .children(breadcrumb_bar)
                .child(div().flex_1().overflow_hidden().child(right_pane_content));

            if self.terminal_fullscreen || is_terminal_mode {
                let terminal_tab_bar = self.render_terminal_tab_bar(cx);
                let active_terminal = self.terminals.get(self.active_terminal).cloned();
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .child(terminal_tab_bar)
                    .child(div().flex_1().overflow_hidden().children(active_terminal))
                    .into_any_element()
            } else if self.terminal_visible && !is_git_mode && !is_settings {
                let terminal_tab_bar = self.render_terminal_tab_bar(cx);
                let active_terminal = self.terminals.get(self.active_terminal).cloned();
                div()
                    .size_full()
                    .child(
                        v_resizable("editor-terminal", self.terminal_resizable_state.clone())
                            .child(resizable_panel().child(editor_pane))
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
                editor_pane.into_any_element()
            }
        };

        let main_area = div()
            .flex_1()
            .h_full()
            .rounded(px(16.0))
            .bg(chrome.editor_bg)
            .border_1()
            .border_color(border_color)
            .shadow_lg()
            .overflow_hidden()
            .flex()
            .flex_col()
            .when(search_visible && !self.terminal_fullscreen && !is_settings, |el| {
                el.child(self.search_bar.clone())
            })
            .when(goto_visible && !self.terminal_fullscreen && !is_settings, |el| {
                el.child(self.render_goto_line(cx))
            })
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .rounded_b(px(16.0))
                    .child(main_content_element),
            );

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
                if this.panel_visible && this.active_mode == ViewMode::Explorer {
                    this.panel_visible = false;
                } else {
                    this.active_mode = ViewMode::Explorer;
                    this.panel_visible = true;
                }
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleTerminal, window, cx| {
                this.toggle_terminal(window, cx);
            }))
            .on_action(
                cx.listener(|this, _: &ToggleTerminalFullscreen, window, cx| {
                    this.toggle_terminal_fullscreen(window, cx);
                }),
            )
            .on_action(cx.listener(|this, _: &NewTerminal, window, cx| {
                this.new_terminal(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleGitView, _, cx| {
                if this.active_mode == ViewMode::Git && this.panel_visible {
                    this.panel_visible = false;
                    this.active_mode = ViewMode::Explorer;
                } else {
                    this.active_mode = ViewMode::Git;
                    this.panel_visible = true;
                }
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleSymbolOutline, _, cx| {
                this.symbol_outline_visible = !this.symbol_outline_visible;
                this.symbol_outline_filter.clear();
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleCommandPalette, window, cx| {
                this.toggle_command_palette(window, cx);
            }))
            .on_action(cx.listener(|this, _: &FoldToggle, _, cx| {
                if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                    let line = buffer.read(cx).cursor().line;
                    buffer.update(cx, |state, cx| {
                        state.toggle_fold_at_line(line, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &FoldAll, _, cx| {
                if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                    buffer.update(cx, |state, cx| state.fold_all(cx));
                }
            }))
            .on_action(cx.listener(|this, _: &UnfoldAll, _, cx| {
                if let Some(buffer) = this.buffers.get(this.active_tab).cloned() {
                    buffer.update(cx, |state, cx| state.unfold_all(cx));
                }
            }))
            .on_action(cx.listener(|this, _: &GitNextFile, _, cx| {
                if this.active_mode == ViewMode::Git {
                    this.git_state.update(cx, |s, cx| s.select_next_file(cx));
                }
            }))
            .on_action(cx.listener(|this, _: &GitPrevFile, _, cx| {
                if this.active_mode == ViewMode::Git {
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
                } else if this.search_visible || this.goto_line_visible {
                    this.close_search_internal(cx);
                } else if this.panel_visible {
                    this.panel_visible = false;
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
            .bg(chrome.bg)
            .child(
                div()
                    .id("titlebar")
                    .w_full()
                    .h(px(44.0))
                    .flex_shrink_0()
                    .window_control_area(WindowControlArea::Drag)
                    .on_click(|event, window, _cx| {
                        if event.click_count() == 2 {
                            window.titlebar_double_click();
                        }
                    }),
            )
            .child(
                {
                    let content_area: AnyElement = if show_left_panel {
                        div()
                            .flex_1()
                            .flex()
                            .gap(px(8.0))
                            .overflow_hidden()
                            .child(
                                h_resizable(
                                    "sidebar-main",
                                    self.sidebar_resizable_state.clone(),
                                )
                                .child(
                                    resizable_panel()
                                        .size(px(256.0))
                                        .min_size(px(180.0))
                                        .max_size(px(450.0))
                                        .child(self.render_left_panel(cx)),
                                )
                                .child(
                                resizable_panel().child(
                                    div().size_full().pl(px(8.0)).child(main_area),
                                ),
                            ),
                            )
                            .into_any_element()
                    } else {
                        main_area.into_any_element()
                    };

                    div()
                        .flex_1()
                        .flex()
                        .overflow_hidden()
                        .px(px(8.0))
                        .pb(px(8.0))
                        .gap(px(8.0))
                        .child(self.render_icon_sidebar(cx))
                        .child(content_area)
                        .when(self.symbol_outline_visible, |el| {
                            el.child(self.render_symbol_outline(cx))
                        })
                }
            )
            .when_some(
                self.command_palette
                    .as_ref()
                    .filter(|_| self.command_palette_open),
                |el, palette| el.child(palette.clone()),
            )
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
