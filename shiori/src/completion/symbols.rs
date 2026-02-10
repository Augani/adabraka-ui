use adabraka_ui::components::editor::Language;
use std::collections::HashSet;
use tree_sitter::{Query, QueryCursor, StreamingIterator, Tree};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Variable,
    Struct,
    Enum,
    Const,
    Type,
    Field,
    Module,
    Class,
    Method,
}

impl SymbolKind {
    pub fn icon_name(&self) -> &'static str {
        match self {
            SymbolKind::Function | SymbolKind::Method => "function",
            SymbolKind::Variable | SymbolKind::Field => "variable",
            SymbolKind::Struct | SymbolKind::Class => "box",
            SymbolKind::Enum => "list",
            SymbolKind::Const => "lock",
            SymbolKind::Type => "type",
            SymbolKind::Module => "folder",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SymbolKind::Function => "fn",
            SymbolKind::Method => "method",
            SymbolKind::Variable => "var",
            SymbolKind::Field => "field",
            SymbolKind::Struct => "struct",
            SymbolKind::Class => "class",
            SymbolKind::Enum => "enum",
            SymbolKind::Const => "const",
            SymbolKind::Type => "type",
            SymbolKind::Module => "mod",
        }
    }
}

pub fn extract_symbols(tree: &Tree, source: &str, language: Language) -> Vec<Symbol> {
    let query_src = symbol_query_for_language(language);
    if query_src.is_empty() {
        return Vec::new();
    }

    let ts_lang = tree.language();

    let query = match Query::new(&ts_lang, query_src) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };

    let mut cursor = QueryCursor::new();
    let source_bytes = source.as_bytes();
    let mut seen = HashSet::new();
    let mut symbols = Vec::new();

    let mut matches = cursor.matches(&query, tree.root_node(), source_bytes);
    while let Some(m) = matches.next() {
        for cap in m.captures {
            let node = cap.node;
            let capture_name = query.capture_names()[cap.index as usize];
            let text: &str = match node.utf8_text(source_bytes) {
                Ok(n) => n,
                Err(_) => continue,
            };
            let name = text.to_string();

            if name.is_empty() || seen.contains(&name) {
                continue;
            }

            let kind = kind_from_capture(capture_name);
            seen.insert(name.clone());
            symbols.push(Symbol { name, kind });
        }
    }

    symbols.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    symbols
}

fn kind_from_capture(capture: &str) -> SymbolKind {
    match capture {
        "function" | "fn" => SymbolKind::Function,
        "method" => SymbolKind::Method,
        "variable" | "var" | "let" | "parameter" => SymbolKind::Variable,
        "struct" | "structure" => SymbolKind::Struct,
        "class" => SymbolKind::Class,
        "enum" => SymbolKind::Enum,
        "constant" | "const" => SymbolKind::Const,
        "type" | "typedef" => SymbolKind::Type,
        "field" | "property" => SymbolKind::Field,
        "module" | "namespace" => SymbolKind::Module,
        _ => SymbolKind::Variable,
    }
}

fn symbol_query_for_language(lang: Language) -> &'static str {
    match lang {
        Language::Rust => {
            r#"
            (function_item name: (identifier) @function)
            (let_declaration pattern: (identifier) @variable)
            (const_item name: (identifier) @constant)
            (static_item name: (identifier) @constant)
            (struct_item name: (type_identifier) @struct)
            (enum_item name: (type_identifier) @enum)
            (type_item name: (type_identifier) @type)
            (mod_item name: (identifier) @module)
            (impl_item type: (type_identifier) @type)
            (field_declaration name: (field_identifier) @field)
            "#
        }
        Language::TypeScript | Language::JavaScript => {
            r#"
            (function_declaration name: (identifier) @function)
            (variable_declarator name: (identifier) @variable)
            (class_declaration name: (identifier) @class)
            (method_definition name: (property_identifier) @method)
            (lexical_declaration (variable_declarator name: (identifier) @variable))
            "#
        }
        Language::Python => {
            r#"
            (function_definition name: (identifier) @function)
            (class_definition name: (identifier) @class)
            (assignment left: (identifier) @variable)
            "#
        }
        Language::Go => {
            r#"
            (function_declaration name: (identifier) @function)
            (method_declaration name: (field_identifier) @method)
            (type_declaration (type_spec name: (type_identifier) @type))
            (var_declaration (var_spec name: (identifier) @variable))
            (const_declaration (const_spec name: (identifier) @constant))
            "#
        }
        Language::C | Language::Cpp => {
            r#"
            (function_definition declarator: (function_declarator declarator: (identifier) @function))
            (declaration declarator: (init_declarator declarator: (identifier) @variable))
            (struct_specifier name: (type_identifier) @struct)
            (enum_specifier name: (type_identifier) @enum)
            (type_definition declarator: (type_identifier) @type)
            "#
        }
        Language::Java => {
            r#"
            (method_declaration name: (identifier) @method)
            (class_declaration name: (identifier) @class)
            (field_declaration declarator: (variable_declarator name: (identifier) @field))
            (local_variable_declaration declarator: (variable_declarator name: (identifier) @variable))
            "#
        }
        _ => "",
    }
}
