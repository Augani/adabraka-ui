use gpui::{Hsla, Rgba};
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Debug)]
pub struct IdeTheme {
    pub name: &'static str,
    pub editor: EditorColors,
    pub syntax: SyntaxColors,
    pub terminal: TerminalColors,
    pub chrome: ChromeColors,
}

#[derive(Clone, Debug)]
pub struct EditorColors {
    pub cursor: Hsla,
    pub selection: Hsla,
    pub line_number: Hsla,
    pub line_number_active: Hsla,
    pub gutter_bg: Hsla,
    pub search_match: Hsla,
    pub search_match_active: Hsla,
}

#[derive(Clone, Debug)]
pub struct SyntaxColors {
    pub keyword: Hsla,
    pub type_name: Hsla,
    pub function: Hsla,
    pub string: Hsla,
    pub number: Hsla,
    pub comment: Hsla,
    pub operator: Hsla,
    pub variable: Hsla,
    pub constant: Hsla,
    pub property: Hsla,
    pub punctuation: Hsla,
    pub attribute: Hsla,
    pub namespace: Hsla,
    pub tag: Hsla,
    pub heading: Hsla,
    pub emphasis: Hsla,
    pub link: Hsla,
    pub literal: Hsla,
    pub embedded: Hsla,
    pub default_fg: Hsla,
}

#[derive(Clone, Debug)]
pub struct TerminalColors {
    pub palette: [Rgba; 16],
    pub fg: Rgba,
    pub bg: Rgba,
    pub cursor_color: Hsla,
    pub selection_color: Hsla,
}

#[derive(Clone, Debug)]
pub struct ChromeColors {
    pub bg: Hsla,
    pub header_bg: Hsla,
    pub header_border: Hsla,
    pub accent: Hsla,
    pub dim: Hsla,
    pub bright: Hsla,
    pub footer_bg: Hsla,
}

static IDE_THEME: LazyLock<Mutex<IdeTheme>> = LazyLock::new(|| Mutex::new(shiori_dark()));

pub fn use_ide_theme() -> IdeTheme {
    IDE_THEME.lock().unwrap().clone()
}

pub fn install_ide_theme(theme: IdeTheme) {
    *IDE_THEME.lock().unwrap() = theme;
}

pub fn all_ide_themes() -> Vec<IdeTheme> {
    vec![
        shiori_dark(),
        shiori_midnight(),
        shiori_light(),
        shiori_warm(),
    ]
}

fn rgba_from_hex(hex: u32) -> Rgba {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla { h, s, l, a }
}

pub fn shiori_dark() -> IdeTheme {
    IdeTheme {
        name: "Shiori Dark",
        editor: EditorColors {
            cursor: hsla(0.58, 0.90, 0.65, 1.0),
            selection: hsla(0.62, 0.50, 0.40, 0.35),
            line_number: hsla(0.63, 0.16, 0.40, 1.0),
            line_number_active: hsla(0.63, 0.16, 0.65, 1.0),
            gutter_bg: hsla(0.67, 0.30, 0.10, 1.0),
            search_match: hsla(0.13, 0.80, 0.50, 0.30),
            search_match_active: hsla(0.07, 0.90, 0.55, 0.45),
        },
        syntax: SyntaxColors {
            keyword: hsla(0.77, 0.75, 0.70, 1.0),
            type_name: hsla(0.47, 0.60, 0.65, 1.0),
            function: hsla(0.58, 0.65, 0.70, 1.0),
            string: hsla(0.25, 0.55, 0.60, 1.0),
            number: hsla(0.08, 0.75, 0.65, 1.0),
            comment: hsla(0.63, 0.16, 0.42, 1.0),
            operator: hsla(0.55, 0.50, 0.70, 1.0),
            variable: hsla(0.0, 0.0, 0.85, 1.0),
            constant: hsla(0.08, 0.75, 0.65, 1.0),
            property: hsla(0.55, 0.50, 0.70, 1.0),
            punctuation: hsla(0.0, 0.0, 0.55, 1.0),
            attribute: hsla(0.12, 0.60, 0.65, 1.0),
            namespace: hsla(0.08, 0.50, 0.70, 1.0),
            tag: hsla(0.0, 0.65, 0.65, 1.0),
            heading: hsla(0.58, 0.65, 0.80, 1.0),
            emphasis: hsla(0.25, 0.55, 0.70, 1.0),
            link: hsla(0.55, 0.60, 0.65, 1.0),
            literal: hsla(0.25, 0.55, 0.60, 1.0),
            embedded: hsla(0.0, 0.0, 0.80, 1.0),
            default_fg: hsla(0.0, 0.0, 0.85, 1.0),
        },
        terminal: TerminalColors {
            palette: [
                rgba_from_hex(0x15161e),
                rgba_from_hex(0xf7768e),
                rgba_from_hex(0x9ece6a),
                rgba_from_hex(0xe0af68),
                rgba_from_hex(0x7aa2f7),
                rgba_from_hex(0xbb9af7),
                rgba_from_hex(0x7dcfff),
                rgba_from_hex(0xa9b1d6),
                rgba_from_hex(0x414868),
                rgba_from_hex(0xf7768e),
                rgba_from_hex(0x9ece6a),
                rgba_from_hex(0xe0af68),
                rgba_from_hex(0x7aa2f7),
                rgba_from_hex(0xbb9af7),
                rgba_from_hex(0x7dcfff),
                rgba_from_hex(0xc0caf5),
            ],
            fg: rgba_from_hex(0xc0caf5),
            bg: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            cursor_color: hsla(0.58, 0.90, 0.65, 1.0),
            selection_color: hsla(0.58, 0.6, 0.5, 0.4),
        },
        chrome: ChromeColors {
            bg: hsla(0.67, 0.30, 0.12, 1.0),
            header_bg: hsla(0.67, 0.35, 0.08, 1.0),
            header_border: hsla(0.63, 0.25, 0.20, 1.0),
            accent: hsla(0.62, 0.89, 0.71, 1.0),
            dim: hsla(0.63, 0.16, 0.46, 1.0),
            bright: hsla(0.63, 0.50, 0.85, 1.0),
            footer_bg: hsla(0.67, 0.35, 0.08, 1.0),
        },
    }
}

