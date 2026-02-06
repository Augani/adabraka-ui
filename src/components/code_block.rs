use gpui::*;

use crate::theme::use_theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockCopyState {
    #[default]
    Idle,
    Copied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Keyword,
    StringLiteral,
    Comment,
    Number,
    Plain,
}

#[derive(IntoElement)]
pub struct CodeBlock {
    base: Div,
    code: SharedString,
    language: Option<SharedString>,
    show_line_numbers: bool,
    show_copy_button: bool,
    highlight_lines: Vec<usize>,
    max_height: Option<Pixels>,
}

impl CodeBlock {
    pub fn new(code: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            code: code.into(),
            language: None,
            show_line_numbers: true,
            show_copy_button: true,
            highlight_lines: Vec::new(),
            max_height: None,
        }
    }

    pub fn language(mut self, lang: impl Into<SharedString>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.highlight_lines = lines;
        self
    }

    pub fn max_height(mut self, height: Pixels) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn show_copy_button(mut self, show: bool) -> Self {
        self.show_copy_button = show;
        self
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let lines: Vec<&str> = self.code.split('\n').collect();
        let is_rust = self
            .language
            .as_ref()
            .map(|l| l.as_ref() == "rust" || l.as_ref() == "rs")
            .unwrap_or(false);

        let keyword_color = theme.tokens.primary;
        let string_color = hsla(0.4, 0.7, 0.5, 1.0);
        let comment_color = theme.tokens.muted_foreground;
        let number_color = hsla(0.08, 0.7, 0.6, 1.0);
        let plain_color = theme.tokens.foreground;
        let line_number_color = theme.tokens.muted_foreground;
        let highlight_bg = theme.tokens.muted.opacity(0.5);

        let gutter_width = px(40.0);

        let code_for_copy = self.code.clone();
        let show_copy = self.show_copy_button;

        let mut outer = self
            .base
            .relative()
            .bg(theme.tokens.muted.opacity(0.3))
            .rounded(theme.tokens.radius_md)
            .font_family(theme.tokens.font_mono.clone())
            .text_size(px(13.0))
            .overflow_hidden();

        let max_h = self.max_height;

        if show_copy {
            let copy_btn = div()
                .id("code-block-copy")
                .absolute()
                .top(px(8.0))
                .right(px(8.0))
                .px(px(8.0))
                .py(px(4.0))
                .rounded(theme.tokens.radius_sm)
                .bg(theme.tokens.muted.opacity(0.6))
                .text_color(theme.tokens.muted_foreground)
                .text_size(px(11.0))
                .cursor_pointer()
                .hover(|s| s.bg(theme.tokens.muted))
                .active(|s| s.opacity(0.7))
                .child("Copy")
                .on_click(move |_, _window, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(code_for_copy.to_string()));
                });
            outer = outer.child(copy_btn);
        }

        let mut content = div().flex().flex_col().py(px(12.0));

        for (idx, line_text) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let is_highlighted = self.highlight_lines.contains(&line_num);

            let mut row = div().flex().flex_row().px(px(12.0));

            if is_highlighted {
                row = row.bg(highlight_bg);
            }

            if self.show_line_numbers {
                row = row.child(
                    div()
                        .w(gutter_width)
                        .flex_shrink_0()
                        .text_color(line_number_color)
                        .text_size(px(12.0))
                        .text_right()
                        .pr(px(12.0))
                        .child(format!("{}", line_num)),
                );
            }

            let mut code_row = div().flex().flex_row().flex_1().min_w_0();
            let tokens = tokenize(line_text, is_rust);

            for (kind, text) in tokens {
                let color = match kind {
                    TokenKind::Keyword => keyword_color,
                    TokenKind::StringLiteral => string_color,
                    TokenKind::Comment => comment_color,
                    TokenKind::Number => number_color,
                    TokenKind::Plain => plain_color,
                };
                code_row = code_row.child(div().text_color(color).child(text.to_string()));
            }

            row = row.child(code_row);
            content = content.child(row);
        }

        if let Some(h) = max_h {
            outer.child(
                div()
                    .id("code-block-scroll")
                    .max_h(h)
                    .overflow_y_scroll()
                    .child(content),
            )
        } else {
            outer.child(content)
        }
    }
}

