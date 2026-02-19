use crate::components::scrollable::scrollable_vertical;
use crate::icon_config::resolve_icon_path;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use regex::Regex;
use ropey::Rope;
use smol::Timer;
use std::cmp::min;
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use tree_sitter::{
    InputEdit, Parser, Point as TSPoint, Query, QueryCursor, StreamingIterator, Tree,
};

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
        MoveWordLeft,
        MoveWordRight,
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
        DeleteWord,
        Enter,
        Tab,
        Copy,
        Cut,
        Paste,
        Undo,
        Redo,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", MoveUp, Some("Editor")),
        KeyBinding::new("down", MoveDown, Some("Editor")),
        KeyBinding::new("left", MoveLeft, Some("Editor")),
        KeyBinding::new("right", MoveRight, Some("Editor")),
        KeyBinding::new("home", MoveToLineStart, Some("Editor")),
        KeyBinding::new("end", MoveToLineEnd, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-left", MoveWordLeft, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-left", MoveWordLeft, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-right", MoveWordRight, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-right", MoveWordRight, Some("Editor")),
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
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-backspace", DeleteWord, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-backspace", DeleteWord, Some("Editor")),
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
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-z", Undo, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-z", Undo, Some("Editor")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-z", Redo, Some("Editor")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-z", Redo, Some("Editor")),
    ]);
}

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

    pub fn range(&self) -> (Position, Position) {
        if self.anchor <= self.cursor {
            (self.anchor, self.cursor)
        } else {
            (self.cursor, self.anchor)
        }
    }
}

#[derive(Debug, Clone)]
enum EditOp {
    Insert { byte_offset: usize, text: String },
    Delete { byte_offset: usize, text: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldRange {
    pub start_line: usize,
    pub end_line: usize,
}

const AUTO_CLOSE_PAIRS: &[(char, char)] = &[
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('"', '"'),
    ('\'', '\''),
    ('`', '`'),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Json,
    Toml,
    Markdown,
    Go,
    C,
    Cpp,
    Java,
    Ruby,
    Bash,
    Css,
    Html,
    Yaml,
    Lua,
    Zig,
    Scala,
    Php,
    OCaml,
    Sql,
    Plain,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" => Language::TypeScript,
            "py" | "pyi" => Language::Python,
            "json" | "jsonc" => Language::Json,
            "toml" => Language::Toml,
            "md" | "markdown" => Language::Markdown,
            "go" => Language::Go,
            "c" | "h" => Language::C,
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => Language::Cpp,
            "java" => Language::Java,
            "rb" | "rake" | "gemspec" => Language::Ruby,
            "sh" | "bash" | "zsh" => Language::Bash,
            "css" => Language::Css,
            "html" | "htm" => Language::Html,
            "yml" | "yaml" => Language::Yaml,
            "lua" => Language::Lua,
            "zig" => Language::Zig,
            "scala" | "sc" => Language::Scala,
            "php" => Language::Php,
            "ml" | "mli" => Language::OCaml,
            "sql" => Language::Sql,
            _ => Language::Plain,
        }
    }