pub fn shiori_midnight() -> IdeTheme {
    IdeTheme {
        name: "Shiori Midnight",
        editor: EditorColors {
            cursor: hsla(0.33, 0.90, 0.55, 1.0),
            selection: hsla(0.58, 0.60, 0.40, 0.30),
            line_number: hsla(0.0, 0.0, 0.30, 1.0),
            line_number_active: hsla(0.0, 0.0, 0.60, 1.0),
            gutter_bg: hsla(0.63, 0.20, 0.06, 1.0),
            search_match: hsla(0.15, 0.90, 0.50, 0.30),
            search_match_active: hsla(0.07, 0.95, 0.55, 0.50),
        },
        syntax: SyntaxColors {
            keyword: hsla(0.95, 0.70, 0.68, 1.0),
            type_name: hsla(0.55, 0.70, 0.68, 1.0),
            function: hsla(0.72, 0.70, 0.75, 1.0),
            string: hsla(0.30, 0.55, 0.60, 1.0),
            number: hsla(0.55, 0.70, 0.75, 1.0),
            comment: hsla(0.0, 0.0, 0.38, 1.0),
            operator: hsla(0.55, 0.60, 0.70, 1.0),
            variable: hsla(0.0, 0.0, 0.88, 1.0),
            constant: hsla(0.55, 0.70, 0.75, 1.0),
            property: hsla(0.58, 0.50, 0.68, 1.0),
            punctuation: hsla(0.0, 0.0, 0.50, 1.0),
            attribute: hsla(0.10, 0.70, 0.68, 1.0),
            namespace: hsla(0.95, 0.50, 0.70, 1.0),
            tag: hsla(0.33, 0.65, 0.60, 1.0),
            heading: hsla(0.58, 0.70, 0.80, 1.0),
            emphasis: hsla(0.30, 0.55, 0.70, 1.0),
            link: hsla(0.55, 0.65, 0.65, 1.0),
            literal: hsla(0.30, 0.55, 0.60, 1.0),
            embedded: hsla(0.0, 0.0, 0.82, 1.0),
            default_fg: hsla(0.0, 0.0, 0.88, 1.0),
        },
        terminal: TerminalColors {
            palette: [
                rgba_from_hex(0x0d1117),
                rgba_from_hex(0xff7b72),
                rgba_from_hex(0x7ee787),
                rgba_from_hex(0xd29922),
                rgba_from_hex(0x79c0ff),
                rgba_from_hex(0xd2a8ff),
                rgba_from_hex(0xa5d6ff),
                rgba_from_hex(0xc9d1d9),
                rgba_from_hex(0x484f58),
                rgba_from_hex(0xff7b72),
                rgba_from_hex(0x7ee787),
                rgba_from_hex(0xd29922),
                rgba_from_hex(0x79c0ff),
                rgba_from_hex(0xd2a8ff),
                rgba_from_hex(0xa5d6ff),
                rgba_from_hex(0xf0f6fc),
            ],
            fg: rgba_from_hex(0xc9d1d9),
            bg: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            cursor_color: hsla(0.33, 0.90, 0.55, 1.0),
            selection_color: hsla(0.58, 0.6, 0.5, 0.35),
        },
        chrome: ChromeColors {
            bg: hsla(0.61, 0.20, 0.07, 1.0),
            header_bg: hsla(0.61, 0.20, 0.05, 1.0),
            header_border: hsla(0.61, 0.15, 0.15, 1.0),
            accent: hsla(0.33, 0.80, 0.65, 1.0),
            dim: hsla(0.61, 0.10, 0.40, 1.0),
            bright: hsla(0.0, 0.0, 0.85, 1.0),
            footer_bg: hsla(0.61, 0.20, 0.05, 1.0),
        },
    }
}

