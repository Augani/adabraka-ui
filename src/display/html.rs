#[cfg(feature = "html-render")]
use html5ever::parse_document;
#[cfg(feature = "html-render")]
use html5ever::tendril::TendrilSink;
#[cfg(feature = "html-render")]
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use gpui::*;

use crate::display::rich_text::LinkClickHandler;
#[cfg(feature = "html-render")]
use crate::display::rich_text::{render_blocks, ListItem, RichBlock, RichInline, TableAlignment};
#[cfg(feature = "html-render")]
use crate::theme::use_theme;

#[cfg(feature = "html-render")]
static ALLOWED_TAGS: &[&str] = &[
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "br",
    "hr",
    "strong",
    "b",
    "em",
    "i",
    "s",
    "strike",
    "del",
    "code",
    "pre",
    "a",
    "img",
    "ul",
    "ol",
    "li",
    "blockquote",
    "table",
    "thead",
    "tbody",
    "tr",
    "th",
    "td",
    "span",
    "div",
];

#[derive(IntoElement)]
pub struct Html {
    base: Div,
    source: SharedString,
    base_font_size: Option<Pixels>,
    on_link_click: Option<LinkClickHandler>,
}

impl Html {
    pub fn new(source: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            source: source.into(),
            base_font_size: None,
            on_link_click: None,
        }
    }

    pub fn base_font_size(mut self, size: Pixels) -> Self {
        self.base_font_size = Some(size);
        self
    }

    pub fn on_link_click(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_link_click = Some(Box::new(handler));
        self
    }
}

#[cfg(feature = "html-render")]
fn parse_html(source: &str) -> Vec<RichBlock> {
    let wrapped = format!("<html><body>{}</body></html>", source);
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut wrapped.as_bytes())
        .expect("html parse");

    let body = find_body(&dom.document);
    match body {
        Some(node) => walk_children_blocks(&node),
        None => Vec::new(),
    }
}

#[cfg(feature = "html-render")]
fn find_body(node: &Handle) -> Option<Handle> {
    match &node.data {
        NodeData::Element { name, .. } if name.local.as_ref() == "body" => {
            return Some(node.clone());
        }
        _ => {}
    }
    for child in node.children.borrow().iter() {
        if let Some(found) = find_body(child) {
            return Some(found);
        }
    }
    None
}

#[cfg(feature = "html-render")]
fn flush_inlines(pending: &mut Vec<RichInline>, blocks: &mut Vec<RichBlock>) {
    if !pending.is_empty() {
        let inlines = std::mem::take(pending);
        blocks.push(RichBlock::Paragraph(inlines));
    }
}

#[cfg(feature = "html-render")]
fn walk_children_blocks(node: &Handle) -> Vec<RichBlock> {
    let mut blocks = Vec::new();
    let mut pending_inlines: Vec<RichInline> = Vec::new();

    for child in node.children.borrow().iter() {
        match &child.data {
            NodeData::Element { name, .. } => {
                let tag = name.local.as_ref();
                if !is_allowed_tag(tag) {
                    continue;
                }
                match tag {
                    "p" | "div" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let inlines = walk_children_inlines(child);
                        if !inlines.is_empty() {
                            blocks.push(RichBlock::Paragraph(inlines));
                        }
                    }
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let level = tag.as_bytes()[1] - b'0';
                        let inlines = walk_children_inlines(child);
                        blocks.push(RichBlock::Heading {
                            level,
                            content: inlines,
                        });
                    }
                    "pre" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let (lang, code) = extract_code_block(child);
                        blocks.push(RichBlock::CodeBlock {
                            language: lang,
                            code,
                        });
                    }
                    "blockquote" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let inner = walk_children_blocks(child);
                        blocks.push(RichBlock::BlockQuote(inner));
                    }
                    "ul" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let items = walk_list_items(child);
                        blocks.push(RichBlock::UnorderedList { items });
                    }
                    "ol" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let start = get_attr(child, "start")
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(1);
                        let items = walk_list_items(child);
                        blocks.push(RichBlock::OrderedList { start, items });
                    }
                    "hr" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        blocks.push(RichBlock::HorizontalRule);
                    }
                    "table" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let table = walk_table(child);
                        blocks.push(table);
                    }
                    "img" => {
                        flush_inlines(&mut pending_inlines, &mut blocks);
                        let src = get_attr(child, "src").unwrap_or_default();
                        let alt = get_attr(child, "alt").unwrap_or_default();
                        blocks.push(RichBlock::Image { alt, url: src });
                    }
                    _ => {
                        let inlines = walk_children_inlines(child);
                        pending_inlines.extend(inlines);
                    }
                }
            }
            NodeData::Text { contents } => {
                let text = contents.borrow().to_string();
                if !text.trim().is_empty() {
                    pending_inlines.push(RichInline::Text(text));
                }
            }
            _ => {}
        }
    }

    flush_inlines(&mut pending_inlines, &mut blocks);
    blocks
}