    pub fn from_path(path: &std::path::Path) -> Self {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(Self::from_extension)
            .unwrap_or(Language::Plain)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Python => "Python",
            Language::Json => "JSON",
            Language::Toml => "TOML",
            Language::Markdown => "Markdown",
            Language::Go => "Go",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::Java => "Java",
            Language::Ruby => "Ruby",
            Language::Bash => "Shell",
            Language::Css => "CSS",
            Language::Html => "HTML",
            Language::Yaml => "YAML",
            Language::Lua => "Lua",
            Language::Zig => "Zig",
            Language::Scala => "Scala",
            Language::Php => "PHP",
            Language::OCaml => "OCaml",
            Language::Sql => "SQL",
            Language::Plain => "Plain Text",
        }
    }

    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            #[cfg(feature = "tree-sitter-rust")]
            Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-javascript")]
            Language::JavaScript => Some(tree_sitter_javascript::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-typescript")]
            Language::TypeScript => Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
            #[cfg(all(
                feature = "tree-sitter-javascript",
                not(feature = "tree-sitter-typescript")
            ))]
            Language::TypeScript => Some(tree_sitter_javascript::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-python")]
            Language::Python => Some(tree_sitter_python::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-json")]
            Language::Json => Some(tree_sitter_json::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-toml-ng")]
            Language::Toml => Some(tree_sitter_toml_ng::language()),
            #[cfg(feature = "tree-sitter-md")]
            Language::Markdown => Some(tree_sitter_md::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-go")]
            Language::Go => Some(tree_sitter_go::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-c")]
            Language::C => Some(tree_sitter_c::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-cpp")]
            Language::Cpp => Some(tree_sitter_cpp::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-java")]
            Language::Java => Some(tree_sitter_java::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-ruby")]
            Language::Ruby => Some(tree_sitter_ruby::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-bash")]
            Language::Bash => Some(tree_sitter_bash::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-css")]
            Language::Css => Some(tree_sitter_css::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-html")]
            Language::Html => Some(tree_sitter_html::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-yaml")]
            Language::Yaml => Some(tree_sitter_yaml::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-lua")]
            Language::Lua => Some(tree_sitter_lua::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-zig")]
            Language::Zig => Some(tree_sitter_zig::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-scala")]
            Language::Scala => Some(tree_sitter_scala::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-php")]
            Language::Php => Some(tree_sitter_php::LANGUAGE_PHP.into()),
            #[cfg(feature = "tree-sitter-ocaml")]
            Language::OCaml => Some(tree_sitter_ocaml::LANGUAGE_OCAML.into()),
            #[cfg(feature = "tree-sitter-sequel")]
            Language::Sql => Some(tree_sitter_sequel::LANGUAGE.into()),
            _ => None,
        }
    }

    pub fn highlight_query_source(&self) -> Option<&'static str> {
        match self {
            #[cfg(feature = "tree-sitter-rust")]
            Language::Rust => Some(tree_sitter_rust::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-javascript")]
            Language::JavaScript => Some(tree_sitter_javascript::HIGHLIGHT_QUERY),
            #[cfg(feature = "tree-sitter-typescript")]
            Language::TypeScript => Some(tree_sitter_typescript::HIGHLIGHTS_QUERY),
            #[cfg(all(
                feature = "tree-sitter-javascript",
                not(feature = "tree-sitter-typescript")
            ))]
            Language::TypeScript => Some(tree_sitter_javascript::HIGHLIGHT_QUERY),
            #[cfg(feature = "tree-sitter-python")]
            Language::Python => Some(tree_sitter_python::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-json")]
            Language::Json => Some(tree_sitter_json::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-toml-ng")]
            Language::Toml => Some(tree_sitter_toml_ng::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-md")]
            Language::Markdown => Some(tree_sitter_md::HIGHLIGHT_QUERY_BLOCK),
            #[cfg(feature = "tree-sitter-go")]
            Language::Go => Some(tree_sitter_go::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-c")]
            Language::C => Some(tree_sitter_c::HIGHLIGHT_QUERY),
            #[cfg(feature = "tree-sitter-cpp")]
            Language::Cpp => Some(tree_sitter_cpp::HIGHLIGHT_QUERY),
            #[cfg(feature = "tree-sitter-java")]
            Language::Java => Some(tree_sitter_java::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-ruby")]
            Language::Ruby => Some(tree_sitter_ruby::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-bash")]
            Language::Bash => Some(tree_sitter_bash::HIGHLIGHT_QUERY),
            #[cfg(feature = "tree-sitter-css")]
            Language::Css => Some(tree_sitter_css::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-html")]
            Language::Html => Some(tree_sitter_html::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-yaml")]
            Language::Yaml => Some(tree_sitter_yaml::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-lua")]
            Language::Lua => Some(tree_sitter_lua::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-zig")]
            Language::Zig => Some(tree_sitter_zig::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-scala")]
            Language::Scala => Some(tree_sitter_scala::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-php")]
            Language::Php => Some(tree_sitter_php::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-ocaml")]
            Language::OCaml => Some(tree_sitter_ocaml::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-sequel")]
            Language::Sql => Some(tree_sitter_sequel::HIGHLIGHTS_QUERY),
            _ => None,
        }
    }
}

pub fn highlight_color_for_capture(capture_name: &str) -> Hsla {
    match capture_name {
        "keyword"
        | "keyword.control"
        | "keyword.operator"
        | "keyword.function"
        | "keyword.return"
        | "keyword.control.repeat"
        | "keyword.control.conditional"
        | "keyword.control.import"
        | "keyword.control.exception"
        | "keyword.directive"
        | "keyword.modifier"
        | "keyword.type"
        | "keyword.coroutine"
        | "keyword.storage.type"
        | "keyword.storage.modifier"
        | "conditional"
        | "repeat"
        | "include"
        | "exception" => hsla(0.77, 0.75, 0.70, 1.0),

        "type" | "type.builtin" | "type.definition" | "type.qualifier" | "storageclass"
        | "structure" => hsla(0.47, 0.60, 0.65, 1.0),

        "function" | "function.call" | "function.method" | "function.builtin"
        | "function.macro" | "method" | "method.call" | "constructor" => {
            hsla(0.58, 0.65, 0.70, 1.0)
        }

        "string"
        | "string.special"
        | "string.escape"
        | "string.regex"
        | "string.special.url"
        | "string.special.path"
        | "character"
        | "character.special" => hsla(0.25, 0.55, 0.60, 1.0),

        "number" | "float" | "constant.numeric" => hsla(0.08, 0.75, 0.65, 1.0),

        "comment" | "comment.line" | "comment.block" | "comment.documentation" => {
            hsla(0.0, 0.0, 0.45, 1.0)
        }

        "operator" => hsla(0.55, 0.50, 0.70, 1.0),

        "variable" | "variable.parameter" | "variable.builtin" | "variable.member"
        | "parameter" | "field" => hsla(0.0, 0.0, 0.85, 1.0),

        "constant" | "constant.builtin" | "constant.macro" | "boolean" | "define" | "symbol" => {
            hsla(0.08, 0.75, 0.65, 1.0)
        }

        "property" | "property.definition" => hsla(0.55, 0.50, 0.70, 1.0),

        "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => {
            hsla(0.0, 0.0, 0.60, 1.0)
        }

        "attribute" | "label" | "annotation" | "decorator" => hsla(0.12, 0.60, 0.65, 1.0),

        "namespace" | "module" => hsla(0.08, 0.50, 0.70, 1.0),

        "tag" | "tag.builtin" | "tag.delimiter" | "tag.attribute" => hsla(0.0, 0.65, 0.65, 1.0),

        "text.title" | "markup.heading" | "text.strong" | "markup.bold" => {
            hsla(0.58, 0.65, 0.80, 1.0)
        }
        "text.emphasis" | "markup.italic" => hsla(0.25, 0.55, 0.70, 1.0),
        "text.uri" | "markup.link.url" | "markup.link" => hsla(0.55, 0.60, 0.65, 1.0),
        "text.literal" | "markup.raw" => hsla(0.25, 0.55, 0.60, 1.0),

        "embedded" | "injection.content" => hsla(0.0, 0.0, 0.80, 1.0),

        _ => hsla(0.0, 0.0, 0.85, 1.0),
    }
}

pub struct EditorState {
    focus_handle: FocusHandle,
    rope: Rope,
    cursor: Position,
    selection: Option<Selection>,

    undo_stack: Vec<EditOp>,
    redo_stack: Vec<EditOp>,

    file_path: Option<PathBuf>,
    is_modified: bool,
    content_version: u64,

    parser: Parser,
    syntax_tree: Option<Tree>,
    highlight_query: Option<Query>,
    language: Language,

    scroll_handle: ScrollHandle,
    scroll_offset_x: Pixels,
    max_line_width: Pixels,
    line_layouts: HashMap<usize, ShapedLine>,
    line_content_hashes: HashMap<usize, u64>,
    cached_highlight_spans: Vec<HighlightSpan>,
    highlight_cache_version: u64,
    highlight_cache_first_line: usize,
    highlight_cache_last_line: usize,
    last_bounds: Option<Bounds<Pixels>>,

    is_selecting: bool,
    dragging_h_scrollbar: bool,
    last_mouse_pos: Option<Point<Pixels>>,
    last_mouse_gutter_width: Pixels,
    autoscroll_task: Option<Task<()>>,
    last_click_time: Option<std::time::Instant>,

    marked_range: Option<Range<usize>>,

    pub show_line_numbers: bool,
    tab_size: usize,
    read_only: bool,

    pub font_size: Pixels,
    pub line_height: Pixels,
    pub font_family_override: Option<SharedString>,

    cursor_visible: bool,
    blink_task: Option<Task<()>>,
    last_cursor_move: std::time::Instant,
    last_blink_cursor: Position,

    overlay_active_check: Option<Box<dyn Fn(&App) -> bool + 'static>>,

    reparse_task: Option<Task<()>>,
    search_task: Option<Task<()>>,

    search_query: String,
    search_matches: Vec<(usize, usize)>,
    current_match_idx: Option<usize>,
    search_case_sensitive: bool,
    search_use_regex: bool,

    pub cursor_color_override: Option<Hsla>,
    pub selection_color_override: Option<Hsla>,
    pub line_number_color_override: Option<Hsla>,
    pub line_number_active_color_override: Option<Hsla>,
    pub gutter_bg_override: Option<Hsla>,
    pub search_match_color_overrides: Option<(Hsla, Hsla)>,
    pub current_line_color_override: Option<Hsla>,
    pub bracket_match_color_override: Option<Hsla>,
    pub word_highlight_color_override: Option<Hsla>,
    pub indent_guide_color_override: Option<Hsla>,
    pub indent_guide_active_color_override: Option<Hsla>,
    pub fold_marker_color_override: Option<Hsla>,
    pub diagnostic_error_color: Option<Hsla>,
    pub diagnostic_warning_color: Option<Hsla>,
    pub diagnostic_info_color: Option<Hsla>,
    pub diagnostic_hint_color: Option<Hsla>,
    pub syntax_color_fn: Option<Box<dyn Fn(&str) -> Hsla>>,

    fold_ranges: Vec<FoldRange>,
    folded: Vec<FoldRange>,
    cached_display_lines: Option<Rc<Vec<usize>>>,

    diagnostics: Vec<EditorDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct EditorDiagnostic {
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub severity: DiagnosticSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

impl EditorState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let parser = Parser::new();

        Self {
            focus_handle: cx.focus_handle(),
            rope: Rope::from_str("\n"),
            cursor: Position::zero(),
            selection: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            file_path: None,
            is_modified: false,
            content_version: 0,
            parser,
            syntax_tree: None,
            highlight_query: None,
            language: Language::Plain,
            scroll_handle: ScrollHandle::new(),
            scroll_offset_x: px(0.0),
            max_line_width: px(0.0),
            line_layouts: HashMap::new(),
            line_content_hashes: HashMap::new(),
            cached_highlight_spans: Vec::new(),
            highlight_cache_version: u64::MAX,
            highlight_cache_first_line: 0,
            highlight_cache_last_line: 0,
            last_bounds: None,
            is_selecting: false,
            dragging_h_scrollbar: false,
            last_mouse_pos: None,
            last_mouse_gutter_width: px(80.0),
            autoscroll_task: None,
            last_click_time: None,
            marked_range: None,
            show_line_numbers: true,
            tab_size: 4,
            read_only: false,
            font_size: px(14.0),
            line_height: px(20.0),
            font_family_override: None,
            cursor_visible: true,
            blink_task: None,
            last_cursor_move: std::time::Instant::now(),
            last_blink_cursor: Position::zero(),
            overlay_active_check: None,
            reparse_task: None,
            search_task: None,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match_idx: None,
            search_case_sensitive: false,
            search_use_regex: false,
            cursor_color_override: None,
            selection_color_override: None,
            line_number_color_override: None,
            line_number_active_color_override: None,
            gutter_bg_override: None,
            search_match_color_overrides: None,
            current_line_color_override: None,
            bracket_match_color_override: None,
            word_highlight_color_override: None,
            indent_guide_color_override: None,
            indent_guide_active_color_override: None,
            fold_marker_color_override: None,
            diagnostic_error_color: None,
            diagnostic_warning_color: None,
            diagnostic_info_color: None,
            diagnostic_hint_color: None,
            syntax_color_fn: None,
            fold_ranges: Vec::new(),
            folded: Vec::new(),
            cached_display_lines: None,
            diagnostics: Vec::new(),
        }
    }

    pub fn set_cursor_position(&mut self, line: usize, col: usize, cx: &mut Context<Self>) {
        let max_line = self.total_lines().saturating_sub(1);
        self.cursor.line = line.min(max_line);
        let line_len = self.line_len(self.cursor.line);
        self.cursor.col = col.min(line_len);
        self.selection = None;
        self.reset_cursor_blink(cx);
        self.ensure_cursor_visible(cx);
    }

    pub fn set_font_size(&mut self, size: f32, cx: &mut Context<Self>) {
        self.font_size = px(size);
        self.line_height = px((size * 1.5).round());
        self.line_layouts.clear();
        self.line_content_hashes.clear();
        cx.notify();
    }

    pub fn set_font_family(&mut self, family: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.font_family_override = Some(family.into());
        self.line_layouts.clear();
        self.line_content_hashes.clear();
        cx.notify();
    }

    fn reset_cursor_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self.last_cursor_move = std::time::Instant::now();
        self.blink_task = Some(cx.spawn(async |this, cx| {
            loop {
                smol::Timer::after(std::time::Duration::from_millis(500)).await;
                let ok = this
                    .update(cx, |state, cx| {
                        state.cursor_visible = !state.cursor_visible;
                        cx.notify();
                    })
                    .is_ok();
                if !ok {
                    break;
                }
            }
        }));
    }

    pub fn set_diagnostics(&mut self, diagnostics: Vec<EditorDiagnostic>, cx: &mut Context<Self>) {
        self.diagnostics = diagnostics;
        cx.notify();
    }

    pub fn diagnostics(&self) -> &[EditorDiagnostic] {
        &self.diagnostics
    }

    pub fn diagnostics_at_line(&self, line: usize) -> Vec<&EditorDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.start_line as usize <= line && line <= d.end_line as usize)
            .collect()
    }

    pub fn content(&self) -> String {
        self.rope.to_string()
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0 || (self.rope.len_bytes() == 1 && self.rope.len_lines() <= 1)
    }

    pub fn line_count(&self) -> usize {
        let lines = self.rope.len_lines();
        if lines > 0 && self.rope.len_bytes() > 0 {
            let last_line = self.rope.line(lines - 1);
            if last_line.len_bytes() == 0 {
                return lines.saturating_sub(1).max(1);
            }
        }
        lines.max(1)
    }

    pub fn cursor(&self) -> Position {
        self.cursor
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn syntax_tree(&self) -> Option<&Tree> {
        self.syntax_tree.as_ref()
    }

    pub fn word_at_cursor(&self) -> Option<(String, usize)> {
        let line_text = self.line_text(self.cursor.line);
        if line_text.is_empty() || self.cursor.col == 0 {
            return None;
        }

        let bytes = line_text.as_bytes();
        let col = self.cursor.col.min(bytes.len());

        let mut word_start = col;
        while word_start > 0 {
            let ch = bytes[word_start - 1];
            if !ch.is_ascii_alphanumeric() && ch != b'_' {
                break;
            }
            word_start -= 1;
        }

        if word_start == col {
            return None;
        }

        let word = line_text[word_start..col].to_string();
        Some((word, word_start))
    }

    pub fn find_matching_bracket(&self) -> Option<(Position, Position)> {
        let line_text = self.line_text(self.cursor.line);
        let col = self.cursor.col.min(line_text.len());
        let bytes = line_text.as_bytes();

        let check_positions: &[usize] = if col > 0 { &[col, col - 1] } else { &[col] };

        for &check_col in check_positions {
            if check_col >= bytes.len() {
                continue;
            }
            let ch = bytes[check_col] as char;
            let (opener, closer, forward) = match ch {
                '(' => ('(', ')', true),
                '[' => ('[', ']', true),
                '{' => ('{', '}', true),
                ')' => ('(', ')', false),
                ']' => ('[', ']', false),
                '}' => ('{', '}', false),
                _ => continue,
            };

            let start_pos = Position::new(self.cursor.line, check_col);

            if forward {
                let mut depth = 1i32;
                let mut scan_line = self.cursor.line;
                let mut scan_col = check_col + 1;
                let total = self.total_lines();
                while scan_line < total {
                    let scan_text = self.line_text(scan_line);
                    let scan_bytes = scan_text.as_bytes();
                    while scan_col < scan_bytes.len() {
                        let sc = scan_bytes[scan_col] as char;
                        if sc == opener {
                            depth += 1;
                        } else if sc == closer {
                            depth -= 1;
                            if depth == 0 {
                                return Some((start_pos, Position::new(scan_line, scan_col)));
                            }
                        }
                        scan_col += 1;
                    }
                    scan_line += 1;
                    scan_col = 0;
                }
            } else {
                let mut depth = 1i32;
                let mut scan_line = self.cursor.line;
                let mut scan_col = check_col as i64 - 1;
                loop {
                    if scan_col < 0 {
                        if scan_line == 0 {
                            break;
                        }
                        scan_line -= 1;
                        let prev_text = self.line_text(scan_line);
                        scan_col = prev_text.len() as i64 - 1;
                        continue;
                    }
                    let scan_text = self.line_text(scan_line);
                    let scan_bytes = scan_text.as_bytes();
                    if (scan_col as usize) < scan_bytes.len() {
                        let sc = scan_bytes[scan_col as usize] as char;
                        if sc == closer {
                            depth += 1;
                        } else if sc == opener {
                            depth -= 1;
                            if depth == 0 {
                                return Some((
                                    Position::new(scan_line, scan_col as usize),
                                    start_pos,
                                ));
                            }
                        }
                    }
                    scan_col -= 1;
                }
            }
        }
        None
    }

    pub fn word_under_cursor_full(&self) -> Option<(String, usize, usize)> {
        let line_text = self.line_text(self.cursor.line);
        if line_text.is_empty() {
            return None;
        }
        let bytes = line_text.as_bytes();
        let col = self.cursor.col.min(bytes.len());

        let mut word_start = col;
        while word_start > 0
            && (bytes[word_start - 1].is_ascii_alphanumeric() || bytes[word_start - 1] == b'_')
        {
            word_start -= 1;
        }

        let mut word_end = col;
        while word_end < bytes.len()
            && (bytes[word_end].is_ascii_alphanumeric() || bytes[word_end] == b'_')
        {
            word_end += 1;
        }

        if word_start == word_end {
            return None;
        }

        let word = line_text[word_start..word_end].to_string();
        if word.len() < 2 {
            return None;
        }
        Some((word, word_start, word_end))
    }

    pub fn compute_fold_ranges(&mut self) {
        let tree = match &self.syntax_tree {
            Some(t) => t,
            None => {
                self.fold_ranges.clear();
                return;
            }
        };

        let mut ranges = Vec::new();
        let mut tree_cursor = tree.root_node().walk();
        let mut did_enter = true;

        loop {
            let node = tree_cursor.node();
            if did_enter {
                let kind = node.kind();
                let start_line = node.start_position().row;
                let end_line = node.end_position().row;
                if end_line > start_line + 1 && Self::is_foldable_kind(kind) {
                    ranges.push(FoldRange {
                        start_line,
                        end_line,
                    });
                }
            }

            if did_enter && tree_cursor.goto_first_child() {
                did_enter = true;
            } else if tree_cursor.goto_next_sibling() {
                did_enter = true;
            } else if tree_cursor.goto_parent() {
                did_enter = false;
            } else {
                break;
            }
        }

        ranges.sort_by_key(|r| r.start_line);
        ranges.dedup_by_key(|r| r.start_line);
        self.fold_ranges = ranges;

        self.folded.retain(|f| {
            self.fold_ranges
                .iter()
                .any(|r| r.start_line == f.start_line)
        });

        if self.folded.is_empty() {
            self.cached_display_lines = None;
        } else {
            self.cached_display_lines = Some(Rc::new(self.compute_display_lines()));
        }
    }

    fn is_foldable_kind(kind: &str) -> bool {
        matches!(
            kind,
            "function_item"
                | "impl_item"
                | "struct_item"
                | "enum_item"
                | "block"
                | "if_expression"
                | "match_expression"
                | "function_declaration"
                | "class_declaration"
                | "class_definition"
                | "method_definition"
                | "if_statement"
                | "for_statement"
                | "while_statement"
                | "for_expression"
                | "while_expression"
                | "object"
                | "array"
                | "trait_item"
                | "mod_item"
                | "use_declaration"
                | "const_item"
                | "static_item"
                | "macro_definition"
                | "interface_declaration"
                | "type_alias_declaration"
                | "arrow_function"
                | "function_expression"
                | "try_statement"
                | "switch_statement"
                | "match_block"
                | "closure_expression"
                | "dictionary"
                | "list"
                | "tuple"
        )
    }

    pub fn toggle_fold_at_line(&mut self, line: usize, cx: &mut Context<Self>) {
        if let Some(idx) = self.folded.iter().position(|f| f.start_line == line) {
            self.folded.remove(idx);
        } else if let Some(range) = self.fold_ranges.iter().find(|r| r.start_line == line) {
            self.folded.push(*range);
        }
        self.invalidate_folds();
        self.clamp_scroll_after_fold();
        cx.notify();
    }

    pub fn fold_all(&mut self, cx: &mut Context<Self>) {
        self.folded = self.fold_ranges.clone();
        self.invalidate_folds();
        self.clamp_scroll_after_fold();
        cx.notify();
    }

    pub fn unfold_all(&mut self, cx: &mut Context<Self>) {
        self.folded.clear();
        self.invalidate_folds();
        self.clamp_scroll_after_fold();
        cx.notify();
    }

    fn invalidate_folds(&mut self) {
        self.invalidate_all_caches();
        if self.folded.is_empty() {
            self.cached_display_lines = None;
        } else {
            self.cached_display_lines = Some(Rc::new(self.compute_display_lines()));
        }
    }

    fn compute_display_lines(&self) -> Vec<usize> {
        let total = self.total_lines();
        let mut lines = Vec::with_capacity(total);
        let mut skip_until: Option<usize> = None;
        for line in 0..total {
            if let Some(end) = skip_until {
                if line <= end {
                    continue;
                }
                skip_until = None;
            }
            lines.push(line);
            if let Some(fold) = self.folded.iter().find(|f| f.start_line == line) {
                skip_until = Some(fold.end_line);
            }
        }
        lines
    }

    fn clamp_scroll_after_fold(&mut self) {
        let line_height = self.line_height;
        let padding_top = px(12.0);
        let padding_bottom = px(12.0);
        let display_count = self.display_line_count();
        let content_height = padding_top + padding_bottom + line_height * display_count as f32;
        let viewport_height = self.scroll_handle.bounds().size.height;

        if viewport_height <= px(0.0) {
            return;
        }

        let max_scroll = (content_height - viewport_height).max(px(0.0));
        let offset = self.scroll_handle.offset();
        let clamped_y = offset.y.max(-max_scroll).min(px(0.0));
        if (clamped_y - offset.y).abs() > px(0.1) {
            self.scroll_handle.set_offset(point(offset.x, clamped_y));
        }
    }

    pub fn is_line_folded(&self, line: usize) -> bool {
        self.folded
            .iter()
            .any(|f| line > f.start_line && line <= f.end_line)
    }

    pub fn display_lines(&self) -> Rc<Vec<usize>> {
        if let Some(ref cached) = self.cached_display_lines {
            return Rc::clone(cached);
        }
        Rc::new(self.compute_display_lines())
    }

    pub fn display_line_count(&self) -> usize {
        if self.folded.is_empty() {
            return self.total_lines();
        }
        if let Some(ref cached) = self.cached_display_lines {
            return cached.len();
        }
        self.compute_display_lines().len()
    }

    pub fn buffer_line_to_display_row(&self, buffer_line: usize) -> Option<usize> {
        let mut display_row = 0usize;
        let mut skip_until: Option<usize> = None;
        let total = self.total_lines();
        for line in 0..total {
            if let Some(end) = skip_until {
                if line <= end {
                    if line == buffer_line {
                        return None;
                    }
                    continue;
                }
                skip_until = None;
            }
            if line == buffer_line {
                return Some(display_row);
            }
            if let Some(fold) = self.folded.iter().find(|f| f.start_line == line) {
                skip_until = Some(fold.end_line);
            }
            display_row += 1;
        }
        None
    }

    pub fn display_row_to_buffer_line(&self, display_row: usize) -> usize {
        let mut current_display = 0usize;
        let mut skip_until: Option<usize> = None;
        let total = self.total_lines();
        for line in 0..total {
            if let Some(end) = skip_until {
                if line <= end {
                    continue;
                }
                skip_until = None;
            }
            if current_display == display_row {
                return line;
            }
            if let Some(fold) = self.folded.iter().find(|f| f.start_line == line) {
                skip_until = Some(fold.end_line);
            }
            current_display += 1;
        }
        total.saturating_sub(1)
    }

    pub fn fold_ranges(&self) -> &[FoldRange] {
        &self.fold_ranges
    }

    pub fn folded_ranges(&self) -> &[FoldRange] {
        &self.folded
    }

    pub fn scope_breadcrumbs(&self) -> Vec<(String, usize)> {
        let tree = match &self.syntax_tree {
            Some(t) => t,
            None => return Vec::new(),
        };

        let byte_offset = self.pos_to_byte_offset(self.cursor);
        let ts_point = self.byte_to_ts_point(byte_offset);
        let mut node = match tree
            .root_node()
            .descendant_for_point_range(ts_point, ts_point)
        {
            Some(n) => n,
            None => return Vec::new(),
        };

        let mut breadcrumbs = Vec::new();
        loop {
            let kind = node.kind();
            if Self::is_scope_kind(kind) {
                if let Some(name) = Self::extract_scope_name(&node, &self.rope) {
                    let line = node.start_position().row;
                    breadcrumbs.push((name, line));
                }
            }
            match node.parent() {
                Some(p) => node = p,
                None => break,
            }
        }
        breadcrumbs.reverse();
        breadcrumbs
    }

    fn is_scope_kind(kind: &str) -> bool {
        matches!(
            kind,
            "function_item"
                | "impl_item"
                | "struct_item"
                | "enum_item"
                | "trait_item"
                | "mod_item"
                | "function_declaration"
                | "class_declaration"
                | "class_definition"
                | "method_definition"
                | "interface_declaration"
                | "module"
                | "namespace_definition"
        )
    }

    fn extract_scope_name(node: &tree_sitter::Node, rope: &Rope) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                let kind = child.kind();
                if kind == "name"
                    || kind == "identifier"
                    || kind == "type_identifier"
                    || kind == "property_identifier"
                {
                    let start = child.start_byte();
                    let end = child.end_byte().min(rope.len_bytes());
                    if start < end {
                        let name: String = rope.byte_slice(start..end).into();
                        return Some(name);
                    }
                }
            }
        }
        None
    }

    fn closing_char_for(&self, ch: char) -> Option<char> {
        for &(opener, closer) in AUTO_CLOSE_PAIRS {
            if ch == opener {
                if opener == closer {
                    let line_text = self.line_text(self.cursor.line);
                    let col = self.cursor.col.min(line_text.len());
                    let before = &line_text[..col];
                    let count = before.chars().filter(|&c| c == ch).count();
                    if count % 2 != 0 {
                        return None;
                    }
                }
                return Some(closer);
            }
        }
        None
    }

    fn should_skip_closing_char(&self, ch: char) -> bool {
        let is_closer = AUTO_CLOSE_PAIRS.iter().any(|&(_, c)| c == ch);
        if !is_closer {
            return false;
        }
        let line_text = self.line_text(self.cursor.line);
        let col = self.cursor.col;
        if col < line_text.len() {
            let next_ch = line_text[col..].chars().next();
            return next_ch == Some(ch);
        }
        false
    }

    fn is_between_auto_close_pair(&self) -> bool {
        let line_text = self.line_text(self.cursor.line);
        let col = self.cursor.col;
        if col == 0 || col >= line_text.len() {
            return false;
        }
        let before = line_text.as_bytes()[col - 1];
        let after = line_text.as_bytes()[col];
        AUTO_CLOSE_PAIRS
            .iter()
            .any(|&(o, c)| before == o as u8 && after == c as u8)
    }

    pub fn cursor_screen_position(&self, line_height: Pixels) -> Option<Point<Pixels>> {
        let bounds = self.last_bounds?;
        let gutter_width = if self.show_line_numbers {
            px(80.0)
        } else {
            px(12.0)
        };
        let padding_top = px(12.0);

        let cursor_y = bounds.top() + padding_top + line_height * (self.cursor.line as f32);

        let cursor_x = if let Some(layout) = self.line_layouts.get(&self.cursor.line) {
            let line_text = self.line_text(self.cursor.line);
            let char_offset = self.cursor.col.min(line_text.len());
            let x_offset = layout.x_for_index(char_offset);
            bounds.left() + gutter_width + x_offset
        } else {
            let approx_char_width = px(8.4);
            bounds.left() + gutter_width + approx_char_width * (self.cursor.col as f32)
        };

        Some(Point::new(cursor_x, cursor_y + line_height))
    }

    pub fn apply_completion(
        &mut self,
        trigger_col: usize,
        insert_text: &str,
        cx: &mut Context<Self>,
    ) {
        if self.read_only {
            return;
        }

        let delete_count = self.cursor.col.saturating_sub(trigger_col);
        if delete_count > 0 {
            let start_pos = Position::new(self.cursor.line, trigger_col);
            let end_pos = self.cursor;
            self.delete_selection_internal(Selection::new(start_pos, end_pos), cx);
        }

        self.insert_text_at_cursor(insert_text, cx);
        self.ensure_cursor_visible(cx);
        cx.notify();
    }

    fn line_text(&self, line: usize) -> String {
        if line >= self.rope.len_lines() {
            return String::new();
        }
        let line_slice = self.rope.line(line);
        let mut s = line_slice.to_string();
        if s.ends_with('\n') {
            s.pop();
        }
        if s.ends_with('\r') {
            s.pop();
        }
        s
    }

    fn line_len(&self, line: usize) -> usize {
        self.line_text(line).len()
    }

    fn rope_insert(&mut self, byte_offset: usize, text: &str) {
        let char_offset = self.rope.byte_to_char(byte_offset.min(self.rope.len_bytes()));
        self.rope.insert(char_offset, text);
    }

    fn rope_remove(&mut self, byte_start: usize, byte_end: usize) {
        let len = self.rope.len_bytes();
        let char_start = self.rope.byte_to_char(byte_start.min(len));
        let char_end = self.rope.byte_to_char(byte_end.min(len));
        self.rope.remove(char_start..char_end);
    }

    fn total_lines(&self) -> usize {
        self.line_count()
    }

    pub fn set_content(&mut self, content: &str, cx: &mut Context<Self>) {
        self.rope = if content.is_empty() {
            Rope::from_str("\n")
        } else if content.ends_with('\n') {
            Rope::from_str(content)
        } else {
            let mut s = content.to_string();
            s.push('\n');
            Rope::from_str(&s)
        };
        self.cursor = Position::zero();
        self.selection = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.is_modified = false;
        self.invalidate_all_caches();
        if self.rope.len_bytes() > 50_000 {
            self.parse_async(cx);
        } else {
            self.update_syntax_tree();
        }
        cx.notify();
    }

    pub fn set_language(&mut self, lang: Language) {
        self.language = lang;
        if let Some(ts_lang) = lang.tree_sitter_language() {
            let _ = self.parser.set_language(&ts_lang);
            self.highlight_query = lang
                .highlight_query_source()
                .filter(|src| !src.is_empty())
                .and_then(|src| Query::new(&ts_lang, src).ok());
        } else {
            self.highlight_query = None;
        }
        self.update_syntax_tree();
    }

    pub fn set_overlay_active_check(&mut self, check: impl Fn(&App) -> bool + 'static) {
        self.overlay_active_check = Some(Box::new(check));
    }

    fn is_overlay_active(&self, cx: &App) -> bool {
        self.overlay_active_check
            .as_ref()
            .map(|check| check(cx))
            .unwrap_or(false)
    }

    pub fn load_file(&mut self, path: impl Into<PathBuf>, cx: &mut Context<Self>) {
        let path = path.into();
        let lang = Language::from_path(&path);
        self.language = lang;
        if let Some(ts_lang) = lang.tree_sitter_language() {
            let _ = self.parser.set_language(&ts_lang);
            self.highlight_query = lang
                .highlight_query_source()
                .filter(|src| !src.is_empty())
                .and_then(|src| Query::new(&ts_lang, src).ok());
        } else {
            self.highlight_query = None;
        }

        match std::fs::File::open(&path) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                match Rope::from_reader(reader) {
                    Ok(rope) => {
                        self.file_path = Some(path);
                        self.rope = rope;
                        self.cursor = Position::zero();
                        self.selection = None;
                        self.undo_stack.clear();
                        self.redo_stack.clear();
                        self.is_modified = false;
                        self.invalidate_all_caches();
                        if self.rope.len_bytes() > 50_000 {
                            self.parse_async(cx);
                        } else {
                            self.update_syntax_tree();
                        }
                        cx.notify();
                    }
                    Err(_) => {
                        self.file_path = Some(path);
                        self.set_content("", cx);
                        self.is_modified = false;
                    }
                }
            }
            Err(_) => {
                self.file_path = Some(path);
                self.set_content("", cx);
                self.is_modified = false;
            }
        }
    }

    pub fn save_to_file(&mut self, path: impl Into<PathBuf>, cx: &mut Context<Self>) -> bool {
        let path = path.into();
        match std::fs::File::create(&path) {
            Ok(file) => {
                let mut writer = std::io::BufWriter::new(file);
                match self.rope.write_to(&mut writer) {
                    Ok(()) => {
                        self.file_path = Some(path);
                        self.is_modified = false;
                        cx.notify();
                        true
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    pub fn save(&mut self, cx: &mut Context<Self>) -> bool {
        if let Some(path) = self.file_path.clone() {
            self.save_to_file(path, cx)
        } else {
            false
        }
    }

    fn update_syntax_tree(&mut self) {
        let rope = &self.rope;
        self.syntax_tree = self.parser.parse_with_options(
            &mut |byte_idx, _pos| -> &[u8] {
                if byte_idx >= rope.len_bytes() {
                    return &[];
                }
                let (chunk, start, _, _) = rope.chunk_at_byte(byte_idx);
                &chunk.as_bytes()[byte_idx - start..]
            },
            None,
            None,
        );
    }

    fn byte_to_ts_point(&self, byte_offset: usize) -> TSPoint {
        let line = self.rope.byte_to_line(byte_offset);
        let line_start = self.rope.line_to_byte(line);
        TSPoint::new(line, byte_offset - line_start)
    }

    fn update_syntax_tree_incremental(
        &mut self,
        start_byte: usize,
        old_end_byte: usize,
        new_end_byte: usize,
        old_end_position: TSPoint,
        cx: &mut Context<Self>,
    ) {
        let start_position = self.byte_to_ts_point(start_byte);
        let new_end_position = self.byte_to_ts_point(new_end_byte.min(self.rope.len_bytes()));
        if let Some(tree) = &mut self.syntax_tree {
            tree.edit(&InputEdit {
                start_byte,
                old_end_byte,
                new_end_byte,
                start_position,
                old_end_position,
                new_end_position,
            });
        }
        self.schedule_reparse(cx);
    }

    fn parse_async(&mut self, cx: &mut Context<Self>) {
        let content = self.rope.to_string();
        let lang = self.language;
        self.syntax_tree = None;
        let (tx, rx) = smol::channel::bounded(1);
        std::thread::spawn(move || {
            let mut parser = Parser::new();
            if let Some(ts_lang) = lang.tree_sitter_language() {
                let _ = parser.set_language(&ts_lang);
                let tree = parser.parse(&content, None);
                let _ = tx.send_blocking(tree);
            }
        });
        cx.spawn(async move |this, cx| {
            if let Ok(tree) = rx.recv().await {
                let _ = cx.update(|cx| {
                    let _ = this.update(cx, |state, cx| {
                        state.syntax_tree = tree;
                        state.compute_fold_ranges();
                        state.invalidate_all_caches();
                        cx.notify();
                    });
                });
            }
        })
        .detach();
    }

    fn schedule_reparse(&mut self, cx: &mut Context<Self>) {
        let entity = cx.entity().clone();
        self.reparse_task = Some(cx.spawn(async move |_, cx| {
            Timer::after(Duration::from_millis(50)).await;
            let _ = cx.update(|cx| {
                entity.update(cx, |state, cx| {
                    state.update_syntax_tree_incremental_now();
                    state.compute_fold_ranges();
                    // Only invalidate highlights — line layouts are still valid
                    // since text content hasn't changed (only syntax tree updated).
                    state.invalidate_after_edit();
                    cx.notify();
                });
            });
        }));
    }

    fn update_syntax_tree_incremental_now(&mut self) {
        if self.syntax_tree.is_none() {
            return;
        }
        let rope = &self.rope;
        self.syntax_tree = self.parser.parse_with_options(
            &mut |byte_idx, _pos| -> &[u8] {
                if byte_idx >= rope.len_bytes() {
                    return &[];
                }
                let (chunk, start, _, _) = rope.chunk_at_byte(byte_idx);
                &chunk.as_bytes()[byte_idx - start..]
            },
            self.syntax_tree.as_ref(),
            None,
        );
    }

    fn pos_to_byte_offset(&self, pos: Position) -> usize {
        if pos.line >= self.rope.len_lines() {
            return self.rope.len_bytes();
        }
        let line_start = self.rope.line_to_byte(pos.line);
        let line_len = self.line_len(pos.line);
        line_start + min(pos.col, line_len)
    }

    fn byte_offset_to_pos(&self, offset: usize) -> Position {
        let offset = min(offset, self.rope.len_bytes());
        let line = self.rope.byte_to_line(offset);
        let line_start = self.rope.line_to_byte(line);
        let col = offset - line_start;
        Position::new(line, col)
    }

    fn clamp_cursor(&mut self) {
        let max_line = self.total_lines().saturating_sub(1);
        self.cursor.line = min(self.cursor.line, max_line);
        let line_len = self.line_len(self.cursor.line);
        self.cursor.col = min(self.cursor.col, line_len);
    }

    fn mark_modified(&mut self) {
        self.is_modified = true;
        self.content_version = self.content_version.wrapping_add(1);
        self.cursor_visible = true;
        self.last_cursor_move = std::time::Instant::now();
    }

    pub fn content_version(&self) -> u64 {
        self.content_version
    }

    fn insert_text_at_cursor(&mut self, text: &str, cx: &mut Context<Self>) {
        if let Some(selection) = self.selection.take() {
            self.delete_selection_internal(selection, cx);
        }

        let byte_offset = self.pos_to_byte_offset(self.cursor);
        let old_end_position = self.byte_to_ts_point(byte_offset);
        self.undo_stack.push(EditOp::Insert {
            byte_offset,
            text: text.to_string(),
        });
        self.redo_stack.clear();

        self.rope_insert(byte_offset, text);
        self.mark_modified();

        let new_end_byte = byte_offset + text.len();
        self.cursor = self.byte_offset_to_pos(new_end_byte);
        self.update_syntax_tree_incremental(
            byte_offset,
            byte_offset,
            new_end_byte,
            old_end_position,
            cx,
        );
        self.invalidate_after_edit();
    }

    fn delete_selection_internal(&mut self, selection: Selection, cx: &mut Context<Self>) {
        let (start, end) = selection.range();
        let start_offset = self.pos_to_byte_offset(start);
        let end_offset = self.pos_to_byte_offset(end);

        if start_offset >= end_offset {
            self.cursor = start;
            return;
        }

        let old_end_position = self.byte_to_ts_point(end_offset);
        let deleted: String = self.rope.byte_slice(start_offset..end_offset).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: start_offset,
            text: deleted,
        });
        self.redo_stack.clear();

        self.rope_remove(start_offset, end_offset);
        self.mark_modified();
        self.cursor = start;
        self.clamp_cursor();
        self.update_syntax_tree_incremental(
            start_offset,
            end_offset,
            start_offset,
            old_end_position,
            cx,
        );
        self.invalidate_after_edit();
    }

    fn get_selection_text(&self, selection: &Selection) -> String {
        let (start, end) = selection.range();
        let start_offset = self.pos_to_byte_offset(start);
        let end_offset = self.pos_to_byte_offset(end);
        if start_offset >= end_offset {
            return String::new();
        }
        self.rope.byte_slice(start_offset..end_offset).into()
    }

    fn find_word_boundary_left(&self, pos: Position) -> Position {
        if pos.col == 0 {
            if pos.line == 0 {
                return pos;
            }
            return Position::new(pos.line - 1, self.line_len(pos.line - 1));
        }
        let line_text = self.line_text(pos.line);
        let bytes = line_text.as_bytes();
        let mut col = pos.col;
        while col > 0 && bytes[col - 1].is_ascii_whitespace() {
            col -= 1;
        }
        while col > 0
            && !bytes[col - 1].is_ascii_whitespace()
            && bytes[col - 1].is_ascii_alphanumeric()
        {
            col -= 1;
        }
        Position::new(pos.line, col)
    }

    fn find_word_boundary_right(&self, pos: Position) -> Position {
        let line_len = self.line_len(pos.line);
        if pos.col >= line_len {
            if pos.line >= self.total_lines() - 1 {
                return pos;
            }
            return Position::new(pos.line + 1, 0);
        }
        let line_text = self.line_text(pos.line);
        let bytes = line_text.as_bytes();
        let mut col = pos.col;
        while col < line_len && bytes[col].is_ascii_alphanumeric() {
            col += 1;
        }
        while col < line_len && bytes[col].is_ascii_whitespace() {
            col += 1;
        }
        if col == pos.col {
            col += 1;
        }
        Position::new(pos.line, min(col, line_len))
    }

    // UTF-16 conversion helpers for IME support
    fn offset_to_utf16(&self, byte_offset: usize) -> usize {
        let byte_offset = min(byte_offset, self.rope.len_bytes());
        let char_offset = self.rope.byte_to_char(byte_offset);
        let mut utf16_offset = 0;
        for ch_idx in 0..char_offset {
            let ch = self.rope.char(ch_idx);
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn offset_from_utf16(&self, utf16_offset: usize) -> usize {
        let mut utf16_count = 0;
        let mut byte_offset = 0;
        for ch_idx in 0..self.rope.len_chars() {
            if utf16_count >= utf16_offset {
                break;
            }
            let ch = self.rope.char(ch_idx);
            utf16_count += ch.len_utf16();
            byte_offset += ch.len_utf8();
        }
        byte_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    pub fn undo(&mut self, _: &Undo, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(op) = self.undo_stack.pop() {
            match &op {
                EditOp::Insert { byte_offset, text } => {
                    let end = byte_offset + text.len();
                    self.rope_remove(*byte_offset, end);
                    self.cursor = self.byte_offset_to_pos(*byte_offset);
                    self.redo_stack.push(op);
                }
                EditOp::Delete { byte_offset, text } => {
                    self.rope_insert(*byte_offset, text);
                    self.cursor = self.byte_offset_to_pos(*byte_offset + text.len());
                    self.redo_stack.push(op);
                }
            }
            self.selection = None;
            self.mark_modified();
            self.update_syntax_tree();
            self.invalidate_after_edit();
            cx.notify();
        }
    }

    pub fn redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(op) = self.redo_stack.pop() {
            match &op {
                EditOp::Insert { byte_offset, text } => {
                    self.rope_insert(*byte_offset, text);
                    self.cursor = self.byte_offset_to_pos(*byte_offset + text.len());
                    self.undo_stack.push(op);
                }
                EditOp::Delete { byte_offset, text } => {
                    let end = byte_offset + text.len();
                    self.rope_remove(*byte_offset, end);
                    self.cursor = self.byte_offset_to_pos(*byte_offset);
                    self.undo_stack.push(op);
                }
            }
            self.selection = None;
            self.mark_modified();
            self.update_syntax_tree();
            self.invalidate_after_edit();
            cx.notify();
        }
    }

    pub fn move_up(&mut self, _: &MoveUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_overlay_active(cx) {
            cx.propagate();
            return;
        }
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.clamp_cursor();
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_down(&mut self, _: &MoveDown, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_overlay_active(cx) {
            cx.propagate();
            return;
        }
        if self.cursor.line < self.total_lines() - 1 {
            self.cursor.line += 1;
            self.clamp_cursor();
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_left(&mut self, _: &MoveLeft, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.col = self.line_len(self.cursor.line);
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_right(&mut self, _: &MoveRight, _: &mut Window, cx: &mut Context<Self>) {
        let line_len = self.line_len(self.cursor.line);
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.line < self.total_lines() - 1 {
            self.cursor.line += 1;
            self.cursor.col = 0;
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_word_left(&mut self, _: &MoveWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor = self.find_word_boundary_left(self.cursor);
        self.selection = None;
        cx.notify();
    }

    pub fn move_word_right(&mut self, _: &MoveWordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor = self.find_word_boundary_right(self.cursor);
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_line_start(
        &mut self,
        _: &MoveToLineStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cursor.col = 0;
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_line_end(&mut self, _: &MoveToLineEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor.col = self.line_len(self.cursor.line);
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_doc_start(
        &mut self,
        _: &MoveToDocStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cursor = Position::zero();
        self.selection = None;
        cx.notify();
    }

    pub fn move_to_doc_end(&mut self, _: &MoveToDocEnd, _: &mut Window, cx: &mut Context<Self>) {
        let last = self.total_lines() - 1;
        self.cursor = Position::new(last, self.line_len(last));
        self.selection = None;
        cx.notify();
    }

    pub fn page_up(&mut self, _: &PageUp, _: &mut Window, cx: &mut Context<Self>) {
        let page_size = 30;
        self.cursor.line = self.cursor.line.saturating_sub(page_size);
        self.clamp_cursor();
        self.selection = None;
        cx.notify();
    }

    pub fn page_down(&mut self, _: &PageDown, _: &mut Window, cx: &mut Context<Self>) {
        let page_size = 30;
        self.cursor.line = min(self.cursor.line + page_size, self.total_lines() - 1);
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
        if self.cursor.line < self.total_lines() - 1 {
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
            self.cursor.col = self.line_len(self.cursor.line);
        }
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.start_selection_if_needed();
        let line_len = self.line_len(self.cursor.line);
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.line < self.total_lines() - 1 {
            self.cursor.line += 1;
            self.cursor.col = 0;
        }
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_to_line_start(
        &mut self,
        _: &SelectToLineStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection_if_needed();
        self.cursor.col = 0;
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_to_line_end(
        &mut self,
        _: &SelectToLineEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection_if_needed();
        self.cursor.col = self.line_len(self.cursor.line);
        if let Some(ref mut sel) = self.selection {
            sel.cursor = self.cursor;
        }
        cx.notify();
    }

    pub fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        let start = Position::zero();
        let last = self.total_lines() - 1;
        let end = Position::new(last, self.line_len(last));
        self.selection = Some(Selection::new(start, end));
        self.cursor = end;
        cx.notify();
    }

    pub fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }
        if let Some(selection) = self.selection.take() {
            self.delete_selection_internal(selection, cx);
            cx.notify();
            return;
        }
        let offset = self.pos_to_byte_offset(self.cursor);
        if offset == 0 {
            return;
        }

        let delete_pair = self.is_between_auto_close_pair();
        let char_idx = self.rope.byte_to_char(offset);
        let prev_char_byte = self.rope.char_to_byte(char_idx.saturating_sub(1));
        let del_start = prev_char_byte;
        let del_end = if delete_pair {
            let next_char_byte = if char_idx < self.rope.len_chars() {
                self.rope.char_to_byte(char_idx + 1)
            } else {
                self.rope.len_bytes()
            };
            next_char_byte.min(self.rope.len_bytes())
        } else {
            offset
        };

        let old_end_position = self.byte_to_ts_point(del_end);
        let deleted: String = self.rope.byte_slice(del_start..del_end).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: del_start,
            text: deleted,
        });
        self.redo_stack.clear();
        self.rope_remove(del_start, del_end);
        self.mark_modified();
        self.cursor = self.byte_offset_to_pos(del_start);
        self.update_syntax_tree_incremental(del_start, del_end, del_start, old_end_position, cx);
        self.invalidate_after_edit();
        cx.notify();
    }

    pub fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }
        if let Some(selection) = self.selection.take() {
            self.delete_selection_internal(selection, cx);
            cx.notify();
            return;
        }
        let offset = self.pos_to_byte_offset(self.cursor);
        if offset >= self.rope.len_bytes() {
            return;
        }
        let char_idx = self.rope.byte_to_char(offset);
        let next_char_byte = if char_idx + 1 <= self.rope.len_chars() {
            self.rope.char_to_byte(char_idx + 1)
        } else {
            self.rope.len_bytes()
        };
        let del_end = min(next_char_byte, self.rope.len_bytes());
        let old_end_position = self.byte_to_ts_point(del_end);
        let deleted: String = self.rope.byte_slice(offset..del_end).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: offset,
            text: deleted,
        });
        self.redo_stack.clear();
        self.rope_remove(offset, del_end);
        self.mark_modified();
        self.update_syntax_tree_incremental(offset, del_end, offset, old_end_position, cx);
        self.invalidate_after_edit();
        cx.notify();
    }

    pub fn delete_word(&mut self, _: &DeleteWord, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }
        let word_start = self.find_word_boundary_left(self.cursor);
        if word_start == self.cursor {
            return;
        }
        let start_offset = self.pos_to_byte_offset(word_start);
        let end_offset = self.pos_to_byte_offset(self.cursor);
        let old_end_position = self.byte_to_ts_point(end_offset);
        let deleted: String = self.rope.byte_slice(start_offset..end_offset).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: start_offset,
            text: deleted,
        });
        self.redo_stack.clear();
        self.rope_remove(start_offset, end_offset);
        self.mark_modified();
        self.cursor = word_start;
        self.update_syntax_tree_incremental(
            start_offset,
            end_offset,
            start_offset,
            old_end_position,
            cx,
        );
        self.invalidate_after_edit();
        cx.notify();
    }

    pub fn enter(&mut self, _: &Enter, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_overlay_active(cx) {
            cx.propagate();
            return;
        }
        if self.read_only {
            return;
        }

        let line_text = self.line_text(self.cursor.line);
        let before_cursor = &line_text[..self.cursor.col.min(line_text.len())];
        let after_cursor = &line_text[self.cursor.col.min(line_text.len())..];

        let base_indent = before_cursor.len() - before_cursor.trim_start().len();
        let trimmed = before_cursor.trim_end();
        let increase = matches!(trimmed.as_bytes().last(), Some(b'{' | b'(' | b'[' | b':'));

        let indent_str = " ".repeat(base_indent);
        let extra_indent = " ".repeat(self.tab_size);

        let after_trimmed = after_cursor.trim_start();
        let between_pair = increase
            && !after_trimmed.is_empty()
            && matches!(
                (trimmed.as_bytes().last(), after_trimmed.as_bytes().first()),
                (Some(b'{'), Some(b'}')) | (Some(b'('), Some(b')')) | (Some(b'['), Some(b']'))
            );

        if between_pair {
            let text = format!("\n{}{}\n{}", indent_str, extra_indent, indent_str);
            self.insert_text_at_cursor(&text, cx);
            let target_line = self.cursor.line - 1;
            let target_col = base_indent + self.tab_size;
            self.cursor = Position::new(target_line, target_col);
        } else if increase {
            let text = format!("\n{}{}", indent_str, extra_indent);
            self.insert_text_at_cursor(&text, cx);
        } else {
            let text = format!("\n{}", indent_str);
            self.insert_text_at_cursor(&text, cx);
        }
        self.ensure_cursor_visible(cx);
    }

    pub fn tab(&mut self, _: &Tab, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_overlay_active(cx) {
            cx.propagate();
            return;
        }
        if self.read_only {
            return;
        }
        let spaces = " ".repeat(self.tab_size);
        self.insert_text_at_cursor(&spaces, cx);
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
            self.delete_selection_internal(selection, cx);
            cx.notify();
        }
    }

    pub fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }
        if let Some(item) = cx.read_from_clipboard() {
            if let Some(text) = item.text() {
                self.insert_text_at_cursor(&text, cx);
            }
        }
    }

    pub fn selection_text(&self) -> Option<String> {
        self.selection
            .as_ref()
            .map(|sel| self.get_selection_text(sel))
            .filter(|s| !s.is_empty())
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn search_match_count(&self) -> usize {
        self.search_matches.len()
    }

    pub fn current_match_index(&self) -> Option<usize> {
        self.current_match_idx
    }

    pub fn search_case_sensitive(&self) -> bool {
        self.search_case_sensitive
    }

    pub fn search_use_regex(&self) -> bool {
        self.search_use_regex
    }

    pub fn find_all(&mut self, query: &str, cx: &mut Context<Self>) {
        self.search_query = query.to_string();

        if query.is_empty() {
            self.search_matches.clear();
            self.current_match_idx = None;
            self.search_task = None;
            cx.notify();
            return;
        }

        // Schedule a debounced search — any new call cancels the previous one.
        // This ensures typing always takes priority; search only runs after
        // the user stops typing for 200ms.
        self.schedule_search(cx);
    }

    fn schedule_search(&mut self, cx: &mut Context<Self>) {
        let query_owned = self.search_query.clone();
        let use_regex = self.search_use_regex;
        let case_sensitive = self.search_case_sensitive;
        let entity = cx.entity().clone();

        // Cancel any in-flight search
        self.search_task = Some(cx.spawn(async move |_, cx| {
            // Wait for user to stop typing
            Timer::after(Duration::from_millis(200)).await;

            // Snapshot content and cursor on the main thread, then search in background
            let search_input = cx.update(|cx| {
                let state = entity.read(cx);
                let content = state.rope.to_string();
                let cursor_byte = state.pos_to_byte_offset(state.cursor);
                (content, cursor_byte)
            });

            let Ok((content, cursor_byte)) = search_input else { return };

            let matches = smol::unblock(move || {
                let mut results = Vec::new();
                if use_regex {
                    let pattern = if case_sensitive {
                        query_owned.to_string()
                    } else {
                        format!("(?i){}", query_owned)
                    };
                    if let Ok(re) = Regex::new(&pattern) {
                        for m in re.find_iter(&content) {
                            results.push((m.start(), m.end()));
                        }
                    }
                } else {
                    let (haystack, needle): (String, String) = if case_sensitive {
                        (content.to_string(), query_owned.to_string())
                    } else {
                        (content.to_lowercase(), query_owned.to_lowercase())
                    };
                    let needle_len = needle.len();
                    let mut start = 0;
                    while let Some(pos) = haystack[start..].find(&needle) {
                        let match_start = start + pos;
                        let match_end = match_start + needle_len;
                        results.push((match_start, match_end));
                        start = match_start + 1;
                    }
                }
                results
            })
            .await;

            let _ = cx.update(|cx| {
                entity.update(cx, |state, cx| {
                    state.search_matches = matches;
                    if !state.search_matches.is_empty() {
                        let idx = state
                            .search_matches
                            .iter()
                            .position(|(s, _)| *s >= cursor_byte)
                            .unwrap_or(0);
                        state.current_match_idx = Some(idx);
                        state.scroll_to_match(idx);
                    } else {
                        state.current_match_idx = None;
                    }
                    cx.notify();
                });
            });
        }));
    }

    pub fn find_next(&mut self, cx: &mut Context<Self>) {
        if self.search_matches.is_empty() {
            return;
        }
        let next = match self.current_match_idx {
            Some(idx) => (idx + 1) % self.search_matches.len(),
            None => 0,
        };
        self.current_match_idx = Some(next);
        let (start, _) = self.search_matches[next];
        self.cursor = self.byte_offset_to_pos(start);
        self.selection = None;
        self.scroll_to_match(next);
        cx.notify();
    }

    pub fn find_previous(&mut self, cx: &mut Context<Self>) {
        if self.search_matches.is_empty() {
            return;
        }
        let prev = match self.current_match_idx {
            Some(0) | None => self.search_matches.len() - 1,
            Some(idx) => idx - 1,
        };
        self.current_match_idx = Some(prev);
        let (start, _) = self.search_matches[prev];
        self.cursor = self.byte_offset_to_pos(start);
        self.selection = None;
        self.scroll_to_match(prev);
        cx.notify();
    }

    pub fn replace_current(&mut self, replacement: &str, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }
        let idx = match self.current_match_idx {
            Some(i) if i < self.search_matches.len() => i,
            _ => return,
        };
        let (start, end) = self.search_matches[idx];
        let old_end_position = self.byte_to_ts_point(end.min(self.rope.len_bytes()));
        let deleted: String = self.rope.byte_slice(start..end).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: start,
            text: deleted,
        });
        self.rope_remove(start, end);
        self.undo_stack.push(EditOp::Insert {
            byte_offset: start,
            text: replacement.to_string(),
        });
        self.rope_insert(start, replacement);
        self.redo_stack.clear();
        self.mark_modified();
        let new_end = start + replacement.len();
        self.update_syntax_tree_incremental(start, end, new_end, old_end_position, cx);
        self.invalidate_after_edit();
        let query = self.search_query.clone();
        self.find_all(&query, cx);
    }

    pub fn replace_all(&mut self, replacement: &str, cx: &mut Context<Self>) {
        if self.read_only || self.search_matches.is_empty() {
            return;
        }
        let matches: Vec<_> = self.search_matches.iter().rev().copied().collect();
        for (start, end) in matches {
            let deleted: String = self.rope.byte_slice(start..end).into();
            self.undo_stack.push(EditOp::Delete {
                byte_offset: start,
                text: deleted,
            });
            self.rope_remove(start, end);
            self.undo_stack.push(EditOp::Insert {
                byte_offset: start,
                text: replacement.to_string(),
            });
            self.rope_insert(start, replacement);
        }
        self.redo_stack.clear();
        self.mark_modified();
        self.update_syntax_tree();
        self.invalidate_after_edit();
        let query = self.search_query.clone();
        self.find_all(&query, cx);
    }

    /// Full invalidation — clears all caches. Use for structural changes
    /// (file load, language change, fold/unfold).
    fn invalidate_all_caches(&mut self) {
        self.line_layouts.clear();
        self.line_content_hashes.clear();
        self.highlight_cache_version = u64::MAX;
    }

    /// Invalidation for text edits. Clears all caches since line indices
    /// shift on insert/delete, making index-keyed caches stale.
    fn invalidate_after_edit(&mut self) {
        self.line_layouts.clear();
        self.line_content_hashes.clear();
        self.highlight_cache_version = u64::MAX;
    }

    pub fn invalidate_line_layouts(&mut self, cx: &mut Context<Self>) {
        self.invalidate_all_caches();
        cx.notify();
    }

    pub fn clear_search(&mut self, cx: &mut Context<Self>) {
        self.search_query.clear();
        self.search_matches.clear();
        self.current_match_idx = None;
        cx.notify();
    }

    pub fn set_search_case_sensitive(&mut self, val: bool, cx: &mut Context<Self>) {
        self.search_case_sensitive = val;
        if !self.search_query.is_empty() {
            let query = self.search_query.clone();
            self.find_all(&query, cx);
        } else {
            cx.notify();
        }
    }

    pub fn set_search_regex(&mut self, val: bool, cx: &mut Context<Self>) {
        self.search_use_regex = val;
        if !self.search_query.is_empty() {
            let query = self.search_query.clone();
            self.find_all(&query, cx);
        } else {
            cx.notify();
        }
    }

    pub fn goto_line(&mut self, line: usize, cx: &mut Context<Self>) {
        let target = line
            .saturating_sub(1)
            .min(self.total_lines().saturating_sub(1));
        self.cursor = Position::new(target, 0);
        self.selection = None;
        self.ensure_cursor_visible(cx);
    }

    fn scroll_to_match(&mut self, idx: usize) {
        if idx >= self.search_matches.len() {
            return;
        }
        let (start, _) = self.search_matches[idx];
        let pos = self.byte_offset_to_pos(start);
        let line_height = self.line_height;
        let padding_top = px(12.0);
        let viewport_bounds = self.scroll_handle.bounds();
        let viewport_height = viewport_bounds.size.height;
        let offset = self.scroll_handle.offset();
        let target_y = padding_top + line_height * (pos.line as f32);
        let current_top = -offset.y;
        let current_bottom = current_top + viewport_height;

        let mut new_offset_y = offset.y;
        if target_y < current_top || target_y + line_height > current_bottom {
            new_offset_y = -(target_y - viewport_height / 2.0 + line_height / 2.0);
            let max_offset = self.scroll_handle.max_offset().height;
            new_offset_y = new_offset_y.max(-max_offset).min(px(0.0));
        }

        if (new_offset_y - offset.y).abs() > px(0.0) {
            self.scroll_handle.set_offset(point(offset.x, new_offset_y));
        }
    }

    fn ensure_cursor_visible(&mut self, cx: &mut Context<Self>) {
        let line_height = self.line_height;
        let padding_top = px(12.0);
        let viewport_bounds = self.scroll_handle.bounds();
        let viewport_height = viewport_bounds.size.height;
        let viewport_width = viewport_bounds.size.width;
        let gutter_width = if self.show_line_numbers {
            px(80.0)
        } else {
            px(12.0)
        };
        let content_width = viewport_width - gutter_width;
        let offset = self.scroll_handle.offset();
        let mut new_offset_y = offset.y;
        let display_row = self
            .buffer_line_to_display_row(self.cursor.line)
            .unwrap_or(0);
        let cursor_y = padding_top + line_height * (display_row as f32);
        let current_top = -offset.y;
        let current_bottom = current_top + viewport_height;

        if cursor_y < current_top {
            new_offset_y = -cursor_y;
        } else if cursor_y + line_height > current_bottom {
            new_offset_y = -(cursor_y + line_height - viewport_height);
        }

        let max_offset = self.scroll_handle.max_offset().height;
        new_offset_y = new_offset_y.max(-max_offset).min(px(0.0));

        if (new_offset_y - offset.y).abs() > px(0.0) {
            self.scroll_handle.set_offset(point(offset.x, new_offset_y));
        }

        if let Some(layout) = self.line_layouts.get(&self.cursor.line) {
            let cursor_x = layout.x_for_index(self.cursor.col);
            let visible_left = self.scroll_offset_x;
            let visible_right = visible_left + content_width - px(20.0);

            if cursor_x < visible_left {
                self.scroll_offset_x = (cursor_x - px(20.0)).max(px(0.0));
            } else if cursor_x > visible_right {
                self.scroll_offset_x = cursor_x - content_width + px(40.0);
            }
        }

        cx.notify();
    }

    pub fn scroll_horizontal(&mut self, delta: Pixels, cx: &mut Context<Self>) {
        let viewport_bounds = self.scroll_handle.bounds();
        let gutter_width = if self.show_line_numbers {
            px(80.0)
        } else {
            px(12.0)
        };
        let content_width = viewport_bounds.size.width - gutter_width;
        let max_scroll = (self.max_line_width - content_width + px(40.0)).max(px(0.0));

        self.scroll_offset_x = (self.scroll_offset_x + delta).max(px(0.0)).min(max_scroll);
        cx.notify();
    }

    pub fn scroll_offset_x(&self) -> Pixels {
        self.scroll_offset_x
    }

    pub fn max_line_width(&self) -> Pixels {
        self.max_line_width
    }

    fn position_for_mouse(
        &self,
        mouse_pos: Point<Pixels>,
        bounds: Bounds<Pixels>,
        gutter_width: Pixels,
        line_height: Pixels,
    ) -> Position {
        let padding_top = px(12.0);
        let relative_y = mouse_pos.y - bounds.top() - padding_top;
        let display_row_f = (relative_y / line_height).floor();
        let display_lines = self.display_lines();
        let display_count = display_lines.len();
        let display_row = if display_row_f < 0.0 {
            0
        } else {
            min(display_row_f as usize, display_count.saturating_sub(1))
        };
        let line = display_lines.get(display_row).copied().unwrap_or(0);

        let relative_x = mouse_pos.x - bounds.left() - gutter_width + self.scroll_offset_x;
        let col = if let Some(layout) = self.line_layouts.get(&line) {
            let idx = layout.closest_index_for_x(relative_x);
            idx.min(self.line_len(line))
        } else {
            let approx_char_width = px(8.4);
            if relative_x > px(0.0) {
                let col = (relative_x / approx_char_width).round() as usize;
                col.min(self.line_len(line))
            } else {
                0
            }
        };

        Position::new(line, col)
    }

    fn start_autoscroll(&mut self, cx: &mut Context<Self>) {
        let entity = cx.entity().clone();
        let line_height = self.line_height;
        self.autoscroll_task = Some(cx.spawn(async move |_, cx| loop {
            Timer::after(Duration::from_millis(50)).await;
            let should_continue = cx
                .update(|cx| {
                    entity.update(cx, |state, cx| {
                        if !state.is_selecting {
                            return false;
                        }
                        let Some(mouse_pos) = state.last_mouse_pos else {
                            return true;
                        };
                        let Some(bounds) = state.last_bounds else {
                            return true;
                        };

                        let viewport_bounds = state.scroll_handle.bounds();
                        if viewport_bounds.size.height == px(0.0) {
                            return true;
                        }
                        let viewport_top = viewport_bounds.top();
                        let viewport_bottom = viewport_bounds.bottom();
                        let mouse_y = mouse_pos.y;
                        let edge_zone = line_height * 1.5;
                        let mut scrolled = false;

                        if mouse_y < viewport_top + edge_zone {
                            let speed = ((viewport_top + edge_zone - mouse_y) / edge_zone)
                                .max(0.5)
                                .min(5.0);
                            let offset = state.scroll_handle.offset();
                            let new_y = (offset.y + line_height * speed).min(px(0.0));
                            state.scroll_handle.set_offset(point(offset.x, new_y));
                            scrolled = true;
                        } else if mouse_y > viewport_bottom - edge_zone {
                            let speed = ((mouse_y - (viewport_bottom - edge_zone)) / edge_zone)
                                .max(0.5)
                                .min(5.0);
                            let offset = state.scroll_handle.offset();
                            let max_offset = state.scroll_handle.max_offset().height;
                            let new_y = (offset.y - line_height * speed).max(-max_offset);
                            state.scroll_handle.set_offset(point(offset.x, new_y));
                            scrolled = true;
                        }

                        if scrolled {
                            let gutter_width = state.last_mouse_gutter_width;
                            let pos = state.position_for_mouse(
                                mouse_pos,
                                bounds,
                                gutter_width,
                                line_height,
                            );
                            if let Some(ref mut sel) = state.selection {
                                sel.cursor = pos;
                            } else {
                                state.selection = Some(Selection::new(state.cursor, pos));
                            }
                            state.cursor = pos;
                            cx.notify();
                        }
                        true
                    })
                })
                .unwrap_or(false);
            if !should_continue {
                break;
            }
        }));
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        bounds: Bounds<Pixels>,
        gutter_width: Pixels,
        line_height: Pixels,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        let click_x = event.position.x - bounds.left();
        let padding_top = px(12.0);
        let display_row = ((event.position.y - bounds.top() - padding_top) / line_height)
            .floor()
            .max(0.0) as usize;
        let dl = self.display_lines();
        let click_line = dl.get(display_row).copied().unwrap_or(0);

        if click_x >= gutter_width - px(16.0) && click_x <= gutter_width {
            if self.fold_ranges.iter().any(|f| f.start_line == click_line) {
                self.toggle_fold_at_line(click_line, cx);
                return;
            }
        }

        let pos = self.position_for_mouse(event.position, bounds, gutter_width, line_height);

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
                Position::new(pos.line, self.line_len(pos.line)),
            ));
            self.cursor = Position::new(pos.line, self.line_len(pos.line));
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
            self.last_mouse_pos = Some(event.position);
            self.last_mouse_gutter_width = gutter_width;
            self.start_autoscroll(cx);
        }

        cx.notify();
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        bounds: Bounds<Pixels>,
        gutter_width: Pixels,
        line_height: Pixels,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging_h_scrollbar {
            if event.pressed_button != Some(MouseButton::Left) {
                self.dragging_h_scrollbar = false;
                cx.notify();
                return;
            }
            let max_w = self.max_line_width;
            let vp = self.scroll_handle.bounds();
            let gw = if self.show_line_numbers {
                px(80.0)
            } else {
                px(12.0)
            };
            let cw = vp.size.width - gw;
            let scroll_range = max_w - cw;

            if scroll_range > px(0.0) {
                let track_width = vp.size.width;
                let click_ratio = (event.position.x - vp.left()) / track_width;
                let new_scroll = scroll_range * click_ratio;
                self.scroll_offset_x = new_scroll.max(px(0.0)).min(scroll_range);
                cx.notify();
            }
            return;
        }

        if !self.is_selecting || event.pressed_button != Some(MouseButton::Left) {
            if self.is_selecting && event.pressed_button != Some(MouseButton::Left) {
                self.is_selecting = false;
                self.last_mouse_pos = None;
                cx.notify();
            }
            return;
        }

        self.last_mouse_pos = Some(event.position);
        self.last_mouse_gutter_width = gutter_width;

        let pos = self.position_for_mouse(event.position, bounds, gutter_width, line_height);
        if let Some(ref mut sel) = self.selection {
            sel.cursor = pos;
        } else {
            self.selection = Some(Selection::new(self.cursor, pos));
        }
        self.cursor = pos;
        self.ensure_cursor_visible(cx);
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.is_selecting = false;
        self.dragging_h_scrollbar = false;
        self.autoscroll_task = None;
        self.last_mouse_pos = None;
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
        let start_pos = self.byte_offset_to_pos(range.start);
        let end_pos = self.byte_offset_to_pos(range.end);
        Some(self.get_selection_text(&Selection::new(start_pos, end_pos)))
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        if let Some(selection) = &self.selection {
            let start_offset = self.pos_to_byte_offset(selection.anchor);
            let end_offset = self.pos_to_byte_offset(selection.cursor);
            let range = self.range_to_utf16(&(start_offset..end_offset));
            Some(UTF16Selection {
                range,
                reversed: selection.anchor > selection.cursor,
            })
        } else {
            let cursor_offset = self.pos_to_byte_offset(self.cursor);
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
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
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
                    let start = self.pos_to_byte_offset(sel.anchor);
                    let end = self.pos_to_byte_offset(sel.cursor);
                    Some(start.min(end)..start.max(end))
                } else {
                    let cursor_offset = self.pos_to_byte_offset(self.cursor);
                    Some(cursor_offset..cursor_offset)
                }
            });

        if let Some(range) = range_utf8 {
            let start_pos = self.byte_offset_to_pos(range.start);
            let end_pos = self.byte_offset_to_pos(range.end);

            if start_pos != end_pos {
                self.delete_selection_internal(Selection::new(start_pos, end_pos), cx);
            }

            if new_text.len() == 1 && self.selection.is_none() {
                let ch = new_text.chars().next().unwrap();

                if let Some(closer) = self.closing_char_for(ch) {
                    let pair_text = format!("{}{}", ch, closer);
                    self.insert_text_at_cursor(&pair_text, cx);
                    self.cursor.col = self.cursor.col.saturating_sub(1);
                    self.marked_range = None;
                    return;
                }

                if self.should_skip_closing_char(ch) {
                    self.cursor.col += 1;
                    self.marked_range = None;
                    cx.notify();
                    return;
                }
            }

            self.insert_text_at_cursor(new_text, cx);
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
                let cursor_offset = self.pos_to_byte_offset(self.cursor);
                cursor_offset..cursor_offset
            });

        let start_pos = self.byte_offset_to_pos(range_utf8.start);
        let end_pos = self.byte_offset_to_pos(range_utf8.end);

        if start_pos != end_pos {
            self.delete_selection_internal(Selection::new(start_pos, end_pos), cx);
        }

        let insert_start = self.pos_to_byte_offset(self.cursor);
        self.insert_text_at_cursor(new_text, cx);
        let insert_end = self.pos_to_byte_offset(self.cursor);

        if !new_text.is_empty() {
            self.marked_range = Some(insert_start..insert_end);
        }

        if let Some(new_sel_utf16) = new_selected_range_utf16 {
            let new_sel_utf8 = self.range_from_utf16(&new_sel_utf16);
            let sel_start = self.byte_offset_to_pos(insert_start + new_sel_utf8.start);
            let sel_end = self.byte_offset_to_pos(insert_start + new_sel_utf8.end);
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
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        if let Some(bounds) = self.last_bounds {
            let gutter_width = if self.show_line_numbers {
                px(80.0)
            } else {
                px(12.0)
            };
            let line_height = self.line_height;
            let pos = self.position_for_mouse(point, bounds, gutter_width, line_height);
            let offset = self.pos_to_byte_offset(pos);
            Some(self.offset_to_utf16(offset))
        } else {
            None
        }
    }
}

impl Render for EditorState {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        EditorElement { state: cx.entity() }
    }
}

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
        let line_height = self.state.read(cx).line_height;
        let padding_top = px(12.0);
        let padding_bottom = px(12.0);
        let num_lines = self.state.read(cx).display_line_count();
        let content_height = padding_top + padding_bottom + (line_height * num_lines as f32);
        let viewport_height = self.state.read(cx).scroll_handle.bounds().size.height;
        let overscroll = if viewport_height > line_height * 5.0 {
            viewport_height / 2.0
        } else {
            px(100.0)
        };
        let final_height = content_height + overscroll;

        let mut layout_style = gpui::Style::default();
        layout_style.size.width = relative(1.).into();
        layout_style.size.height = final_height.into();

        (window.request_layout(layout_style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let state = self.state.read(cx);
        let show_line_numbers = state.show_line_numbers;
        let line_height = state.line_height;
        PrepaintState {
            gutter_width: if show_line_numbers {
                px(80.0)
            } else {
                px(12.0)
            },
            line_height,
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
        let line_height = prepaint.line_height;
        let gutter_width = prepaint.gutter_width;
        let font_size = self.state.read(cx).font_size;

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.state.clone()),
            cx,
        );

        self.state.update(cx, |state, _| {
            state.last_bounds = Some(bounds);
        });

        let scroll_offset = self.state.read(cx).scroll_handle.offset();
        let viewport_height = self.state.read(cx).scroll_handle.bounds().size.height;

        let display_lines_vec = self.state.read(cx).display_lines();
        let display_count = display_lines_vec.len();
        let buf_to_disp =
            |line: usize| -> Option<usize> { display_lines_vec.binary_search(&line).ok() };

        let first_visible_display_row = ((-scroll_offset.y - padding_top) / line_height)
            .floor()
            .max(0.0) as usize;
        let visible_rows = ((viewport_height / line_height).ceil() as usize + 2).max(1);
        let last_visible_display_row = min(first_visible_display_row + visible_rows, display_count);

        let visible_buffer_lines = if first_visible_display_row < display_count
            && last_visible_display_row <= display_count
        {
            &display_lines_vec[first_visible_display_row..last_visible_display_row]
        } else {
            &[]
        };

        let (cursor, selection, show_line_numbers, scroll_offset_x) = {
            let state = self.state.read(cx);
            (
                state.cursor,
                state.selection,
                state.show_line_numbers,
                state.scroll_offset_x,
            )
        };

        let (
            gutter_bg_color,
            line_num_color,
            line_num_active_color,
            current_line_color,
            bracket_match_color,
            word_highlight_color,
            indent_guide_color,
            indent_guide_active_color,
            fold_marker_color,
            tab_size,
            folded_ranges,
            fold_ranges,
        ) = {
            let s = self.state.read(cx);
            (
                s.gutter_bg_override.unwrap_or(theme.tokens.background),
                s.line_number_color_override
                    .unwrap_or(theme.tokens.muted_foreground),
                s.line_number_active_color_override
                    .unwrap_or(theme.tokens.foreground),
                s.current_line_color_override
                    .unwrap_or(hsla(0.0, 0.0, 1.0, 0.06)),
                s.bracket_match_color_override
                    .unwrap_or(hsla(0.58, 0.70, 0.65, 0.60)),
                s.word_highlight_color_override
                    .unwrap_or(hsla(0.0, 0.0, 1.0, 0.08)),
                s.indent_guide_color_override
                    .unwrap_or(hsla(0.0, 0.0, 1.0, 0.06)),
                s.indent_guide_active_color_override
                    .unwrap_or(hsla(0.0, 0.0, 1.0, 0.15)),
                s.fold_marker_color_override
                    .unwrap_or(theme.tokens.muted_foreground),
                s.tab_size,
                s.folded.clone(),
                s.fold_ranges.clone(),
            )
        };

        let is_focused = focus_handle.is_focused(window);
        let is_single_cursor =
            selection.is_none() || selection.as_ref().map(|s| s.is_empty()).unwrap_or(true);

        if is_focused && is_single_cursor {
            if let Some(display_row) = buf_to_disp(cursor.line) {
                if display_row >= first_visible_display_row
                    && display_row < last_visible_display_row
                {
                    let hl_y = bounds.top() + padding_top + line_height * display_row as f32;
                    window.paint_quad(fill(
                        Bounds::new(
                            point(bounds.left(), hl_y),
                            size(bounds.size.width, line_height),
                        ),
                        current_line_color,
                    ));
                }
            }
        }

        // Cache highlight spans — only recompute when content changes or visible range shifts
        let content_version = self.state.read(cx).content_version;
        let first_buf = visible_buffer_lines.first().copied().unwrap_or(0);
        let last_buf = visible_buffer_lines.last().copied().unwrap_or(0) + 1;
        {
            let state = self.state.read(cx);
            let needs_rehighlight = state.highlight_cache_version != content_version
                || first_buf < state.highlight_cache_first_line
                || last_buf > state.highlight_cache_last_line;
            if needs_rehighlight {
                let spans = self.collect_highlight_spans_for_lines(visible_buffer_lines, cx);
                self.state.update(cx, |state, _| {
                    state.cached_highlight_spans = spans;
                    state.highlight_cache_version = content_version;
                    state.highlight_cache_first_line = first_buf;
                    state.highlight_cache_last_line = last_buf;
                });
            }
        }

        let text_style = window.text_style();
        let mut shaped_layouts: Vec<(usize, Option<ShapedLine>, u64)> =
            Vec::with_capacity(visible_buffer_lines.len());
        let mut max_line_width = px(0.0);

        let char_width = {
            let space_run = TextRun {
                len: 1,
                font: text_style.font(),
                color: theme.tokens.foreground,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let shaped_space =
                window
                    .text_system()
                    .shape_line(" ".into(), font_size, &[space_run], None);
            shaped_space.x_for_index(1)
        };

        let cursor_indent = if tab_size > 0 {
            let cursor_line_text = self.state.read(cx).line_text(cursor.line);
            let cursor_leading = cursor_line_text.len() - cursor_line_text.trim_start().len();
            cursor_leading / tab_size
        } else {
            0
        };

        for display_row in first_visible_display_row..last_visible_display_row {
            let line_idx = display_lines_vec[display_row];
            let y = bounds.top() + padding_top + line_height * display_row as f32;

            let line_text = self.state.read(cx).line_text(line_idx);
            let leading_spaces = line_text.len() - line_text.trim_start().len();
            let indent_levels = if tab_size > 0 {
                leading_spaces / tab_size
            } else {
                0
            };

            for level in 0..indent_levels {
                let guide_x = bounds.left() + gutter_width + char_width * (level * tab_size) as f32
                    - scroll_offset_x;
                let color = if level == cursor_indent.saturating_sub(1) && is_focused {
                    indent_guide_active_color
                } else {
                    indent_guide_color
                };
                window.paint_quad(fill(
                    Bounds::new(point(guide_x, y), size(px(1.0), line_height)),
                    color,
                ));
            }

            // Content-hash-based cache: only re-shape lines whose content changed
            let line_hash = {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                line_text.hash(&mut hasher);
                hasher.finish()
            };
            let cached_layout = {
                let state = self.state.read(cx);
                if state.line_content_hashes.get(&line_idx) == Some(&line_hash) {
                    state.line_layouts.get(&line_idx).cloned()
                } else {
                    None
                }
            };
            if let Some(cached) = cached_layout {
                let line_width = cached.x_for_index(cached.len());
                if line_width > max_line_width {
                    max_line_width = line_width;
                }
                let _ = cached.paint(
                    point(bounds.left() + gutter_width - scroll_offset_x, y),
                    line_height,
                    window,
                    cx,
                );
                continue;
            }

            if line_text.is_empty() {
                shaped_layouts.push((line_idx, None, line_hash));
                continue;
            }

            let highlight_spans = &self.state.read(cx).cached_highlight_spans;
            let text_runs =
                self.build_text_runs(&line_text, line_idx, highlight_spans, &text_style, &theme);

            let line_len = line_text.len();
            let shaped =
                window
                    .text_system()
                    .shape_line(line_text.into(), font_size, &text_runs, None);

            let line_width = shaped.x_for_index(line_len);
            if line_width > max_line_width {
                max_line_width = line_width;
            }

            let _ = shaped.paint(
                point(bounds.left() + gutter_width - scroll_offset_x, y),
                line_height,
                window,
                cx,
            );

            shaped_layouts.push((line_idx, Some(shaped), line_hash));
        }

        self.state.update(cx, |state, _| {
            state
                .line_layouts
                .retain(|&line_idx, _| line_idx >= first_buf && line_idx < last_buf);
            state
                .line_content_hashes
                .retain(|&line_idx, _| line_idx >= first_buf && line_idx < last_buf);
            for (idx, layout, hash) in shaped_layouts {
                if let Some(shaped) = layout {
                    state.line_layouts.insert(idx, shaped);
                }
                state.line_content_hashes.insert(idx, hash);
            }
            if max_line_width > state.max_line_width {
                state.max_line_width = max_line_width;
            }
        });

        if show_line_numbers {
            window.paint_quad(PaintQuad {
                bounds: Bounds {
                    origin: bounds.origin,
                    size: Size {
                        width: gutter_width,
                        height: bounds.size.height,
                    },
                },
                corner_radii: Corners::default(),
                background: gutter_bg_color.into(),
                border_widths: Edges::default(),
                border_color: Hsla::transparent_black(),
                border_style: BorderStyle::default(),
                continuous_corners: false,
                transform: Default::default(),
                blend_mode: Default::default(),
            });

            let mut line_num_buf2 = String::with_capacity(8);
            for display_row in first_visible_display_row..last_visible_display_row {
                let line_idx = display_lines_vec[display_row];
                let y = bounds.top() + padding_top + line_height * display_row as f32;
                let is_current_line = line_idx == cursor.line;
                let num_color = if is_current_line && is_focused {
                    line_num_active_color
                } else {
                    line_num_color
                };
                line_num_buf2.clear();
                use std::fmt::Write;
                let _ = write!(line_num_buf2, "{:>4}", line_idx + 1);
                let num_font = if is_current_line && is_focused {
                    let mut f = text_style.font();
                    f.weight = FontWeight::BOLD;
                    f
                } else {
                    text_style.font()
                };
                let line_num_run = TextRun {
                    len: line_num_buf2.len(),
                    font: num_font,
                    color: num_color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let shaped = window.text_system().shape_line(
                    SharedString::from(line_num_buf2.clone()),
                    font_size,
                    &[line_num_run],
                    None,
                );
                let _ = shaped.paint(point(bounds.left() + px(6.0), y), line_height, window, cx);

                let fold_start = fold_ranges.iter().any(|f| f.start_line == line_idx);
                let is_folded = folded_ranges.iter().any(|f| f.start_line == line_idx);
                if fold_start {
                    let icon_name = if is_folded {
                        "chevron-right"
                    } else {
                        "chevron-down"
                    };
                    let icon_path = SharedString::from(resolve_icon_path(icon_name));
                    let icon_size = px(16.0);
                    let icon_x = bounds.left() + gutter_width - px(18.0);
                    let icon_y = y + (line_height - icon_size) / 2.0;
                    let icon_bounds =
                        Bounds::new(point(icon_x, icon_y), size(icon_size, icon_size));
                    let _ = window.paint_svg(
                        icon_bounds,
                        icon_path,
                        TransformationMatrix::default(),
                        fold_marker_color,
                        cx,
                    );
                }
            }
        }

        if is_focused && is_single_cursor {
            let word_occurrences = self.find_word_occurrences(visible_buffer_lines, cx);
            for (occ_line, occ_start, occ_end) in &word_occurrences {
                if let Some(dr) = buf_to_disp(*occ_line) {
                    if let Some(layout) = self.state.read(cx).line_layouts.get(occ_line) {
                        let occ_y = bounds.top() + padding_top + line_height * dr as f32;
                        let x_start = layout.x_for_index(*occ_start);
                        let x_end = layout.x_for_index(*occ_end);
                        window.paint_quad(fill(
                            Bounds::new(
                                point(
                                    bounds.left() + gutter_width + x_start - scroll_offset_x,
                                    occ_y,
                                ),
                                size(x_end - x_start, line_height),
                            ),
                            word_highlight_color,
                        ));
                    }
                }
            }
        }

        let sel_color = self
            .state
            .read(cx)
            .selection_color_override
            .unwrap_or(theme.tokens.primary.opacity(0.25));

        if let Some(selection) = &selection {
            let (start, end) = selection.range();
            for line_idx in start.line..=end.line {
                let dr = match buf_to_disp(line_idx) {
                    Some(d) => d,
                    None => continue,
                };
                if dr < first_visible_display_row || dr >= last_visible_display_row {
                    continue;
                }
                let line_y = bounds.top() + padding_top + line_height * dr as f32;
                let line_len = self.state.read(cx).line_len(line_idx);
                let start_col = if line_idx == start.line { start.col } else { 0 };
                let end_col = if line_idx == end.line {
                    end.col
                } else {
                    line_len
                };

                let (sel_x, sel_width) =
                    if let Some(layout) = self.state.read(cx).line_layouts.get(&line_idx) {
                        let x_start = layout.x_for_index(start_col);
                        let x_end = layout.x_for_index(end_col);
                        (
                            bounds.left() + gutter_width + x_start - scroll_offset_x,
                            x_end - x_start,
                        )
                    } else {
                        (bounds.left() + gutter_width - scroll_offset_x, px(0.0))
                    };

                window.paint_quad(fill(
                    Bounds::new(point(sel_x, line_y), size(sel_width, line_height)),
                    sel_color,
                ));
            }
        }

        {
            let state = self.state.read(cx);
            let (search_normal, search_active) = state
                .search_match_color_overrides
                .unwrap_or((rgba(0xFFD70040).into(), rgba(0xFF990060).into()));
            let current_match = state.current_match_idx;
            for (match_idx, &(match_start, match_end)) in state.search_matches.iter().enumerate() {
                let start_pos = state.byte_offset_to_pos(match_start);
                let end_pos = state.byte_offset_to_pos(match_end);
                let is_current = current_match == Some(match_idx);
                let color = if is_current {
                    search_active
                } else {
                    search_normal
                };

                for line_idx in start_pos.line..=end_pos.line {
                    let dr = match buf_to_disp(line_idx) {
                        Some(d) => d,
                        None => continue,
                    };
                    if dr < first_visible_display_row || dr >= last_visible_display_row {
                        continue;
                    }
                    let line_y = bounds.top() + padding_top + line_height * dr as f32;
                    let sc = if line_idx == start_pos.line {
                        start_pos.col
                    } else {
                        0
                    };
                    let ec = if line_idx == end_pos.line {
                        end_pos.col
                    } else {
                        state.line_len(line_idx)
                    };

                    let (hx, hw) = if let Some(layout) = state.line_layouts.get(&line_idx) {
                        let x_start = layout.x_for_index(sc);
                        let x_end = layout.x_for_index(ec);
                        (
                            bounds.left() + gutter_width + x_start - scroll_offset_x,
                            x_end - x_start,
                        )
                    } else {
                        continue;
                    };

                    window.paint_quad(fill(
                        Bounds::new(point(hx, line_y), size(hw, line_height)),
                        color,
                    ));
                }
            }
        }

        if is_focused {
            if let Some((pos_a, pos_b)) = self.state.read(cx).find_matching_bracket() {
                for pos in [pos_a, pos_b] {
                    if let Some(dr) = buf_to_disp(pos.line) {
                        if dr >= first_visible_display_row && dr < last_visible_display_row {
                            if let Some(layout) = self.state.read(cx).line_layouts.get(&pos.line) {
                                let bx = bounds.left() + gutter_width + layout.x_for_index(pos.col)
                                    - scroll_offset_x;
                                let by = bounds.top() + padding_top + line_height * dr as f32;
                                let bw =
                                    layout.x_for_index(pos.col + 1) - layout.x_for_index(pos.col);
                                let bracket_bounds =
                                    Bounds::new(point(bx, by), size(bw, line_height));
                                window.paint_quad(PaintQuad {
                                    bounds: bracket_bounds,
                                    corner_radii: Corners::default(),
                                    background: bracket_match_color.opacity(0.3).into(),
                                    border_widths: Edges::all(px(1.0)),
                                    border_color: bracket_match_color,
                                    border_style: BorderStyle::default(),
                                    continuous_corners: false,
                                    transform: Default::default(),
                                    blend_mode: Default::default(),
                                });
                            }
                        }
                    }
                }
            }
        }

        {
            let diagnostics = &self.state.read(cx).diagnostics;
            if !diagnostics.is_empty() {
                for diag in diagnostics {
                    let diag_line = diag.start_line as usize;
                    let dr = match buf_to_disp(diag_line) {
                        Some(d) => d,
                        None => continue,
                    };
                    if dr < first_visible_display_row || dr >= last_visible_display_row {
                        continue;
                    }

                    let underline_color = match diag.severity {
                        DiagnosticSeverity::Error => self
                            .state
                            .read(cx)
                            .diagnostic_error_color
                            .unwrap_or(hsla(0.0, 0.85, 0.6, 1.0)),
                        DiagnosticSeverity::Warning => self
                            .state
                            .read(cx)
                            .diagnostic_warning_color
                            .unwrap_or(hsla(0.12, 0.85, 0.55, 1.0)),
                        DiagnosticSeverity::Information => self
                            .state
                            .read(cx)
                            .diagnostic_info_color
                            .unwrap_or(hsla(0.6, 0.7, 0.6, 1.0)),
                        DiagnosticSeverity::Hint => self
                            .state
                            .read(cx)
                            .diagnostic_hint_color
                            .unwrap_or(hsla(0.0, 0.0, 0.5, 0.6)),
                    };

                    let diag_y =
                        bounds.top() + padding_top + line_height * dr as f32 + line_height - px(2.0);

                    if let Some(layout) = self.state.read(cx).line_layouts.get(&diag_line) {
                        let start_col = diag.start_col as usize;
                        let end_col = if diag.end_line == diag.start_line {
                            (diag.end_col as usize).max(start_col + 1)
                        } else {
                            self.state.read(cx).line_len(diag_line)
                        };
                        let x_start = layout.x_for_index(start_col);
                        let x_end = layout.x_for_index(end_col);
                        let underline_width = (x_end - x_start).max(char_width);
                        window.paint_quad(fill(
                            Bounds::new(
                                point(
                                    bounds.left() + gutter_width + x_start - scroll_offset_x,
                                    diag_y,
                                ),
                                size(underline_width, px(2.0)),
                            ),
                            underline_color,
                        ));
                    }

                    if show_line_numbers {
                        let dot_size = px(6.0);
                        let dot_x = bounds.left() + px(2.0);
                        let dot_y = bounds.top()
                            + padding_top
                            + line_height * dr as f32
                            + (line_height - dot_size) / 2.0;
                        window.paint_quad(PaintQuad {
                            bounds: Bounds::new(point(dot_x, dot_y), size(dot_size, dot_size)),
                            corner_radii: Corners::all(dot_size / 2.0),
                            background: underline_color.into(),
                            border_widths: Edges::default(),
                            border_color: Hsla::transparent_black(),
                            border_style: BorderStyle::default(),
                            continuous_corners: false,
                            transform: Default::default(),
                            blend_mode: Default::default(),
                        });
                    }
                }
            }
        }

        if is_focused {
            let cursor_moved = {
                let state = self.state.read(cx);
                state.last_blink_cursor != cursor
            };
            if cursor_moved {
                self.state.update(cx, |state, cx| {
                    state.last_blink_cursor = cursor;
                    state.reset_cursor_blink(cx);
                });
            } else if self.state.read(cx).blink_task.is_none() {
                self.state.update(cx, |state, cx| {
                    state.reset_cursor_blink(cx);
                });
            }

            let cursor_visible = self.state.read(cx).cursor_visible;
            if cursor_visible {
                if let Some(cursor_display_row) = buf_to_disp(cursor.line) {
                    let total = self.state.read(cx).total_lines();
                    let cursor_col = if cursor.line < total {
                        cursor.col.min(self.state.read(cx).line_len(cursor.line))
                    } else {
                        0
                    };
                    let cursor_y =
                        bounds.top() + padding_top + line_height * cursor_display_row as f32;
                    let cursor_x = if let Some(layout) =
                        self.state.read(cx).line_layouts.get(&cursor.line)
                    {
                        bounds.left() + gutter_width + layout.x_for_index(cursor_col)
                            - scroll_offset_x
                    } else {
                        bounds.left() + gutter_width - scroll_offset_x
                    };

                    let cursor_draw_color = self
                        .state
                        .read(cx)
                        .cursor_color_override
                        .unwrap_or(theme.tokens.primary);

                    window.paint_quad(fill(
                        Bounds::new(point(cursor_x, cursor_y), size(px(2.0), line_height)),
                        cursor_draw_color,
                    ));
                }
            }
        }
    }
}

struct HighlightSpan {
    line: usize,
    start_col: usize,
    end_col: usize,
    color: Hsla,
}

impl EditorElement {
    fn find_word_occurrences(
        &self,
        visible_lines: &[usize],
        cx: &App,
    ) -> Vec<(usize, usize, usize)> {
        let state = self.state.read(cx);
        let word = match state.word_under_cursor_full() {
            Some((w, _, _)) => w,
            None => return Vec::new(),
        };
        let mut results = Vec::new();
        for &line_idx in visible_lines {
            let line_text = state.line_text(line_idx);
            let mut search_from = 0;
            while let Some(pos) = line_text[search_from..].find(&word) {
                let abs_start = search_from + pos;
                let abs_end = abs_start + word.len();
                let before_ok = abs_start == 0
                    || !line_text.as_bytes()[abs_start - 1].is_ascii_alphanumeric()
                        && line_text.as_bytes()[abs_start - 1] != b'_';
                let after_ok = abs_end >= line_text.len()
                    || !line_text.as_bytes()[abs_end].is_ascii_alphanumeric()
                        && line_text.as_bytes()[abs_end] != b'_';
                if before_ok && after_ok {
                    results.push((line_idx, abs_start, abs_end));
                }
                search_from = abs_start + 1;
            }
        }
        results
    }

    fn collect_highlight_spans_for_lines(
        &self,
        visible_lines: &[usize],
        cx: &App,
    ) -> Vec<HighlightSpan> {
        if visible_lines.is_empty() {
            return Vec::new();
        }

        let state = self.state.read(cx);
        let tree = match &state.syntax_tree {
            Some(t) => t,
            None => return Vec::new(),
        };

        let query = match &state.highlight_query {
            Some(q) => q,
            None => return Vec::new(),
        };

        let rope = &state.rope;
        let total_lines = rope.len_lines();
        let mut spans = Vec::new();

        let mut chunk_start = 0usize;
        while chunk_start < visible_lines.len() {
            let mut chunk_end = chunk_start;
            while chunk_end + 1 < visible_lines.len()
                && visible_lines[chunk_end + 1] == visible_lines[chunk_end] + 1
            {
                chunk_end += 1;
            }

            let first_line = visible_lines[chunk_start];
            let last_line = visible_lines[chunk_end] + 1;

            let first_byte = rope.line_to_byte(first_line);
            let last_byte = if last_line < total_lines {
                rope.line_to_byte(last_line)
            } else {
                rope.len_bytes()
            };

            let mut cursor = QueryCursor::new();
            cursor.set_byte_range(first_byte..last_byte);

            let mut matches = cursor.matches(query, tree.root_node(), |node: tree_sitter::Node| {
                let range = node.byte_range();
                let text: String = rope
                    .byte_slice(range.start..range.end.min(rope.len_bytes()))
                    .into();
                std::iter::once(text)
            });

            while let Some(m) = matches.next() {
                for capture in m.captures {
                    let capture_name = &query.capture_names()[capture.index as usize];
                    let node = capture.node;
                    let start_byte = node.start_byte();
                    let end_byte = node.end_byte();
                    let color = if let Some(ref color_fn) = state.syntax_color_fn {
                        color_fn(capture_name)
                    } else {
                        highlight_color_for_capture(capture_name)
                    };

                    let start_line = rope.byte_to_line(start_byte);
                    let end_line =
                        rope.byte_to_line(end_byte.min(rope.len_bytes().saturating_sub(1)));

                    for line in start_line..=end_line {
                        if line < first_line || line >= last_line {
                            continue;
                        }
                        let line_start_byte = rope.line_to_byte(line);
                        let line_text = state.line_text(line);
                        let line_end_byte = line_start_byte + line_text.len();

                        let span_start = start_byte.max(line_start_byte) - line_start_byte;
                        let span_end = end_byte.min(line_end_byte) - line_start_byte;

                        if span_start < span_end {
                            spans.push(HighlightSpan {
                                line,
                                start_col: span_start,
                                end_col: span_end,
                                color,
                            });
                        }
                    }
                }
            }

            chunk_start = chunk_end + 1;
        }

        spans
    }

    fn build_text_runs(
        &self,
        line_text: &str,
        line_idx: usize,
        highlight_spans: &[HighlightSpan],
        text_style: &gpui::TextStyle,
        theme: &crate::theme::Theme,
    ) -> Vec<TextRun> {
        let mut line_spans: Vec<&HighlightSpan> = highlight_spans
            .iter()
            .filter(|s| s.line == line_idx)
            .collect();
        line_spans.sort_by_key(|s| s.start_col);

        if line_spans.is_empty() {
            return vec![TextRun {
                len: line_text.len(),
                font: text_style.font(),
                color: theme.tokens.foreground,
                background_color: None,
                underline: None,
                strikethrough: None,
            }];
        }

        let text_len = line_text.len();
        let mut runs = Vec::new();
        let mut pos = 0;

        for span in &line_spans {
            let start = span.start_col.min(text_len).max(pos);
            let end = span.end_col.min(text_len);
            if end <= start {
                continue;
            }
            if start > pos {
                runs.push(TextRun {
                    len: start - pos,
                    font: text_style.font(),
                    color: theme.tokens.foreground,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                });
            }
            runs.push(TextRun {
                len: end - start,
                font: text_style.font(),
                color: span.color,
                background_color: None,
                underline: None,
                strikethrough: None,
            });
            pos = end;
        }

        if pos < text_len {
            runs.push(TextRun {
                len: text_len - pos,
                font: text_style.font(),
                color: theme.tokens.foreground,
                background_color: None,
                underline: None,
                strikethrough: None,
            });
        }

        let total_len: usize = runs.iter().map(|r| r.len).sum();
        if runs.is_empty() || total_len != text_len {
            return vec![TextRun {
                len: text_len,
                font: text_style.font(),
                color: theme.tokens.foreground,
                background_color: None,
                underline: None,
                strikethrough: None,
            }];
        }

        runs
    }
}

#[derive(IntoElement)]
pub struct Editor {
    state: Entity<EditorState>,
    min_lines: Option<usize>,
    max_lines: Option<usize>,
    show_border: bool,
    style: StyleRefinement,
    cursor_color: Option<Hsla>,
    selection_color: Option<Hsla>,
    line_number_color: Option<Hsla>,
    line_number_active_color: Option<Hsla>,
    gutter_bg: Option<Hsla>,
    search_match_colors: Option<(Hsla, Hsla)>,
    current_line_color: Option<Hsla>,
    bracket_match_color: Option<Hsla>,
    word_highlight_color: Option<Hsla>,
    indent_guide_color: Option<Hsla>,
    indent_guide_active_color: Option<Hsla>,
    fold_marker_color: Option<Hsla>,
    syntax_color_fn: Option<Box<dyn Fn(&str) -> Hsla>>,
}

impl Editor {
    pub fn new(state: &Entity<EditorState>) -> Self {
        Self {
            state: state.clone(),
            min_lines: None,
            max_lines: None,
            show_border: true,
            style: StyleRefinement::default(),
            cursor_color: None,
            selection_color: None,
            line_number_color: None,
            line_number_active_color: None,
            gutter_bg: None,
            search_match_colors: None,
            current_line_color: None,
            bracket_match_color: None,
            word_highlight_color: None,
            indent_guide_color: None,
            indent_guide_active_color: None,
            fold_marker_color: None,
            syntax_color_fn: None,
        }
    }

    pub fn content(self, content: impl Into<String>, cx: &mut App) -> Self {
        self.state.update(cx, |state, cx| {
            state.set_content(&content.into(), cx);
        });
        self
    }

    pub fn min_lines(mut self, lines: usize) -> Self {
        self.min_lines = Some(lines);
        self
    }

    pub fn max_lines(mut self, lines: usize) -> Self {
        self.max_lines = Some(lines);
        self
    }

    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    pub fn show_line_numbers(self, show: bool, cx: &mut App) -> Self {
        self.state.update(cx, |state, cx| {
            state.show_line_numbers = show;
            cx.notify();
        });
        self
    }

    pub fn cursor_color(mut self, color: Hsla) -> Self {
        self.cursor_color = Some(color);
        self
    }

    pub fn selection_color(mut self, color: Hsla) -> Self {
        self.selection_color = Some(color);
        self
    }

    pub fn line_number_color(mut self, color: Hsla) -> Self {
        self.line_number_color = Some(color);
        self
    }

    pub fn line_number_active_color(mut self, color: Hsla) -> Self {
        self.line_number_active_color = Some(color);
        self
    }

    pub fn gutter_bg(mut self, color: Hsla) -> Self {
        self.gutter_bg = Some(color);
        self
    }

    pub fn search_match_colors(mut self, normal: Hsla, active: Hsla) -> Self {
        self.search_match_colors = Some((normal, active));
        self
    }

    pub fn current_line_color(mut self, color: Hsla) -> Self {
        self.current_line_color = Some(color);
        self
    }

    pub fn bracket_match_color(mut self, color: Hsla) -> Self {
        self.bracket_match_color = Some(color);
        self
    }

    pub fn word_highlight_color(mut self, color: Hsla) -> Self {
        self.word_highlight_color = Some(color);
        self
    }

    pub fn indent_guide_colors(mut self, normal: Hsla, active: Hsla) -> Self {
        self.indent_guide_color = Some(normal);
        self.indent_guide_active_color = Some(active);
        self
    }

    pub fn fold_marker_color(mut self, color: Hsla) -> Self {
        self.fold_marker_color = Some(color);
        self
    }

    pub fn syntax_color_fn(mut self, f: impl Fn(&str) -> Hsla + 'static) -> Self {
        self.syntax_color_fn = Some(Box::new(f));
        self
    }

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
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let syn_fn = self.syntax_color_fn.take();
        self.state.update(cx, |state, _| {
            state.cursor_color_override = self.cursor_color;
            state.selection_color_override = self.selection_color;
            state.line_number_color_override = self.line_number_color;
            state.line_number_active_color_override = self.line_number_active_color;
            state.gutter_bg_override = self.gutter_bg;
            state.search_match_color_overrides = self.search_match_colors;
            state.current_line_color_override = self.current_line_color;
            state.bracket_match_color_override = self.bracket_match_color;
            state.word_highlight_color_override = self.word_highlight_color;
            state.indent_guide_color_override = self.indent_guide_color;
            state.indent_guide_active_color_override = self.indent_guide_active_color;
            state.fold_marker_color_override = self.fold_marker_color;
            state.syntax_color_fn = syn_fn;
        });
        let theme = use_theme();
        let font_family_for_editor = self
            .state
            .read(cx)
            .font_family_override
            .clone()
            .unwrap_or_else(|| theme.tokens.font_mono.clone());
        let min_height = self.min_lines.map(|lines| px(lines as f32 * 20.0));
        let max_height = self.max_lines.map(|lines| px(lines as f32 * 20.0));
        let scroll_handle = self.state.read(cx).scroll_handle.clone();

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

        let styled_base = base
            .bg(theme.tokens.background)
            .rounded(theme.tokens.radius_md);

        let final_base = if self.show_border {
            styled_base.border_1().border_color(theme.tokens.border)
        } else {
            styled_base
        };

        let user_style = self.style;

        final_base
            .map(|this| {
                let mut d = this;
                d.style().refine(&user_style);
                d
            })
            .font_family(font_family_for_editor.clone())
            .on_action(window.listener_for(&self.state, EditorState::move_up))
            .on_action(window.listener_for(&self.state, EditorState::move_down))
            .on_action(window.listener_for(&self.state, EditorState::move_left))
            .on_action(window.listener_for(&self.state, EditorState::move_right))
            .on_action(window.listener_for(&self.state, EditorState::move_word_left))
            .on_action(window.listener_for(&self.state, EditorState::move_word_right))
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
            .on_action(window.listener_for(&self.state, EditorState::delete_word))
            .on_action(window.listener_for(&self.state, EditorState::enter))
            .on_action(window.listener_for(&self.state, EditorState::tab))
            .on_action(window.listener_for(&self.state, EditorState::copy))
            .on_action(window.listener_for(&self.state, EditorState::cut))
            .on_action(window.listener_for(&self.state, EditorState::paste))
            .on_action(window.listener_for(&self.state, EditorState::undo))
            .on_action(window.listener_for(&self.state, EditorState::redo))
            .on_mouse_down(MouseButton::Left, {
                let state = self.state.clone();
                move |event: &MouseDownEvent, window: &mut Window, cx: &mut App| {
                    let (bounds, gutter_width, line_height) = {
                        let s = state.read(cx);
                        let b = s.last_bounds.unwrap_or_default();
                        let gw = if s.show_line_numbers { px(80.0) } else { px(12.0) };
                        let lh = s.line_height;
                        (b, gw, lh)
                    };
                    state.update(cx, |s, cx| {
                        s.on_mouse_down(event, bounds, gutter_width, line_height, window, cx);
                    });
                    window.focus(&state.read(cx).focus_handle(cx));
                }
            })
            .on_mouse_move({
                let state = self.state.clone();
                move |event: &MouseMoveEvent, window: &mut Window, cx: &mut App| {
                    let (bounds, gutter_width, line_height) = {
                        let s = state.read(cx);
                        let b = s.last_bounds.unwrap_or_default();
                        let gw = if s.show_line_numbers { px(80.0) } else { px(12.0) };
                        let lh = s.line_height;
                        (b, gw, lh)
                    };
                    state.update(cx, |s, cx| {
                        s.on_mouse_move(event, bounds, gutter_width, line_height, window, cx);
                    });
                }
            })
            .on_mouse_up(
                MouseButton::Left,
                window.listener_for(&self.state, EditorState::on_mouse_up),
            )
            .on_scroll_wheel({
                let state = self.state.clone();
                move |event: &ScrollWheelEvent, _window: &mut Window, cx: &mut App| {
                    let delta_x = match event.delta {
                        ScrollDelta::Pixels(p) => p.x,
                        ScrollDelta::Lines(l) => px(l.x * 20.0),
                    };
                    if delta_x.abs() > px(0.5) {
                        state.update(cx, |s, cx| {
                            s.scroll_horizontal(-delta_x, cx);
                        });
                    }
                }
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .child(div().flex_1().overflow_hidden().child(
                        scrollable_vertical(self.state.clone())
                            .with_scroll_handle(scroll_handle),
                    ))
                    .child(HorizontalScrollbar::new(self.state.clone(), cx)),
            )
    }
}

struct HorizontalScrollbar {
    state: Entity<EditorState>,
    needs_scrollbar: bool,
    thumb_width_pct: f32,
    thumb_left_pct: f32,
}

impl HorizontalScrollbar {
    fn new(state: Entity<EditorState>, cx: &App) -> Self {
        let s = state.read(cx);
        let max_width = s.max_line_width;
        let scroll_x = s.scroll_offset_x;
        let viewport_bounds = s.scroll_handle.bounds();
        let gutter_width = if s.show_line_numbers {
            px(80.0)
        } else {
            px(12.0)
        };
        let content_width = viewport_bounds.size.width - gutter_width;
        let needs_scrollbar = max_width > content_width && content_width > px(0.0);

        let (thumb_width_pct, thumb_left_pct) = if needs_scrollbar {
            let visible_ratio = (content_width / max_width).min(1.0);
            let twp = (visible_ratio * 100.0).max(5.0);
            let scroll_range = max_width - content_width;
            let tlp = if scroll_range > px(0.0) {
                ((scroll_x / scroll_range) * (100.0 - twp)).max(0.0)
            } else {
                0.0
            };
            (twp, tlp)
        } else {
            (0.0, 0.0)
        };

        Self {
            state,
            needs_scrollbar,
            thumb_width_pct,
            thumb_left_pct,
        }
    }
}

impl IntoElement for HorizontalScrollbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        if !self.needs_scrollbar {
            return div().h(px(0.0)).into_any_element();
        }

        let theme = use_theme();
        let editor_state = self.state.clone();

        div()
            .id("h-scrollbar")
            .w_full()
            .h(px(12.0))
            .bg(theme.tokens.muted.opacity(0.3))
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, {
                let state = editor_state.clone();
                move |event: &MouseDownEvent, _window, cx| {
                    cx.stop_propagation();
                    state.update(cx, |s, cx| {
                        s.dragging_h_scrollbar = true;
                        let max_w = s.max_line_width;
                        let vp = s.scroll_handle.bounds();
                        let gw = if s.show_line_numbers {
                            px(80.0)
                        } else {
                            px(12.0)
                        };
                        let cw = vp.size.width - gw;
                        let scroll_range = max_w - cw;

                        if scroll_range > px(0.0) {
                            let track_width = vp.size.width;
                            let click_ratio = (event.position.x - vp.left()) / track_width;
                            let new_scroll = scroll_range * click_ratio;
                            s.scroll_offset_x = new_scroll.max(px(0.0)).min(scroll_range);
                        }
                        cx.notify();
                    });
                }
            })
            .on_mouse_up(MouseButton::Left, {
                let state = editor_state.clone();
                move |_: &MouseUpEvent, _window, cx| {
                    state.update(cx, |s, cx| {
                        s.dragging_h_scrollbar = false;
                        cx.notify();
                    });
                }
            })
            .child(
                div()
                    .absolute()
                    .top(px(2.0))
                    .bottom(px(2.0))
                    .left(relative(self.thumb_left_pct / 100.0))
                    .w(relative(self.thumb_width_pct / 100.0))
                    .bg(theme.tokens.muted_foreground.opacity(0.6))
                    .rounded(px(3.0))
                    .hover(|s| s.bg(theme.tokens.muted_foreground.opacity(0.8))),
            )
            .into_any_element()
    }
}

#[allow(dead_code)]
struct VerticalScrollbar {
    state: Entity<EditorState>,
    needs_scrollbar: bool,
    thumb_height_pct: f32,
    thumb_top_pct: f32,
}

impl VerticalScrollbar {
    #[allow(dead_code)]
    fn new(state: Entity<EditorState>, cx: &App) -> Self {
        let s = state.read(cx);
        let line_height = s.line_height;
        let padding = px(24.0);
        let num_lines = s.display_line_count();
        let content_height = padding + (line_height * num_lines as f32);
        let viewport_height = s.scroll_handle.bounds().size.height;
        let needs_scrollbar = content_height > viewport_height && viewport_height > px(0.0);

        let (thumb_height_pct, thumb_top_pct) = if needs_scrollbar {
            let visible_ratio = (viewport_height / content_height).min(1.0);
            let thp = (visible_ratio * 100.0).max(5.0);
            let scroll_y = -s.scroll_handle.offset().y;
            let overscroll = if viewport_height > line_height * 5.0 {
                viewport_height / 2.0
            } else {
                px(100.0)
            };
            let max_scroll = content_height + overscroll - viewport_height;
            let ttp = if max_scroll > px(0.0) {
                ((scroll_y / max_scroll) * (100.0 - thp)).max(0.0).min(100.0 - thp)
            } else {
                0.0
            };
            (thp, ttp)
        } else {
            (0.0, 0.0)
        };

        Self {
            state,
            needs_scrollbar,
            thumb_height_pct,
            thumb_top_pct,
        }
    }
}

impl IntoElement for VerticalScrollbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        if !self.needs_scrollbar {
            return div().w(px(0.0)).into_any_element();
        }

        let theme = use_theme();
        let editor_state = self.state.clone();

        div()
            .id("v-scrollbar")
            .h_full()
            .w(px(12.0))
            .bg(theme.tokens.muted.opacity(0.3))
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, {
                let state = editor_state.clone();
                move |event: &MouseDownEvent, _window, cx| {
                    cx.stop_propagation();
                    state.update(cx, |s, cx| {
                        let vp = s.scroll_handle.bounds();
                        let track_height = vp.size.height;
                        let click_ratio = (event.position.y - vp.top()) / track_height;

                        let line_height = s.line_height;
                        let padding = px(24.0);
                        let content_height =
                            padding + (line_height * s.display_line_count() as f32);
                        let overscroll = if track_height > line_height * 5.0 {
                            track_height / 2.0
                        } else {
                            px(100.0)
                        };
                        let max_scroll = content_height + overscroll - track_height;

                        if max_scroll > px(0.0) {
                            let new_scroll = max_scroll * click_ratio;
                            let offset = s.scroll_handle.offset();
                            s.scroll_handle.set_offset(point(
                                offset.x,
                                -new_scroll.max(px(0.0)).min(max_scroll),
                            ));
                        }
                        cx.notify();
                    });
                }
            })
            .child(
                div()
                    .absolute()
                    .left(px(2.0))
                    .right(px(2.0))
                    .top(relative(self.thumb_top_pct / 100.0))
                    .h(relative(self.thumb_height_pct / 100.0))
                    .bg(theme.tokens.muted_foreground.opacity(0.6))
                    .rounded(px(3.0))
                    .hover(|s| s.bg(theme.tokens.muted_foreground.opacity(0.8))),
            )
            .into_any_element()
    }
}