fn tokenize<'a>(line: &'a str, is_rust: bool) -> Vec<(TokenKind, &'a str)> {
    let mut tokens = Vec::new();
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    while pos < len {
        if pos + 1 < len && bytes[pos] == b'/' && bytes[pos + 1] == b'/' {
            tokens.push((TokenKind::Comment, &line[pos..]));
            return tokens;
        }

        if bytes[pos] == b'"' {
            let start = pos;
            pos += 1;
            while pos < len && bytes[pos] != b'"' {
                if bytes[pos] == b'\\' && pos + 1 < len {
                    pos += 1;
                }
                pos += 1;
            }
            if pos < len {
                pos += 1;
            }
            tokens.push((TokenKind::StringLiteral, &line[start..pos]));
            continue;
        }

        if bytes[pos] == b'\'' && is_rust {
            let start = pos;
            pos += 1;
            if pos < len && bytes[pos] == b'\\' && pos + 1 < len {
                pos += 2;
            } else if pos < len {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'\'' {
                pos += 1;
                tokens.push((TokenKind::StringLiteral, &line[start..pos]));
                continue;
            }
            pos = start + 1;
            tokens.push((TokenKind::Plain, &line[start..start + 1]));
            continue;
        }

        if bytes[pos].is_ascii_digit()
            || (bytes[pos] == b'-' && pos + 1 < len && bytes[pos + 1].is_ascii_digit())
        {
            let start = pos;
            if bytes[pos] == b'-' {
                pos += 1;
            }
            while pos < len
                && (bytes[pos].is_ascii_digit() || bytes[pos] == b'.' || bytes[pos] == b'_')
            {
                pos += 1;
            }
            if pos < len && (bytes[pos] == b'e' || bytes[pos] == b'E') {
                pos += 1;
                if pos < len && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                    pos += 1;
                }
                while pos < len && bytes[pos].is_ascii_digit() {
                    pos += 1;
                }
            }
            tokens.push((TokenKind::Number, &line[start..pos]));
            continue;
        }

        if bytes[pos].is_ascii_alphabetic() || bytes[pos] == b'_' {
            let start = pos;
            while pos < len && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                pos += 1;
            }
            let word = &line[start..pos];
            if is_rust && is_rust_keyword(word) {
                tokens.push((TokenKind::Keyword, word));
            } else {
                tokens.push((TokenKind::Plain, word));
            }
            continue;
        }

        if bytes[pos] == b' ' {
            let start = pos;
            while pos < len && bytes[pos] == b' ' {
                pos += 1;
            }
            tokens.push((TokenKind::Plain, &line[start..pos]));
            continue;
        }

        let start = pos;
        pos += 1;
        tokens.push((TokenKind::Plain, &line[start..pos]));
    }

    tokens
}

fn is_rust_keyword(word: &str) -> bool {
    matches!(
        word,
        "fn" | "let"
            | "mut"
            | "pub"
            | "struct"
            | "enum"
            | "impl"
            | "use"
            | "mod"
            | "if"
            | "else"
            | "for"
            | "while"
            | "match"
            | "return"
            | "self"
            | "Self"
            | "crate"
            | "super"
            | "true"
            | "false"
            | "async"
            | "await"
            | "move"
            | "ref"
            | "where"
            | "type"
            | "trait"
            | "const"
            | "static"
            | "loop"
            | "break"
            | "continue"
            | "in"
            | "as"
            | "unsafe"
            | "dyn"
            | "extern"
    )
}

impl Styled for CodeBlock {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for CodeBlock {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for CodeBlock {}

impl ParentElement for CodeBlock {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements)
    }
}
