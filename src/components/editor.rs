//! High-performance SQL code editor component with tree-sitter syntax highlighting
//!
//! A feature-complete SQL editor optimized for PostgreSQL with precise syntax highlighting.
//! # Features
//! - Multi-line text editing with full cursor and selection support
//! - Real-time syntax highlighting using tree-sitter for SQL
//! - Line numbers with proper gutter
//! - Keyboard shortcuts (arrow keys, home/end, page up/down, etc.)
//! - Mouse selection support
//! - Copy/paste/cut clipboard operations
//! - Vertical scrolling with visible scrollbars
//!
//! # Example
//! ```ignore
//! use adabraka_ui::components::editor::{Editor, EditorState};
//!
//! let editor = cx.new(|cx| EditorState::new(cx));
//! Editor::new(&editor)
//!     .content("SELECT * FROM users WHERE age > 21;")
//!     .show_line_numbers(true)
//! ```

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;
use crate::components::scrollable::scrollable_vertical;
use tree_sitter::{Parser, Tree, Node};
use once_cell::sync::Lazy;
use std::cmp::min;
use std::ops::Range;

static SQL_PARSER: Lazy<std::sync::Mutex<Parser>> = Lazy::new(|| {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_sequel::LANGUAGE.into())
        .expect("Failed to load SQL grammar");
    std::sync::Mutex::new(parser)
});

fn collect_highlights(node: Node, highlights: &mut Vec<(usize, usize, String)>) {
    let start = node.start_byte();
    let end = node.end_byte();
    let kind = node.kind();

    // tree-sitter-sequel uses these specific node kinds
    let syntax_category = if kind.starts_with("keyword_") {
        Some("keyword")
    } else {
        match kind {
            "literal" => Some("literal"),
            "comment" | "line_comment" | "block_comment" | "marginal_comment" => Some("comment"),
            "invocation" => Some("function"),
            // Don't highlight structural nodes, only leaf nodes
            _ => None,
        }
    };

    if let Some(category) = syntax_category {
        highlights.push((start, end, category.to_string()));
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_highlights(child, highlights);
    }
}

/// Get color for a specific syntax token type using standard SQL editor colors
fn get_highlight_color(capture_name: &str, _theme: &crate::theme::Theme) -> Hsla {
    match capture_name {
        "keyword" => hsla(0.58, 0.85, 0.65, 1.0),
        "literal" => hsla(0.08, 0.90, 0.65, 1.0),
        "comment" => hsla(0.33, 0.50, 0.55, 1.0),
        "function" => hsla(0.83, 0.70, 0.70, 1.0),
        "identifier" => hsla(0.0, 0.0, 0.9, 1.0),
        _ => hsla(0.0, 0.0, 0.9, 1.0),
    }
}

actions!(
    editor,
    [
        MoveUp,
        MoveDown,
        MoveLeft,
        MoveRight,
        MoveToLineStart,
        MoveToLineEnd,
        MoveToDocStart,
        MoveToDocEnd,
        PageUp,
        PageDown,
        SelectUp,
        SelectDown,
        SelectLeft,
        SelectRight,
        SelectToLineStart,
        SelectToLineEnd,
        SelectAll,
        Backspace,
        Delete,
        Enter,
        Tab,
        Copy,
        Cut,
        Paste,
    ]
);

/// Initialize editor key bindings
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", MoveUp, Some("Editor")),
        KeyBinding::new("down", MoveDown, Some("Editor")),
        KeyBinding::new("left", MoveLeft, Some("Editor")),
        KeyBinding::new("right", MoveRight, Some("Editor")),
        KeyBinding::new("home", MoveToLineStart, Some("Editor")),
        KeyBinding::new("end", MoveToLineEnd, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-up", MoveToDocStart, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-home", MoveToDocStart, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-down", MoveToDocEnd, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-end", MoveToDocEnd, Some("Editor")),
        KeyBinding::new("pageup", PageUp, Some("Editor")),
        KeyBinding::new("pagedown", PageDown, Some("Editor")),
        KeyBinding::new("shift-up", SelectUp, Some("Editor")),
        KeyBinding::new("shift-down", SelectDown, Some("Editor")),
        KeyBinding::new("shift-left", SelectLeft, Some("Editor")),
        KeyBinding::new("shift-right", SelectRight, Some("Editor")),
        KeyBinding::new("shift-home", SelectToLineStart, Some("Editor")),
        KeyBinding::new("shift-end", SelectToLineEnd, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some("Editor")),
        KeyBinding::new("backspace", Backspace, Some("Editor")),
        KeyBinding::new("delete", Delete, Some("Editor")),
        KeyBinding::new("enter", Enter, Some("Editor")),
        KeyBinding::new("tab", Tab, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some("Editor")),
    ]);
}