#[cfg(feature = "html-render")]
fn walk_children_inlines(node: &Handle) -> Vec<RichInline> {
    let mut inlines = Vec::new();

    for child in node.children.borrow().iter() {
        match &child.data {
            NodeData::Text { contents } => {
                let text = contents.borrow().to_string();
                if !text.is_empty() {
                    inlines.push(RichInline::Text(text));
                }
            }
            NodeData::Element { name, attrs: _, .. } => {
                let tag = name.local.as_ref();
                if !is_allowed_tag(tag) {
                    continue;
                }
                match tag {
                    "strong" | "b" => {
                        let children = walk_children_inlines(child);
                        inlines.push(RichInline::Bold(children));
                    }
                    "em" | "i" => {
                        let children = walk_children_inlines(child);
                        inlines.push(RichInline::Italic(children));
                    }
                    "s" | "strike" | "del" => {
                        let children = walk_children_inlines(child);
                        inlines.push(RichInline::Strikethrough(children));
                    }
                    "code" => {
                        let text = collect_text(child);
                        inlines.push(RichInline::Code(text));
                    }
                    "a" => {
                        let href = get_attr(child, "href").unwrap_or_default();
                        let children = walk_children_inlines(child);
                        inlines.push(RichInline::Link {
                            text: children,
                            url: href,
                        });
                    }
                    "br" => {
                        inlines.push(RichInline::LineBreak);
                    }
                    "img" => {
                        let src = get_attr(child, "src").unwrap_or_default();
                        let alt = get_attr(child, "alt").unwrap_or_default();
                        inlines.push(RichInline::Image { alt, url: src });
                    }
                    "span" | "div" => {
                        let children = walk_children_inlines(child);
                        let style_attr = get_attr(child, "style");
                        if let Some(style_str) = style_attr {
                            let parsed = parse_inline_style(&style_str);
                            if parsed.has_any() {
                                inlines.push(RichInline::Styled {
                                    children,
                                    color: parsed.color,
                                    background_color: parsed.background_color,
                                    bold: parsed.bold,
                                    italic: parsed.italic,
                                    font_size: parsed.font_size,
                                });
                            } else {
                                inlines.extend(children);
                            }
                        } else {
                            inlines.extend(children);
                        }
                    }
                    _ => {
                        let children = walk_children_inlines(child);
                        inlines.extend(children);
                    }
                }
            }
            _ => {}
        }
    }

    inlines
}

#[cfg(feature = "html-render")]
fn extract_code_block(pre_node: &Handle) -> (Option<String>, String) {
    for child in pre_node.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            if name.local.as_ref() == "code" {
                let lang = get_attr(child, "class").and_then(|cls| {
                    cls.strip_prefix("language-")
                        .map(|l| l.to_string())
                        .or_else(|| cls.strip_prefix("lang-").map(|l| l.to_string()))
                });
                let code = collect_text(child);
                return (lang, code);
            }
        }
    }
    (None, collect_text(pre_node))
}

#[cfg(feature = "html-render")]
fn walk_list_items(node: &Handle) -> Vec<ListItem> {
    let mut items = Vec::new();

    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            if name.local.as_ref() == "li" {
                let inlines = walk_children_inlines(child);
                let mut sub_items = Vec::new();

                for sub in child.children.borrow().iter() {
                    if let NodeData::Element { name: sub_name, .. } = &sub.data {
                        let sub_tag = sub_name.local.as_ref();
                        if sub_tag == "ul" || sub_tag == "ol" {
                            sub_items = walk_list_items(sub);
                        }
                    }
                }

                items.push(ListItem {
                    checked: None,
                    content: inlines,
                    children: sub_items,
                });
            }
        }
    }

    items
}

