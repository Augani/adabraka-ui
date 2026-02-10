use crate::components::scrollable::scrollable_vertical;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use regex::Regex;
use ropey::Rope;
use smol::Timer;
use std::cmp::min;
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;
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

    fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            #[cfg(feature = "tree-sitter-rust")]
            Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-javascript")]
            Language::JavaScript => Some(tree_sitter_javascript::LANGUAGE.into()),
            #[cfg(feature = "tree-sitter-typescript")]
            Language::TypeScript => Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
            #[cfg(all(feature = "tree-sitter-javascript", not(feature = "tree-sitter-typescript")))]
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

    fn highlight_query_source(&self) -> Option<&'static str> {
        match self {
            #[cfg(feature = "tree-sitter-rust")]
            Language::Rust => Some(tree_sitter_rust::HIGHLIGHTS_QUERY),
            #[cfg(feature = "tree-sitter-javascript")]
            Language::JavaScript => Some(tree_sitter_javascript::HIGHLIGHT_QUERY),
            #[cfg(feature = "tree-sitter-typescript")]
            Language::TypeScript => Some(tree_sitter_typescript::HIGHLIGHTS_QUERY),
            #[cfg(all(feature = "tree-sitter-javascript", not(feature = "tree-sitter-typescript")))]
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

fn highlight_color_for_capture(capture_name: &str) -> Hsla {
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

        "type" | "type.builtin" | "type.definition" | "type.qualifier"
        | "storageclass" | "structure" => hsla(0.47, 0.60, 0.65, 1.0),

        "function" | "function.call" | "function.method" | "function.builtin"
        | "function.macro" | "method" | "method.call" | "constructor" => {
            hsla(0.58, 0.65, 0.70, 1.0)
        }

        "string" | "string.special" | "string.escape" | "string.regex"
        | "string.special.url" | "string.special.path" | "character"
        | "character.special" => hsla(0.25, 0.55, 0.60, 1.0),

        "number" | "float" | "constant.numeric" => hsla(0.08, 0.75, 0.65, 1.0),

        "comment" | "comment.line" | "comment.block" | "comment.documentation" => {
            hsla(0.0, 0.0, 0.45, 1.0)
        }

        "operator" => hsla(0.55, 0.50, 0.70, 1.0),

        "variable" | "variable.parameter" | "variable.builtin"
        | "variable.member" | "parameter" | "field" => hsla(0.0, 0.0, 0.85, 1.0),

        "constant" | "constant.builtin" | "constant.macro" | "boolean"
        | "define" | "symbol" => hsla(0.08, 0.75, 0.65, 1.0),

        "property" | "property.definition" => hsla(0.55, 0.50, 0.70, 1.0),

        "punctuation" | "punctuation.bracket" | "punctuation.delimiter"
        | "punctuation.special" => hsla(0.0, 0.0, 0.60, 1.0),

        "attribute" | "label" | "annotation" | "decorator" => hsla(0.12, 0.60, 0.65, 1.0),

        "namespace" | "module" => hsla(0.08, 0.50, 0.70, 1.0),

        "tag" | "tag.builtin" | "tag.delimiter" | "tag.attribute" => {
            hsla(0.0, 0.65, 0.65, 1.0)
        }

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

    parser: Parser,
    syntax_tree: Option<Tree>,
    highlight_query: Option<Query>,
    language: Language,

    scroll_handle: ScrollHandle,
    line_layouts: HashMap<usize, ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,

    is_selecting: bool,
    last_mouse_pos: Option<Point<Pixels>>,
    last_mouse_gutter_width: Pixels,
    autoscroll_task: Option<Task<()>>,
    last_click_time: Option<std::time::Instant>,

    marked_range: Option<Range<usize>>,

    pub show_line_numbers: bool,
    tab_size: usize,
    read_only: bool,

    reparse_task: Option<Task<()>>,

    search_query: String,
    search_matches: Vec<(usize, usize)>,
    current_match_idx: Option<usize>,
    search_case_sensitive: bool,
    search_use_regex: bool,
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
            parser,
            syntax_tree: None,
            highlight_query: None,
            language: Language::Plain,
            scroll_handle: ScrollHandle::new(),
            line_layouts: HashMap::new(),
            last_bounds: None,
            is_selecting: false,
            last_mouse_pos: None,
            last_mouse_gutter_width: px(60.0),
            autoscroll_task: None,
            last_click_time: None,
            marked_range: None,
            show_line_numbers: true,
            tab_size: 4,
            read_only: false,
            reparse_task: None,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match_idx: None,
            search_case_sensitive: false,
            search_use_regex: false,
        }
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
        self.line_layouts.clear();
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
                        self.line_layouts.clear();
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
                    this.update(cx, |state, cx| {
                        state.syntax_tree = tree;
                        state.line_layouts.clear();
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
                    state.line_layouts.clear();
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

        self.rope.insert(byte_offset, text);
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
        self.line_layouts.clear();
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

        self.rope.remove(start_offset..end_offset);
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
        self.line_layouts.clear();
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
                    self.rope.remove(*byte_offset..end);
                    self.cursor = self.byte_offset_to_pos(*byte_offset);
                    self.redo_stack.push(op);
                }
                EditOp::Delete { byte_offset, text } => {
                    self.rope.insert(*byte_offset, text);
                    self.cursor = self.byte_offset_to_pos(*byte_offset + text.len());
                    self.redo_stack.push(op);
                }
            }
            self.selection = None;
            self.mark_modified();
            self.update_syntax_tree();
            self.line_layouts.clear();
            cx.notify();
        }
    }

    pub fn redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(op) = self.redo_stack.pop() {
            match &op {
                EditOp::Insert { byte_offset, text } => {
                    self.rope.insert(*byte_offset, text);
                    self.cursor = self.byte_offset_to_pos(*byte_offset + text.len());
                    self.undo_stack.push(op);
                }
                EditOp::Delete { byte_offset, text } => {
                    let end = byte_offset + text.len();
                    self.rope.remove(*byte_offset..end);
                    self.cursor = self.byte_offset_to_pos(*byte_offset);
                    self.undo_stack.push(op);
                }
            }
            self.selection = None;
            self.mark_modified();
            self.update_syntax_tree();
            self.line_layouts.clear();
            cx.notify();
        }
    }

    pub fn move_up(&mut self, _: &MoveUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.clamp_cursor();
        }
        self.selection = None;
        cx.notify();
    }

    pub fn move_down(&mut self, _: &MoveDown, _: &mut Window, cx: &mut Context<Self>) {
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
        let del_start = offset - 1;
        let old_end_position = self.byte_to_ts_point(offset);
        let deleted: String = self.rope.byte_slice(del_start..offset).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: del_start,
            text: deleted,
        });
        self.redo_stack.clear();
        self.rope.remove(del_start..offset);
        self.mark_modified();
        self.cursor = self.byte_offset_to_pos(del_start);
        self.update_syntax_tree_incremental(del_start, offset, del_start, old_end_position, cx);
        self.line_layouts.clear();
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
        let del_end = min(offset + 1, self.rope.len_bytes());
        let old_end_position = self.byte_to_ts_point(del_end);
        let deleted: String = self.rope.byte_slice(offset..del_end).into();
        self.undo_stack.push(EditOp::Delete {
            byte_offset: offset,
            text: deleted,
        });
        self.redo_stack.clear();
        self.rope.remove(offset..del_end);
        self.mark_modified();
        self.update_syntax_tree_incremental(offset, del_end, offset, old_end_position, cx);
        self.line_layouts.clear();
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
        self.rope.remove(start_offset..end_offset);
        self.mark_modified();
        self.cursor = word_start;
        self.update_syntax_tree_incremental(
            start_offset,
            end_offset,
            start_offset,
            old_end_position,
            cx,
        );
        self.line_layouts.clear();
        cx.notify();
    }

    pub fn enter(&mut self, _: &Enter, _: &mut Window, cx: &mut Context<Self>) {
        if self.read_only {
            return;
        }
        self.insert_text_at_cursor("\n", cx);
        self.ensure_cursor_visible(cx);
    }

    pub fn tab(&mut self, _: &Tab, _: &mut Window, cx: &mut Context<Self>) {
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
        self.selection.as_ref().map(|sel| self.get_selection_text(sel))
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
        self.search_matches.clear();
        self.current_match_idx = None;

        if query.is_empty() {
            cx.notify();
            return;
        }

        let content = self.rope.to_string();

        if self.search_use_regex {
            let pattern = if self.search_case_sensitive {
                query.to_string()
            } else {
                format!("(?i){}", query)
            };
            if let Ok(re) = Regex::new(&pattern) {
                for m in re.find_iter(&content) {
                    self.search_matches.push((m.start(), m.end()));
                }
            }
        } else {
            let (haystack, needle) = if self.search_case_sensitive {
                (content.clone(), query.to_string())
            } else {
                (content.to_lowercase(), query.to_lowercase())
            };
            let needle_len = needle.len();
            let mut start = 0;
            while let Some(pos) = haystack[start..].find(&needle) {
                let match_start = start + pos;
                let match_end = match_start + needle_len;
                self.search_matches.push((match_start, match_end));
                start = match_start + 1;
            }
        }

        if !self.search_matches.is_empty() {
            let cursor_byte = self.pos_to_byte_offset(self.cursor);
            let idx = self.search_matches
                .iter()
                .position(|(s, _)| *s >= cursor_byte)
                .unwrap_or(0);
            self.current_match_idx = Some(idx);
            self.scroll_to_match(idx);
        }

        cx.notify();
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
        self.rope.remove(start..end);
        self.undo_stack.push(EditOp::Insert {
            byte_offset: start,
            text: replacement.to_string(),
        });
        self.rope.insert(start, replacement);
        self.redo_stack.clear();
        self.mark_modified();
        let new_end = start + replacement.len();
        self.update_syntax_tree_incremental(start, end, new_end, old_end_position, cx);
        self.line_layouts.clear();
        let query = self.search_query.clone();
        self.find_all(&query, cx);
    }

    pub fn replace_all(&mut self, replacement: &str, cx: &mut Context<Self>) {
        if self.read_only || self.search_matches.is_empty() {
            return;
        }
        for &(start, end) in self.search_matches.iter().rev() {
            let deleted: String = self.rope.byte_slice(start..end).into();
            self.undo_stack.push(EditOp::Delete {
                byte_offset: start,
                text: deleted,
            });
            self.rope.remove(start..end);
            self.undo_stack.push(EditOp::Insert {
                byte_offset: start,
                text: replacement.to_string(),
            });
            self.rope.insert(start, replacement);
        }
        self.redo_stack.clear();
        self.mark_modified();
        self.update_syntax_tree();
        self.line_layouts.clear();
        let query = self.search_query.clone();
        self.find_all(&query, cx);
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
        let target = line.saturating_sub(1).min(self.total_lines().saturating_sub(1));
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
        let line_height = px(20.0);
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
        } else if cursor_y + line_height > current_bottom {
            new_offset_y = -(cursor_y + line_height - viewport_height);
        }

        let max_offset = self.scroll_handle.max_offset().height;
        new_offset_y = new_offset_y.max(-max_offset).min(px(0.0));

        if (new_offset_y - offset.y).abs() > px(0.0) {
            self.scroll_handle.set_offset(point(offset.x, new_offset_y));
        }
        cx.notify();
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
        let line_f = (relative_y / line_height).floor();
        let line = if line_f < 0.0 {
            0
        } else {
            min(line_f as usize, self.total_lines().saturating_sub(1))
        };

        let relative_x = mouse_pos.x - bounds.left() - gutter_width;
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
        let line_height = px(20.0);
        self.autoscroll_task = Some(cx.spawn(async move |_, cx| {
            loop {
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
                                let speed =
                                    ((viewport_top + edge_zone - mouse_y) / edge_zone)
                                        .max(0.5)
                                        .min(5.0);
                                let offset = state.scroll_handle.offset();
                                let new_y = (offset.y + line_height * speed).min(px(0.0));
                                state.scroll_handle.set_offset(point(offset.x, new_y));
                                scrolled = true;
                            } else if mouse_y > viewport_bottom - edge_zone {
                                let speed = ((mouse_y - (viewport_bottom - edge_zone))
                                    / edge_zone)
                                    .max(0.5)
                                    .min(5.0);
                                let offset = state.scroll_handle.offset();
                                let max_offset = state.scroll_handle.max_offset().height;
                                let new_y =
                                    (offset.y - line_height * speed).max(-max_offset);
                                state.scroll_handle.set_offset(point(offset.x, new_y));
                                scrolled = true;
                            }

                            if scrolled {
                                let gutter_width = state.last_mouse_gutter_width;
                                let pos = state.position_for_mouse(
                                    mouse_pos, bounds, gutter_width, line_height,
                                );
                                if let Some(ref mut sel) = state.selection {
                                    sel.cursor = pos;
                                } else {
                                    state.selection =
                                        Some(Selection::new(state.cursor, pos));
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
        if !self.is_selecting {
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
                px(60.0)
            } else {
                px(12.0)
            };
            let line_height = px(20.0);
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
        let line_height = px(20.0);
        let padding_top = px(12.0);
        let padding_bottom = px(12.0);
        let num_lines = self.state.read(cx).total_lines();
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
        cx: &mut App,
    ) -> Self::PrepaintState {
        let show_line_numbers = self.state.read(cx).show_line_numbers;
        PrepaintState {
            gutter_width: if show_line_numbers {
                px(60.0)
            } else {
                px(12.0)
            },
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
        let line_height = prepaint.line_height;
        let gutter_width = prepaint.gutter_width;
        let font_size = px(14.0);

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

        let first_visible_line = ((-scroll_offset.y - padding_top) / line_height)
            .floor()
            .max(0.0) as usize;
        let visible_lines = ((viewport_height / line_height).ceil() as usize + 2).max(1);
        let total = self.state.read(cx).total_lines();
        let last_visible_line = min(first_visible_line + visible_lines, total);

        let (cursor, selection, show_line_numbers) = {
            let state = self.state.read(cx);
            (state.cursor, state.selection, state.show_line_numbers)
        };

        let highlight_spans =
            self.collect_highlight_spans(first_visible_line, last_visible_line, cx);

        let text_style = window.text_style();
        let mut shaped_layouts: Vec<(usize, Option<ShapedLine>)> =
            Vec::with_capacity(last_visible_line - first_visible_line);

        for line_idx in first_visible_line..last_visible_line {
            let y = bounds.top() + padding_top + line_height * line_idx as f32;
            let line_text = self.state.read(cx).line_text(line_idx);

            if show_line_numbers {
                let line_num_text = format!("{:>4}", line_idx + 1);
                let line_num_run = TextRun {
                    len: line_num_text.len(),
                    font: text_style.font(),
                    color: hsla(0.0, 0.0, 0.45, 1.0),
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
                let _ = shaped.paint(point(bounds.left() + px(6.0), y), line_height, window, cx);
            }

            if line_text.is_empty() {
                shaped_layouts.push((line_idx, None));
                continue;
            }

            let text_runs =
                self.build_text_runs(&line_text, line_idx, &highlight_spans, &text_style, &theme);

            let shaped =
                window
                    .text_system()
                    .shape_line(line_text.into(), font_size, &text_runs, None);

            let _ = shaped.paint(
                point(bounds.left() + gutter_width, y),
                line_height,
                window,
                cx,
            );

            shaped_layouts.push((line_idx, Some(shaped)));
        }

        self.state.update(cx, |state, _| {
            state.line_layouts.clear();
            for (idx, layout) in shaped_layouts {
                if let Some(shaped) = layout {
                    state.line_layouts.insert(idx, shaped);
                }
            }
        });

        if let Some(selection) = &selection {
            let (start, end) = selection.range();
            for line_idx in start.line..=end.line {
                if line_idx < first_visible_line || line_idx >= last_visible_line {
                    continue;
                }
                let line_y = bounds.top() + padding_top + line_height * line_idx as f32;
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
                        (bounds.left() + gutter_width + x_start, x_end - x_start)
                    } else {
                        (bounds.left() + gutter_width, px(0.0))
                    };

                window.paint_quad(fill(
                    Bounds::new(point(sel_x, line_y), size(sel_width, line_height)),
                    rgba(0x4444ff40),
                ));
            }
        }

        {
            let state = self.state.read(cx);
            let current_match = state.current_match_idx;
            for (match_idx, &(match_start, match_end)) in state.search_matches.iter().enumerate() {
                let start_pos = state.byte_offset_to_pos(match_start);
                let end_pos = state.byte_offset_to_pos(match_end);
                let is_current = current_match == Some(match_idx);
                let color = if is_current {
                    rgba(0xFF990060)
                } else {
                    rgba(0xFFD70040)
                };

                for line_idx in start_pos.line..=end_pos.line {
                    if line_idx < first_visible_line || line_idx >= last_visible_line {
                        continue;
                    }
                    let line_y = bounds.top() + padding_top + line_height * line_idx as f32;
                    let sc = if line_idx == start_pos.line { start_pos.col } else { 0 };
                    let ec = if line_idx == end_pos.line {
                        end_pos.col
                    } else {
                        state.line_len(line_idx)
                    };

                    let (hx, hw) = if let Some(layout) = state.line_layouts.get(&line_idx) {
                        let x_start = layout.x_for_index(sc);
                        let x_end = layout.x_for_index(ec);
                        (bounds.left() + gutter_width + x_start, x_end - x_start)
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

        if focus_handle.is_focused(window) {
            let cursor_col = if cursor.line < total {
                cursor.col.min(self.state.read(cx).line_len(cursor.line))
            } else {
                0
            };
            let cursor_y = bounds.top() + padding_top + line_height * cursor.line as f32;
            let cursor_x =
                if let Some(layout) = self.state.read(cx).line_layouts.get(&cursor.line) {
                    bounds.left() + gutter_width + layout.x_for_index(cursor_col)
                } else {
                    bounds.left() + gutter_width
                };

            window.paint_quad(fill(
                Bounds::new(point(cursor_x, cursor_y), size(px(2.0), line_height)),
                rgb(0x0099ff),
            ));
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
    fn collect_highlight_spans(
        &self,
        first_line: usize,
        last_line: usize,
        cx: &App,
    ) -> Vec<HighlightSpan> {
        let state = self.state.read(cx);
        let tree = match &state.syntax_tree {
            Some(t) => t,
            None => return Vec::new(),
        };

        let query = match &state.highlight_query {
            Some(q) => q,
            None => return Vec::new(),
        };

        let first_byte = state.rope.line_to_byte(first_line);
        let last_byte = if last_line < state.rope.len_lines() {
            state.rope.line_to_byte(last_line)
        } else {
            state.rope.len_bytes()
        };

        let mut cursor = QueryCursor::new();
        cursor.set_byte_range(first_byte..last_byte);

        let rope = &state.rope;

        let mut spans = Vec::new();
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
                let color = highlight_color_for_capture(capture_name);

                let start_line = state.rope.byte_to_line(start_byte);
                let end_line = state
                    .rope
                    .byte_to_line(end_byte.min(state.rope.len_bytes().saturating_sub(1)));

                for line in start_line..=end_line {
                    if line < first_line || line >= last_line {
                        continue;
                    }
                    let line_start_byte = state.rope.line_to_byte(line);
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
            .font_family(theme.tokens.font_mono.clone())
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
                    let bounds = state.read(cx).last_bounds.unwrap_or_default();
                    let gutter_width = if state.read(cx).show_line_numbers {
                        px(60.0)
                    } else {
                        px(12.0)
                    };
                    let line_height = px(20.0);
                    state.update(cx, |s, cx| {
                        s.on_mouse_down(event, bounds, gutter_width, line_height, window, cx);
                    });
                    window.focus(&state.read(cx).focus_handle(cx));
                }
            })
            .on_mouse_move({
                let state = self.state.clone();
                move |event: &MouseMoveEvent, window: &mut Window, cx: &mut App| {
                    let bounds = state.read(cx).last_bounds.unwrap_or_default();
                    let gutter_width = if state.read(cx).show_line_numbers {
                        px(60.0)
                    } else {
                        px(12.0)
                    };
                    let line_height = px(20.0);
                    state.update(cx, |s, cx| {
                        s.on_mouse_move(event, bounds, gutter_width, line_height, window, cx);
                    });
                }
            })
            .on_mouse_up(
                MouseButton::Left,
                window.listener_for(&self.state, EditorState::on_mouse_up),
            )
            .child(scrollable_vertical(self.state.clone()).with_scroll_handle(scroll_handle))
    }
}