/// Position in the editor (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    pub fn zero() -> Self {
        Self { line: 0, col: 0 }
    }
}

/// Selection range in the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Position,
    pub cursor: Position,
}

impl Selection {
    pub fn new(anchor: Position, cursor: Position) -> Self {
        Self { anchor, cursor }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.cursor
    }

    /// Get the ordered range (start, end)
    pub fn range(&self) -> (Position, Position) {
        if self.anchor <= self.cursor {
            (self.anchor, self.cursor)
        } else {
            (self.cursor, self.anchor)
        }
    }
}

/// Core editor state with EntityInputHandler support
pub struct EditorState {
    focus_handle: FocusHandle,
    lines: Vec<String>,
    cursor: Position,
    selection: Option<Selection>,
    selection_reversed: bool,
    scroll_offset: usize,
    marked_range: Option<Range<usize>>,

    last_bounds: Option<Bounds<Pixels>>,
    line_layouts: Vec<Option<ShapedLine>>,
    syntax_tree: Option<Tree>,

    pub show_line_numbers: bool,
    tab_size: usize,
    read_only: bool,

    is_selecting: bool,
    last_click_time: Option<std::time::Instant>,

    // Scroll state for programmatic control of the scrollable container
    scroll_handle: ScrollHandle,
}

impl EditorState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            lines: vec![String::new()],
            cursor: Position::zero(),
            selection: None,
            selection_reversed: false,
            scroll_offset: 0,
            marked_range: None,
            last_bounds: None,
            line_layouts: vec![],
            syntax_tree: None,
            show_line_numbers: true,
            tab_size: 4,
            read_only: false,
            is_selecting: false,
            last_click_time: None,
            scroll_handle: ScrollHandle::new(),
        }
    }

    /// Get all content as a single string
    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    /// Check if the editor is empty (no content or only whitespace)
    pub fn is_empty(&self) -> bool {
        self.content().trim().is_empty()
    }

    /// Set content from a string
    pub fn set_content(&mut self, content: &str, cx: &mut Context<Self>) {
        self.lines = content.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor = Position::zero();
        self.selection = None;
        self.scroll_offset = 0;
        self.line_layouts.clear();
        self.update_syntax_tree();
        cx.notify();
    }

    /// Update the syntax tree after text changes
    fn update_syntax_tree(&mut self) {
        let content = self.content();
        let mut parser = SQL_PARSER.lock().unwrap();
        self.syntax_tree = parser.parse(&content, None);
    }

    /// Convert position to flat byte offset
    fn position_to_offset(&self, pos: Position) -> usize {
        let mut offset = 0;
        for line_idx in 0..pos.line {
            offset += self.lines[line_idx].len() + 1;
        }
        offset + pos.col
    }

    /// Convert flat byte offset to position
    fn offset_to_position(&self, mut offset: usize) -> Position {
        for (line_idx, line) in self.lines.iter().enumerate() {
            let line_len = line.len() + 1;
            if offset <= line_len {
                return Position::new(line_idx, min(offset, line.len()));
            }
            offset -= line_len;
        }
        Position::new(self.lines.len() - 1, self.lines.last().unwrap().len())
    }

    /// Convert UTF-8 offset to UTF-16 offset for the entire document
    fn offset_to_utf16(&self, offset: usize) -> usize {
        let content = self.content();
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    /// Convert UTF-16 offset to UTF-8 offset for the entire document
    fn offset_from_utf16(&self, offset: usize) -> usize {
        let content = self.content();
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    /// Get the current line
    fn current_line(&self) -> &str {
        &self.lines[self.cursor.line]
    }

    /// Get the current line mutably
    fn current_line_mut(&mut self) -> &mut String {
        &mut self.lines[self.cursor.line]
    }

    /// Clamp cursor to valid position
    fn clamp_cursor(&mut self) {
        self.cursor.line = min(self.cursor.line, self.lines.len().saturating_sub(1));
        let line_len = self.lines[self.cursor.line].len();
        self.cursor.col = min(self.cursor.col, line_len);
    }

    pub fn move_up(&mut self, _: &MoveUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.clamp_cursor();
            self.selection = None;
            cx.notify();
        }
    }

    pub fn move_down(&mut self, _: &MoveDown, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor.line < self.lines.len() - 1 {
            self.cursor.line += 1;
            self.clamp_cursor();
            self.selection = None;
            cx.notify();
        }
    }

    pub fn move_left(&mut self, _: &MoveLeft, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.col = self.lines[self.cursor.line].len();
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_right(&mut self, _: &MoveRight, _: &mut Window, cx: &mut Context<Self>) {
        let line_len = self.current_line().len();
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.line < self.lines.len() - 1 {
            self.cursor.line += 1;
            self.cursor.col = 0;
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_line_start(&mut self, _: &MoveToLineStart, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor.col = 0;
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_line_end(&mut self, _: &MoveToLineEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor.col = self.current_line().len();
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_doc_start(&mut self, _: &MoveToDocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor = Position::zero();
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_doc_end(&mut self, _: &MoveToDocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor.line = self.lines.len() - 1;
        self.cursor.col = self.lines[self.cursor.line].len();
        self.selection = None;
        cx.notify();
    }

    pub fn page_up(&mut self, _: &PageUp, _: &mut Window, cx: &mut Context<Self>) {
        let page_size = 20;
        self.cursor.line = self.cursor.line.saturating_sub(page_size);
        self.clamp_cursor();
        self.selection = None;
        cx.notify();
    }

    pub fn page_down(&mut self, _: &PageDown, _: &mut Window, cx: &mut Context<Self>) {
        let page_size = 20;
        self.cursor.line = min(self.cursor.line + page_size, self.lines.len() - 1);
        self.clamp_cursor();
        self.selection = None;
        cx.notify();
    }

    fn start_selection_if_needed(&mut self) {
        if self.selection.is_none() {
            self.selection = Some(Selection::new(self.cursor, self.cursor));
        }
    }

    pub fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.clamp_cursor();
            if let Some(ref mut sel) = self.selection {
                sel.cursor = self.cursor;
            }
            cx.notify();
        }
    }

    pub fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        if self.cursor.line < self.lines.len() - 1 {
            self.cursor.line += 1;
            self.clamp_cursor();
            if let Some(ref mut sel) = self.selection {
                sel.cursor = self.cursor;
            }
            cx.notify();
        }
    }

    pub fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.col = self.lines[self.cursor.line].len();
        }
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        let line_len = self.current_line().len();
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.line < self.lines.len() - 1 {
            self.cursor.line += 1;
            self.cursor.col = 0;
        }
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_to_line_start(&mut self, _: &SelectToLineStart, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        self.cursor.col = 0;
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_to_line_end(&mut self, _: &SelectToLineEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        self.cursor.col = self.current_line().len();
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        let start = Position::zero();
        let end = Position::new(
            self.lines.len() - 1,
            self.lines[self.lines.len() - 1].len(),
        );
        self.selection = Some(Selection::new(start, end));
        self.cursor = end;
        cx.notify();
    }

    pub fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }

        if let Some(selection) = self.selection.take() {
            self.delete_selection(selection, cx);
        } else if self.cursor.col > 0 {
            self.cursor.col -= 1;
            let col = self.cursor.col;
            self.current_line_mut().remove(col);
            self.update_syntax_tree();
            cx.notify();
        } else if self.cursor.line > 0 {
            let current = self.lines.remove(self.cursor.line);
            self.cursor.line -= 1;
            self.cursor.col = self.lines[self.cursor.line].len();
            self.lines[self.cursor.line].push_str(&current);
            self.update_syntax_tree();
            cx.notify();
        }
    }

    pub fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }

        if let Some(selection) = self.selection.take() {
            self.delete_selection(selection, cx);
        } else {
            let line_len = self.current_line().len();
            if self.cursor.col < line_len {
                let col = self.cursor.col;
                self.current_line_mut().remove(col);
                self.update_syntax_tree();
                cx.notify();
            } else if self.cursor.line < self.lines.len() - 1 {
                let next = self.lines.remove(self.cursor.line + 1);
                self.lines[self.cursor.line].push_str(&next);
                self.update_syntax_tree();
                cx.notify();
            }
        }
    }

    pub fn enter(&mut self, _: &Enter, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }

        if let Some(selection) = self.selection.take() {
            self.delete_selection(selection, cx);
        }

        let col = self.cursor.col;
        let current = self.current_line_mut();
        let remainder = current.split_off(col);
        self.cursor.line += 1;
        self.cursor.col = 0;
        self.lines.insert(self.cursor.line, remainder);
        self.update_syntax_tree();
        self.ensure_cursor_visible_after_newline(cx);
    }

    pub fn tab(&mut self, _: &Tab, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }

        if let Some(selection) = self.selection.take() {
            self.delete_selection(selection, cx);
        }
        let spaces = " ".repeat(self.tab_size);
        self.insert_text(&spaces, cx);
    }

    pub fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(selection) = &self.selection {
            let text = self.get_selection_text(selection);
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }
    }

    pub fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }

        if let Some(selection) = self.selection.take() {
            let text = self.get_selection_text(&selection);
            cx.write_to_clipboard(ClipboardItem::new_string(text));
            self.delete_selection(selection, cx);
        }
    }

    pub fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }

        if let Some(item) = cx.read_from_clipboard() {
            if let Some(text) = item.text() {
                if let Some(selection) = self.selection.take() {
                    self.delete_selection(selection, cx);
                }
                self.insert_text(&text, cx);
            }
        }
    }

    fn insert_text(&mut self, text: &str, cx: &mut Context<Self>) {
        if text.contains('\n') {
            let lines: Vec<&str> = text.split('\n').collect();
            let col = self.cursor.col;
            let current = self.current_line_mut();
            let remainder = current.split_off(col);
            current.push_str(lines[0]);

            for (i, line) in lines.iter().enumerate().skip(1) {
                self.cursor.line += 1;
                if i == lines.len() - 1 {
                    self.lines.insert(self.cursor.line, format!("{}{}", line, remainder));
                    self.cursor.col = line.len();
                } else {
                    self.lines.insert(self.cursor.line, line.to_string());
                }
            }
        } else {
            let col = self.cursor.col;
            self.current_line_mut().insert_str(col, text);
            self.cursor.col += text.len();
        }
        self.update_syntax_tree();
        if text.contains('\n') {
            self.ensure_cursor_visible_after_newline(cx);
        } else {
            cx.notify();
        }
    }

    /// Ensure the caret is visible after creating a new line by scrolling minimally.
    fn ensure_cursor_visible_after_newline(&mut self, cx: &mut Context<Self>) {
        let line_height = px(20.0);
        let padding_top = px(12.0);

        let viewport_bounds = self.scroll_handle.bounds();
        let viewport_height = viewport_bounds.size.height;

        let offset = self.scroll_handle.offset();
        let mut new_offset_y = offset.y;

        let cursor_y = padding_top + line_height * (self.cursor.line as f32);

        let current_top = -offset.y;
        let current_bottom = current_top + viewport_height;

        if cursor_y + line_height > current_bottom {
            let target_top = cursor_y + line_height - viewport_height;
            new_offset_y = -target_top;
        }

        let max_offset = self.scroll_handle.max_offset().height;
        if new_offset_y < -max_offset { new_offset_y = -max_offset; }
        if new_offset_y > px(0.0) { new_offset_y = px(0.0); }

        if (new_offset_y - offset.y).abs() > px(0.0) {
            self.scroll_handle.set_offset(point(offset.x, new_offset_y));
        }
        cx.notify();
    }

    /// Ensure the cursor stays visible during mouse selection by scrolling if needed.
    fn ensure_cursor_visible_during_selection(&mut self, cx: &mut Context<Self>) {
        let line_height = px(20.0);
        let padding_top = px(12.0);

        let viewport_bounds = self.scroll_handle.bounds();
        let viewport_height = viewport_bounds.size.height;

        let offset = self.scroll_handle.offset();
        let mut new_offset_y = offset.y;

        let cursor_y = padding_top + line_height * (self.cursor.line as f32);

        let current_top = -offset.y;
        let current_bottom = current_top + viewport_height;

        if cursor_y < current_top {
            new_offset_y = -cursor_y;
        }
        else if cursor_y + line_height > current_bottom {
            let target_top = cursor_y + line_height - viewport_height;
            new_offset_y = -target_top;
        }

        let max_offset = self.scroll_handle.max_offset().height;
        if new_offset_y < -max_offset { new_offset_y = -max_offset; }
        if new_offset_y > px(0.0) { new_offset_y = px(0.0); }

        if (new_offset_y - offset.y).abs() > px(0.0) {
            self.scroll_handle.set_offset(point(offset.x, new_offset_y));
            cx.notify();
        }
    }

    fn delete_selection(&mut self, selection: Selection, cx: &mut Context<Self>) {
        let (start, end) = selection.range();

        if start.line == end.line {
            self.lines[start.line].replace_range(start.col..end.col, "");
            self.cursor = start;
        } else {
            let remainder = self.lines[end.line][end.col..].to_string();
            let start_line = &mut self.lines[start.line];
            start_line.truncate(start.col);
            start_line.push_str(&remainder);
            self.lines.drain(start.line + 1..=end.line);
            self.cursor = start;
        }

        self.update_syntax_tree();
        cx.notify();
    }

    fn get_selection_text(&self, selection: &Selection) -> String {
        let (start, end) = selection.range();

        if start.line == end.line {
            self.lines[start.line][start.col..end.col].to_string()
        } else {
            let mut result = String::new();
            result.push_str(&self.lines[start.line][start.col..]);
            result.push('\n');

            for line_idx in start.line + 1..end.line {
                result.push_str(&self.lines[line_idx]);
                result.push('\n');
            }

            result.push_str(&self.lines[end.line][..end.col]);
            result
        }
    }

    fn position_for_mouse(&self, mouse_pos: Point<Pixels>, bounds: Bounds<Pixels>, gutter_width: Pixels, line_height: Pixels, _font_size: Pixels, _window: &Window) -> Position {
        let padding_top = px(12.0);
        let relative_y = mouse_pos.y - bounds.top() - padding_top;
        let line = self.scroll_offset + (relative_y / line_height).floor() as usize;
        let line = min(line, self.lines.len().saturating_sub(1));

        let relative_x = mouse_pos.x - bounds.left() - gutter_width;
        let col = if let Some(Some(layout)) = self.line_layouts.get(line) {
            let idx = layout.closest_index_for_x(relative_x);
            idx.min(self.lines[line].len())
        } else {
            0
        };

        Position::new(line, col)
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, bounds: Bounds<Pixels>, gutter_width: Pixels, line_height: Pixels, font_size: Pixels, window: &Window, cx: &mut Context<Self>) {
        let pos = self.position_for_mouse(event.position, bounds, gutter_width, line_height, font_size, window);

        let now = std::time::Instant::now();
        let is_double_click = if let Some(last_time) = self.last_click_time {
            now.duration_since(last_time).as_millis() < 500
        } else {
            false
        };
        self.last_click_time = Some(now);

        if is_double_click {
            self.selection = Some(Selection::new(
                Position::new(pos.line, 0),
                Position::new(pos.line, self.lines[pos.line].len()),
            ));
            self.cursor = Position::new(pos.line, self.lines[pos.line].len());
        } else if event.modifiers.shift {
            if let Some(ref mut sel) = self.selection {
                sel.cursor = pos;
                self.cursor = pos;
            } else {
                self.selection = Some(Selection::new(self.cursor, pos));
                self.cursor = pos;
            }
        } else {
            self.cursor = pos;
            self.selection = None;
            self.is_selecting = true;
        }

        cx.notify();
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, bounds: Bounds<Pixels>, gutter_width: Pixels, line_height: Pixels, font_size: Pixels, window: &Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            let pos = self.position_for_mouse(event.position, bounds, gutter_width, line_height, font_size, window);
            if let Some(ref mut sel) = self.selection {
                sel.cursor = pos;
            } else {
                self.selection = Some(Selection::new(self.cursor, pos));
            }
            self.cursor = pos;
            self.ensure_cursor_visible_during_selection(cx);
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.is_selecting = false;
        cx.notify();
    }
}