#[cfg(feature = "html-render")]
fn walk_table(node: &Handle) -> RichBlock {
    let mut headers: Vec<Vec<RichInline>> = Vec::new();
    let mut rows: Vec<Vec<Vec<RichInline>>> = Vec::new();
    let mut alignments: Vec<TableAlignment> = Vec::new();

    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            match name.local.as_ref() {
                "thead" => {
                    for tr in child.children.borrow().iter() {
                        if let NodeData::Element { name: tr_name, .. } = &tr.data {
                            if tr_name.local.as_ref() == "tr" {
                                headers = walk_table_cells(tr, &mut alignments, true);
                            }
                        }
                    }
                }
                "tbody" => {
                    for tr in child.children.borrow().iter() {
                        if let NodeData::Element { name: tr_name, .. } = &tr.data {
                            if tr_name.local.as_ref() == "tr" {
                                let mut dummy = Vec::new();
                                let row = walk_table_cells(tr, &mut dummy, false);
                                rows.push(row);
                            }
                        }
                    }
                }
                "tr" => {
                    if headers.is_empty() {
                        headers = walk_table_cells(child, &mut alignments, true);
                    } else {
                        let mut dummy = Vec::new();
                        let row = walk_table_cells(child, &mut dummy, false);
                        rows.push(row);
                    }
                }
                _ => {}
            }
        }
    }

    RichBlock::Table {
        headers,
        alignments,
        rows,
    }
}

#[cfg(feature = "html-render")]
fn walk_table_cells(
    tr: &Handle,
    alignments: &mut Vec<TableAlignment>,
    is_header: bool,
) -> Vec<Vec<RichInline>> {
    let mut cells = Vec::new();

    for child in tr.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            let tag = name.local.as_ref();
            if tag == "th" || tag == "td" {
                let inlines = walk_children_inlines(child);
                cells.push(inlines);

                if is_header {
                    let align = get_attr(child, "align")
                        .map(|a| match a.as_str() {
                            "center" => TableAlignment::Center,
                            "right" => TableAlignment::Right,
                            _ => TableAlignment::Left,
                        })
                        .unwrap_or(TableAlignment::Left);
                    alignments.push(align);
                }
            }
        }
    }

    cells
}

#[cfg(feature = "html-render")]
fn collect_text(node: &Handle) -> String {
    let mut out = String::new();
    collect_text_recursive(node, &mut out);
    out
}

#[cfg(feature = "html-render")]
fn collect_text_recursive(node: &Handle, out: &mut String) {
    match &node.data {
        NodeData::Text { contents } => {
            out.push_str(&contents.borrow());
        }
        _ => {
            for child in node.children.borrow().iter() {
                collect_text_recursive(child, out);
            }
        }
    }
}

#[cfg(feature = "html-render")]
fn get_attr(node: &Handle, attr_name: &str) -> Option<String> {
    if let NodeData::Element { attrs, .. } = &node.data {
        for attr in attrs.borrow().iter() {
            if attr.name.local.as_ref() == attr_name {
                return Some(attr.value.to_string());
            }
        }
    }
    None
}

#[cfg(feature = "html-render")]
fn is_allowed_tag(tag: &str) -> bool {
    ALLOWED_TAGS.contains(&tag)
}

#[cfg(feature = "html-render")]
struct InlineStyle {
    color: Option<Hsla>,
    background_color: Option<Hsla>,
    bold: Option<bool>,
    italic: Option<bool>,
    font_size: Option<f32>,
}

#[cfg(feature = "html-render")]
impl InlineStyle {
    fn has_any(&self) -> bool {
        self.color.is_some()
            || self.background_color.is_some()
            || self.bold.is_some()
            || self.italic.is_some()
            || self.font_size.is_some()
    }
}