pub fn shiori_light() -> IdeTheme {
    IdeTheme {
        name: "Shiori Light",
        editor: EditorColors {
            cursor: hsla(0.58, 0.90, 0.45, 1.0),
            selection: hsla(0.58, 0.50, 0.70, 0.30),
            line_number: hsla(0.0, 0.0, 0.55, 1.0),
            line_number_active: hsla(0.0, 0.0, 0.30, 1.0),
            gutter_bg: hsla(0.0, 0.0, 0.96, 1.0),
            search_match: hsla(0.15, 0.90, 0.60, 0.30),
            search_match_active: hsla(0.07, 0.95, 0.55, 0.45),
        },
        syntax: SyntaxColors {
            keyword: hsla(0.77, 0.65, 0.45, 1.0),
            type_name: hsla(0.47, 0.55, 0.40, 1.0),
            function: hsla(0.58, 0.60, 0.42, 1.0),
            string: hsla(0.25, 0.60, 0.38, 1.0),
            number: hsla(0.08, 0.70, 0.45, 1.0),
            comment: hsla(0.0, 0.0, 0.55, 1.0),
            operator: hsla(0.55, 0.45, 0.42, 1.0),
            variable: hsla(0.0, 0.0, 0.20, 1.0),
            constant: hsla(0.08, 0.70, 0.45, 1.0),
            property: hsla(0.55, 0.45, 0.42, 1.0),
            punctuation: hsla(0.0, 0.0, 0.40, 1.0),
            attribute: hsla(0.12, 0.55, 0.45, 1.0),
            namespace: hsla(0.08, 0.45, 0.45, 1.0),
            tag: hsla(0.0, 0.60, 0.45, 1.0),
            heading: hsla(0.58, 0.60, 0.40, 1.0),
            emphasis: hsla(0.25, 0.55, 0.42, 1.0),
            link: hsla(0.55, 0.55, 0.42, 1.0),
            literal: hsla(0.25, 0.60, 0.38, 1.0),
            embedded: hsla(0.0, 0.0, 0.25, 1.0),
            default_fg: hsla(0.0, 0.0, 0.20, 1.0),
        },
        terminal: TerminalColors {
            palette: [
                rgba_from_hex(0xf5f5f5),
                rgba_from_hex(0xd32f2f),
                rgba_from_hex(0x388e3c),
                rgba_from_hex(0xf57f17),
                rgba_from_hex(0x1976d2),
                rgba_from_hex(0x7b1fa2),
                rgba_from_hex(0x0097a7),
                rgba_from_hex(0x37474f),
                rgba_from_hex(0x90a4ae),
                rgba_from_hex(0xe53935),
                rgba_from_hex(0x43a047),
                rgba_from_hex(0xfb8c00),
                rgba_from_hex(0x1e88e5),
                rgba_from_hex(0x8e24aa),
                rgba_from_hex(0x00acc1),
                rgba_from_hex(0x263238),
            ],
            fg: rgba_from_hex(0x263238),
            bg: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            cursor_color: hsla(0.58, 0.90, 0.45, 1.0),
            selection_color: hsla(0.58, 0.5, 0.6, 0.35),
        },
        chrome: ChromeColors {
            bg: hsla(0.0, 0.0, 0.97, 1.0),
            header_bg: hsla(0.0, 0.0, 0.94, 1.0),
            header_border: hsla(0.0, 0.0, 0.85, 1.0),
            accent: hsla(0.58, 0.70, 0.45, 1.0),
            dim: hsla(0.0, 0.0, 0.55, 1.0),
            bright: hsla(0.0, 0.0, 0.15, 1.0),
            footer_bg: hsla(0.0, 0.0, 0.94, 1.0),
        },
    }
}