impl Focusable for EditorState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for EditorState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));

        let start_pos = self.offset_to_position(range.start);
        let end_pos = self.offset_to_position(range.end);

        Some(self.get_selection_text(&Selection::new(start_pos, end_pos)))
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        if let Some(selection) = &self.selection {
            let start_offset = self.position_to_offset(selection.anchor);
            let end_offset = self.position_to_offset(selection.cursor);
            let range = self.range_to_utf16(&(start_offset..end_offset));

            Some(UTF16Selection {
                range,
                reversed: self.selection_reversed,
            })
        } else {
            let cursor_offset = self.position_to_offset(self.cursor);
            let range = self.range_to_utf16(&(cursor_offset..cursor_offset));
            Some(UTF16Selection {
                range,
                reversed: false,
            })
        }
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range.as_ref().map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only {
            return;
        }

        let range_utf8 = range_utf16
            .as_ref()
            .map(|r| self.range_from_utf16(r))
            .or_else(|| self.marked_range.clone())
            .or_else(|| {
                if let Some(sel) = &self.selection {
                    let start = self.position_to_offset(sel.anchor);
                    let end = self.position_to_offset(sel.cursor);
                    Some(start.min(end)..start.max(end))
                } else {
                    let cursor_offset = self.position_to_offset(self.cursor);
                    Some(cursor_offset..cursor_offset)
                }
            });

        if let Some(range) = range_utf8 {
            let start_pos = self.offset_to_position(range.start);
            let end_pos = self.offset_to_position(range.end);

            if start_pos != end_pos {
                self.delete_selection(Selection::new(start_pos, end_pos), cx);
            }

            self.insert_text(new_text, cx);
        }

        self.marked_range = None;
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only {
            return;
        }

        let range_utf8 = range_utf16
            .map(|r| self.range_from_utf16(&r))
            .unwrap_or_else(|| {
                let cursor_offset = self.position_to_offset(self.cursor);
                cursor_offset..cursor_offset
            });

        let start_pos = self.offset_to_position(range_utf8.start);
        let end_pos = self.offset_to_position(range_utf8.end);

        if start_pos != end_pos {
            self.delete_selection(Selection::new(start_pos, end_pos), cx);
        }

        let insert_start = self.position_to_offset(self.cursor);
        self.insert_text(new_text, cx);
        let insert_end = self.position_to_offset(self.cursor);

        if !new_text.is_empty() {
            self.marked_range = Some(insert_start..insert_end);
        }

        if let Some(new_sel_utf16) = new_selected_range_utf16 {
            let new_sel_utf8 = self.range_from_utf16(&new_sel_utf16);
            let sel_start = self.offset_to_position(insert_start + new_sel_utf8.start);
            let sel_end = self.offset_to_position(insert_start + new_sel_utf8.end);
            self.selection = Some(Selection::new(sel_start, sel_end));
            self.cursor = sel_end;
        }

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        self.last_bounds
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        if let Some(bounds) = self.last_bounds {
            let gutter_width = px(60.0);
            let line_height = px(20.0);
            let font_size = px(14.0);
            let pos = self.position_for_mouse(point, bounds, gutter_width, line_height, font_size, window);
            let offset = self.position_to_offset(pos);
            Some(self.offset_to_utf16(offset))
        } else {
            None
        }
    }
}