#[cfg(feature = "html-render")]
fn parse_inline_style(style: &str) -> InlineStyle {
    let mut result = InlineStyle {
        color: None,
        background_color: None,
        bold: None,
        italic: None,
        font_size: None,
    };

    for declaration in style.split(';') {
        let declaration = declaration.trim();
        if declaration.is_empty() {
            continue;
        }
        if let Some((prop, value)) = declaration.split_once(':') {
            let prop = prop.trim().to_lowercase();
            let value = value.trim();

            match prop.as_str() {
                "color" => {
                    result.color = parse_css_color(value);
                }
                "background-color" | "background" => {
                    result.background_color = parse_css_color(value);
                }
                "font-weight" => match value {
                    "bold" | "700" | "800" | "900" => result.bold = Some(true),
                    "normal" | "400" => result.bold = Some(false),
                    _ => {}
                },
                "font-style" => match value {
                    "italic" | "oblique" => result.italic = Some(true),
                    "normal" => result.italic = Some(false),
                    _ => {}
                },
                "font-size" => {
                    if let Some(stripped) = value.strip_suffix("px") {
                        if let Ok(size) = stripped.trim().parse::<f32>() {
                            result.font_size = Some(size);
                        }
                    } else if let Some(stripped) = value.strip_suffix("em") {
                        if let Ok(size) = stripped.trim().parse::<f32>() {
                            result.font_size = Some(size * 14.0);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    result
}

#[cfg(feature = "html-render")]
fn parse_css_color(value: &str) -> Option<Hsla> {
    let value = value.trim();

    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }

    if let Some(inner) = value
        .strip_prefix("rgb(")
        .or_else(|| value.strip_prefix("RGB("))
        .and_then(|s| s.strip_suffix(')'))
    {
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 3 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            return Some(rgb_to_hsla(r, g, b, 1.0));
        }
    }

    if let Some(inner) = value
        .strip_prefix("rgba(")
        .or_else(|| value.strip_prefix("RGBA("))
        .and_then(|s| s.strip_suffix(')'))
    {
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 4 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            let a = parts[3].trim().parse::<f32>().ok()?;
            return Some(rgb_to_hsla(r, g, b, a));
        }
    }

    match value.to_lowercase().as_str() {
        "red" => Some(rgb_to_hsla(255, 0, 0, 1.0)),
        "green" => Some(rgb_to_hsla(0, 128, 0, 1.0)),
        "blue" => Some(rgb_to_hsla(0, 0, 255, 1.0)),
        "yellow" => Some(rgb_to_hsla(255, 255, 0, 1.0)),
        "orange" => Some(rgb_to_hsla(255, 165, 0, 1.0)),
        "purple" => Some(rgb_to_hsla(128, 0, 128, 1.0)),
        "white" => Some(rgb_to_hsla(255, 255, 255, 1.0)),
        "black" => Some(rgb_to_hsla(0, 0, 0, 1.0)),
        "gray" | "grey" => Some(rgb_to_hsla(128, 128, 128, 1.0)),
        "cyan" => Some(rgb_to_hsla(0, 255, 255, 1.0)),
        "magenta" => Some(rgb_to_hsla(255, 0, 255, 1.0)),
        "pink" => Some(rgb_to_hsla(255, 192, 203, 1.0)),
        "brown" => Some(rgb_to_hsla(165, 42, 42, 1.0)),
        "coral" => Some(rgb_to_hsla(255, 127, 80, 1.0)),
        "teal" => Some(rgb_to_hsla(0, 128, 128, 1.0)),
        "navy" => Some(rgb_to_hsla(0, 0, 128, 1.0)),
        "maroon" => Some(rgb_to_hsla(128, 0, 0, 1.0)),
        "lime" => Some(rgb_to_hsla(0, 255, 0, 1.0)),
        "olive" => Some(rgb_to_hsla(128, 128, 0, 1.0)),
        "silver" => Some(rgb_to_hsla(192, 192, 192, 1.0)),
        "gold" => Some(rgb_to_hsla(255, 215, 0, 1.0)),
        _ => None,
    }
}

#[cfg(feature = "html-render")]
fn parse_hex_color(hex: &str) -> Option<Hsla> {
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(rgb_to_hsla(r, g, b, 1.0))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(rgb_to_hsla(r, g, b, 1.0))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(rgb_to_hsla(r, g, b, a as f32 / 255.0))
        }
        _ => None,
    }
}

#[cfg(feature = "html-render")]
fn rgb_to_hsla(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    if delta < f32::EPSILON {
        return hsla(0.0, 0.0, lightness, a);
    }

    let saturation = if lightness <= 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let hue = if (max - rf).abs() < f32::EPSILON {
        ((gf - bf) / delta + if gf < bf { 6.0 } else { 0.0 }) / 6.0
    } else if (max - gf).abs() < f32::EPSILON {
        ((bf - rf) / delta + 2.0) / 6.0
    } else {
        ((rf - gf) / delta + 4.0) / 6.0
    };

    hsla(hue, saturation, lightness, a)
}

#[cfg(feature = "html-render")]
impl RenderOnce for Html {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let base_size = self.base_font_size.unwrap_or(px(14.0));

        let blocks = parse_html(&self.source);
        let elements = render_blocks(&blocks, base_size, &self.on_link_click, "html");

        self.base
            .flex()
            .flex_col()
            .font_family(theme.tokens.font_family.clone())
            .text_color(theme.tokens.foreground)
            .children(elements)
    }
}

#[cfg(not(feature = "html-render"))]
impl RenderOnce for Html {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let _ = &self.source;
        self.base
            .child("Enable the 'html-render' feature to render HTML content.")
    }
}

impl Styled for Html {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}
