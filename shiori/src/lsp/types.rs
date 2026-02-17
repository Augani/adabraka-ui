use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range_start_line: u32,
    pub range_start_col: u32,
    pub range_end_line: u32,
    pub range_end_col: u32,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub contents: String,
    pub range: Option<(u32, u32, u32, u32)>,
}

#[derive(Debug, Clone)]
pub struct LocationInfo {
    pub path: PathBuf,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone)]
pub struct LspCompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub insert_text: String,
    pub kind: LspCompletionKind,
    pub sort_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspCompletionKind {
    Function,
    Variable,
    Field,
    Module,
    Struct,
    Enum,
    Keyword,
    Snippet,
    Method,
    Property,
    Constant,
    Class,
    Interface,
    Other,
}

impl LspCompletionKind {
    pub fn from_lsp_i32(kind: i32) -> Self {
        match kind {
            2 => Self::Method,
            3 => Self::Function,
            4 => Self::Function,
            5 => Self::Field,
            6 => Self::Variable,
            7 => Self::Class,
            8 => Self::Interface,
            9 => Self::Module,
            10 => Self::Property,
            13 => Self::Enum,
            14 => Self::Keyword,
            15 => Self::Snippet,
            21 => Self::Constant,
            22 => Self::Struct,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileDiagnostics {
    pub path: PathBuf,
    pub diagnostics: Vec<Diagnostic>,
}