impl Render for EditorState {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        EditorElement {
            state: cx.entity(),
        }
    }
}

/// Custom element for rendering the editor with tree-sitter highlighting
struct EditorElement {
    state: Entity<EditorState>,
}

struct PrepaintState {
    gutter_width: Pixels,
    line_height: Pixels,
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditorElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // Calculate the total height needed for all lines (for scrolling)
        let line_height = px(20.0);
        let padding_top = px(12.0);
        let padding_bottom = px(12.0);
        let num_lines = self.state.read(cx).lines.len();
        let content_height = padding_top + padding_bottom + (line_height * num_lines as f32);

        let mut layout_style = gpui::Style::default();
        layout_style.size.width = relative(1.).into();
        layout_style.size.height = content_height.into();

        (window.request_layout(layout_style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        PrepaintState {
            gutter_width: px(60.0),
            line_height: px(20.0),
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.state.read(cx).focus_handle.clone();
        let theme = use_theme();

        let padding_top = px(12.0);

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.state.clone()),
            cx,
        );

        self.state.update(cx, |state, _| {
            state.last_bounds = Some(bounds);
        });

        window.paint_quad(fill(bounds, theme.tokens.card));

        let (lines, show_line_numbers, cursor, selection, syntax_tree) = {
            let state = self.state.read(cx);
            (
                state.lines.clone(),
                state.show_line_numbers,
                state.cursor,
                state.selection,
                state.syntax_tree.clone(),
            )
        };

        let text_style = window.text_style();
        let line_height = prepaint.line_height;
        let gutter_width = prepaint.gutter_width;
        let font_size = px(14.0);

        let mut highlights: Vec<(usize, usize, String)> = Vec::new();
        if let Some(tree) = syntax_tree {
            let root = tree.root_node();
            collect_highlights(root, &mut highlights);
        }

        let mut shaped_layouts: Vec<Option<ShapedLine>> = Vec::with_capacity(lines.len());

        for (idx, line) in lines.iter().enumerate() {
            let y = bounds.top() + padding_top + line_height * idx as f32;

            if show_line_numbers {
                let line_num_text = format!("{:>3}", idx + 1);
                let line_num_run = TextRun {
                    len: line_num_text.len(),
                    font: text_style.font(),
                    color: hsla(0.0, 0.0, 0.5, 1.0),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };

                let shaped = window.text_system().shape_line(
                    line_num_text.into(),
                    font_size,
                    &[line_num_run],
                    None,
                );

                let _ = shaped.paint(
                    point(bounds.left() + px(10.0), y),
                    line_height,
                    window,
                    cx,
                );
            }

            if line.is_empty() {
                shaped_layouts.push(None);
            } else {
                let plain_run = TextRun {
                    len: line.len(),
                    font: text_style.font(),
                    color: hsla(0.0, 0.0, 1.0, 1.0),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let shaped_plain = window.text_system().shape_line(
                    line.clone().into(),
                    font_size,
                    &[plain_run],
                    None,
                );
                shaped_layouts.push(Some(shaped_plain));
            }

            if !line.is_empty() {
                let line_start_offset: usize = lines[..idx].iter().map(|l| l.len() + 1).sum();
                let line_end_offset = line_start_offset + line.len();

                let mut line_highlights: Vec<(usize, usize, String)> = highlights
                    .iter()
                    .filter(|(start, end, _)| *start < line_end_offset && *end > line_start_offset)
                    .map(|(start, end, name)| {
                        let rel_start = start.saturating_sub(line_start_offset);
                        let rel_end = (end.saturating_sub(line_start_offset)).min(line.len());
                        (rel_start, rel_end, name.clone())
                    })
                    .collect();

                line_highlights.sort_by_key(|(start, _, _)| *start);

                let mut text_runs = Vec::new();
                let mut current_pos = 0;

                for (start, end, capture_name) in line_highlights {
                    if current_pos < start {
                        text_runs.push(TextRun {
                            len: start - current_pos,
                            font: text_style.font(),
                            color: theme.tokens.foreground,
                            background_color: None,
                            underline: None,
                            strikethrough: None,
                        });
                    }

                    let color = get_highlight_color(&capture_name, &theme);
                    text_runs.push(TextRun {
                        len: end - start,
                        font: text_style.font(),
                        color: color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    });

                    current_pos = end;
                }

                if current_pos < line.len() {
                    text_runs.push(TextRun {
                        len: line.len() - current_pos,
                        font: text_style.font(),
                        color: theme.tokens.foreground,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    });
                }

                if text_runs.is_empty() {
                    text_runs.push(TextRun {
                        len: line.len(),
                        font: text_style.font(),
                        color: theme.tokens.foreground,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    });
                }

                let shaped = window.text_system().shape_line(
                    line.clone().into(),
                    font_size,
                    &text_runs,
                    None,
                );

                let _ = shaped.paint(
                    point(bounds.left() + gutter_width, y),
                    line_height,
                    window,
                    cx,
                );
            }
        }

        self.state.update(cx, |state, _| {
            state.line_layouts = shaped_layouts;
        });

        if let Some(selection) = &selection {
            let (start, end) = selection.range();

            for line_idx in start.line..=end.line {
                let line_y = bounds.top() + padding_top + line_height * line_idx as f32;
                let start_col = if line_idx == start.line { start.col } else { 0 };
                let end_col = if line_idx == end.line {
                    end.col
                } else {
                    lines[line_idx].len()
                };

                let (sel_x, sel_width) = if let Some(Some(layout)) = self.state.read(cx).line_layouts.get(line_idx) {
                    let x_start = layout.x_for_index(start_col);
                    let x_end = layout.x_for_index(end_col);
                    (bounds.left() + gutter_width + x_start, (x_end - x_start))
                } else {
                    (bounds.left() + gutter_width, px(0.0))
                };

                window.paint_quad(fill(
                    Bounds::new(
                        point(sel_x, line_y),
                        size(sel_width, line_height),
                    ),
                    rgba(0x4444ff40),
                ));
            }
        }

        if focus_handle.is_focused(window) {
            let cursor_col = if cursor.line < lines.len() {
                cursor.col.min(lines[cursor.line].len())
            } else {
                0
            };

            let cursor_y = bounds.top() + padding_top + line_height * cursor.line as f32;

            let cursor_x = if let Some(Some(layout)) = self.state.read(cx).line_layouts.get(cursor.line) {
                bounds.left() + gutter_width + layout.x_for_index(cursor_col)
            } else {
                bounds.left() + gutter_width
            };

            window.paint_quad(fill(
                Bounds::new(
                    point(cursor_x, cursor_y),
                    size(px(2.0), line_height),
                ),
                rgb(0x0099ff),
            ));
        }
    }
}

/// Editor component wrapper
#[derive(IntoElement)]
pub struct Editor {
    state: Entity<EditorState>,
    min_lines: Option<usize>,
    max_lines: Option<usize>,
    show_border: bool,
    style: StyleRefinement,
}

impl Editor {
    pub fn new(state: &Entity<EditorState>) -> Self {
        Self {
            state: state.clone(),
            min_lines: None,
            max_lines: None,
            show_border: true,
            style: StyleRefinement::default(),
        }
    }