pub fn shiori_warm() -> IdeTheme {
    IdeTheme {
        name: "Shiori Warm",
        editor: EditorColors {
            cursor: hsla(0.08, 0.90, 0.60, 1.0),
            selection: hsla(0.08, 0.40, 0.40, 0.30),
            line_number: hsla(0.08, 0.15, 0.38, 1.0),
            line_number_active: hsla(0.08, 0.25, 0.60, 1.0),
            gutter_bg: hsla(0.06, 0.15, 0.12, 1.0),
            search_match: hsla(0.13, 0.80, 0.50, 0.30),
            search_match_active: hsla(0.07, 0.90, 0.55, 0.45),
        },
        syntax: SyntaxColors {
            keyword: hsla(0.03, 0.70, 0.65, 1.0),
            type_name: hsla(0.12, 0.60, 0.65, 1.0),
            function: hsla(0.15, 0.65, 0.70, 1.0),
            string: hsla(0.25, 0.50, 0.58, 1.0),
            number: hsla(0.08, 0.75, 0.65, 1.0),
            comment: hsla(0.08, 0.12, 0.42, 1.0),
            operator: hsla(0.08, 0.45, 0.65, 1.0),
            variable: hsla(0.08, 0.08, 0.82, 1.0),
            constant: hsla(0.08, 0.75, 0.65, 1.0),
            property: hsla(0.08, 0.45, 0.65, 1.0),
            punctuation: hsla(0.08, 0.10, 0.52, 1.0),
            attribute: hsla(0.12, 0.55, 0.62, 1.0),
            namespace: hsla(0.03, 0.50, 0.65, 1.0),
            tag: hsla(0.03, 0.60, 0.60, 1.0),
            heading: hsla(0.08, 0.65, 0.75, 1.0),
            emphasis: hsla(0.12, 0.55, 0.68, 1.0),
            link: hsla(0.55, 0.55, 0.60, 1.0),
            literal: hsla(0.25, 0.50, 0.58, 1.0),
            embedded: hsla(0.08, 0.08, 0.78, 1.0),
            default_fg: hsla(0.08, 0.08, 0.82, 1.0),
        },
        terminal: TerminalColors {
            palette: [
                rgba_from_hex(0x1d2021),
                rgba_from_hex(0xcc241d),
                rgba_from_hex(0x98971a),
                rgba_from_hex(0xd79921),
                rgba_from_hex(0x458588),
                rgba_from_hex(0xb16286),
                rgba_from_hex(0x689d6a),
                rgba_from_hex(0xa89984),
                rgba_from_hex(0x928374),
                rgba_from_hex(0xfb4934),
                rgba_from_hex(0xb8bb26),
                rgba_from_hex(0xfabd2f),
                rgba_from_hex(0x83a598),
                rgba_from_hex(0xd3869b),
                rgba_from_hex(0x8ec07c),
                rgba_from_hex(0xebdbb2),
            ],
            fg: rgba_from_hex(0xebdbb2),
            bg: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            cursor_color: hsla(0.08, 0.90, 0.60, 1.0),
            selection_color: hsla(0.08, 0.5, 0.45, 0.4),
        },
        chrome: ChromeColors {
            bg: hsla(0.06, 0.15, 0.13, 1.0),
            header_bg: hsla(0.06, 0.18, 0.09, 1.0),
            header_border: hsla(0.06, 0.12, 0.22, 1.0),
            accent: hsla(0.08, 0.80, 0.60, 1.0),
            dim: hsla(0.08, 0.12, 0.45, 1.0),
            bright: hsla(0.10, 0.30, 0.82, 1.0),
            footer_bg: hsla(0.06, 0.18, 0.09, 1.0),
        },
    }
}

impl SyntaxColors {
    pub fn color_for_capture(&self, capture_name: &str) -> Hsla {
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
            | "exception" => self.keyword,

            "type" | "type.builtin" | "type.definition" | "type.qualifier" | "storageclass"
            | "structure" => self.type_name,

            "function" | "function.call" | "function.method" | "function.builtin"
            | "function.macro" | "method" | "method.call" | "constructor" => self.function,

            "string"
            | "string.special"
            | "string.escape"
            | "string.regex"
            | "string.special.url"
            | "string.special.path"
            | "character"
            | "character.special" => self.string,

            "number" | "float" | "constant.numeric" => self.number,

            "comment" | "comment.line" | "comment.block" | "comment.documentation" => self.comment,

            "operator" => self.operator,

            "variable" | "variable.parameter" | "variable.builtin" | "variable.member"
            | "parameter" | "field" => self.variable,

            "constant" | "constant.builtin" | "constant.macro" | "boolean" | "define"
            | "symbol" => self.constant,

            "property" | "property.definition" => self.property,

            "punctuation"
            | "punctuation.bracket"
            | "punctuation.delimiter"
            | "punctuation.special" => self.punctuation,

            "attribute" | "label" | "annotation" | "decorator" => self.attribute,

            "namespace" | "module" => self.namespace,

            "tag" | "tag.builtin" | "tag.delimiter" | "tag.attribute" => self.tag,

            "text.title" | "markup.heading" | "text.strong" | "markup.bold" => self.heading,

            "text.emphasis" | "markup.italic" => self.emphasis,

            "text.uri" | "markup.link.url" | "markup.link" => self.link,

            "text.literal" | "markup.raw" => self.literal,

            "embedded" | "injection.content" => self.embedded,

            _ => self.default_fg,
        }
    }
}