    /// Set initial content
    pub fn content(self, content: impl Into<String>, cx: &mut App) -> Self {
        self.state.update(cx, |state, cx| {
            state.set_content(&content.into(), cx);
        });
        self
    }

    /// Set minimum number of visible lines
    pub fn min_lines(mut self, lines: usize) -> Self {
        self.min_lines = Some(lines);
        self
    }

    /// Set maximum number of visible lines
    pub fn max_lines(mut self, lines: usize) -> Self {
        self.max_lines = Some(lines);
        self
    }

    /// Show or hide the border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Show or hide line numbers
    pub fn show_line_numbers(self, show: bool, cx: &mut App) -> Self {
        self.state.update(cx, |state, cx| {
            state.show_line_numbers = show;
            cx.notify();
        });
        self
    }

    /// Get current content
    pub fn get_content(&self, cx: &App) -> String {
        self.state.read(cx).content()
    }
}

impl Styled for Editor {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Editor {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let min_height = self.min_lines.map(|lines| px(lines as f32 * 20.0));
        let max_height = self.max_lines.map(|lines| px(lines as f32 * 20.0));

        let mut base = div()
            .id(("editor", self.state.entity_id()))
            .key_context("Editor")
            .track_focus(&self.state.read(cx).focus_handle(cx))
            .w_full()
            .h_full()
            .max_h_full();

        if let Some(h) = min_height {
            base = base.min_h(h);
        }
        if let Some(h) = max_height {
            base = base.max_h(h);
        }

        let styled_base = base.bg(theme.tokens.background)
            .rounded(theme.tokens.radius_md);

        let final_base = if self.show_border {
            styled_base.border_1().border_color(theme.tokens.border)
        } else {
            styled_base
        };

        let user_style = self.style;

        final_base
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .font_family(theme.tokens.font_mono.clone())
            .on_action(window.listener_for(&self.state, EditorState::move_up))
            .on_action(window.listener_for(&self.state, EditorState::move_down))
            .on_action(window.listener_for(&self.state, EditorState::move_left))
            .on_action(window.listener_for(&self.state, EditorState::move_right))
            .on_action(window.listener_for(&self.state, EditorState::move_to_line_start))
            .on_action(window.listener_for(&self.state, EditorState::move_to_line_end))
            .on_action(window.listener_for(&self.state, EditorState::move_to_doc_start))
            .on_action(window.listener_for(&self.state, EditorState::move_to_doc_end))
            .on_action(window.listener_for(&self.state, EditorState::page_up))
            .on_action(window.listener_for(&self.state, EditorState::page_down))
            .on_action(window.listener_for(&self.state, EditorState::select_up))
            .on_action(window.listener_for(&self.state, EditorState::select_down))
            .on_action(window.listener_for(&self.state, EditorState::select_left))
            .on_action(window.listener_for(&self.state, EditorState::select_right))
            .on_action(window.listener_for(&self.state, EditorState::select_to_line_start))
            .on_action(window.listener_for(&self.state, EditorState::select_to_line_end))
            .on_action(window.listener_for(&self.state, EditorState::select_all))
            .on_action(window.listener_for(&self.state, EditorState::backspace))
            .on_action(window.listener_for(&self.state, EditorState::delete))
            .on_action(window.listener_for(&self.state, EditorState::enter))
            .on_action(window.listener_for(&self.state, EditorState::tab))
            .on_action(window.listener_for(&self.state, EditorState::copy))
            .on_action(window.listener_for(&self.state, EditorState::cut))
            .on_action(window.listener_for(&self.state, EditorState::paste))
            .on_mouse_down(MouseButton::Left, {
                let state = self.state.clone();
                move |event: &MouseDownEvent, window: &mut Window, cx: &mut App| {
                    let bounds = state.read(cx).last_bounds.unwrap_or_default();
                    let gutter_width = px(60.0);
                    let line_height = px(20.0);
                    let font_size = px(14.0);
                    state.update(cx, |s, cx| {
                        s.on_mouse_down(event, bounds, gutter_width, line_height, font_size, window, cx);
                    });
                    window.focus(&state.read(cx).focus_handle(cx));
                }
            })
            .on_mouse_move({
                let state = self.state.clone();
                move |event: &MouseMoveEvent, window: &mut Window, cx: &mut App| {
                    let bounds = state.read(cx).last_bounds.unwrap_or_default();
                    let gutter_width = px(60.0);
                    let line_height = px(20.0);
                    let font_size = px(14.0);
                    state.update(cx, |s, cx| {
                        s.on_mouse_move(event, bounds, gutter_width, line_height, font_size, window, cx);
                    });
                }
            })
            .on_mouse_up(MouseButton::Left, window.listener_for(&self.state, EditorState::on_mouse_up))
            .child(scrollable_vertical(self.state.clone()).with_scroll_handle(self.state.read(cx).scroll_handle.clone()))
    }
}
