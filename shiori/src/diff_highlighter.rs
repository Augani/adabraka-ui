use adabraka_ui::components::editor::{highlight_color_for_capture, Language};
use gpui::Hsla;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

#[derive(Clone, Debug)]
pub struct HighlightRun {
    pub start: usize,
    pub len: usize,
    pub color: Hsla,
}

pub fn compute_line_highlights(content: &str, language: Language) -> Vec<Vec<HighlightRun>> {
    let ts_lang = match language.tree_sitter_language() {
        Some(l) => l,
        None => {
            return vec![Vec::new(); content.lines().count().max(1)];
        }
    };

    let query_src = match language.highlight_query_source() {
        Some(s) if !s.is_empty() => s,
        _ => {
            return vec![Vec::new(); content.lines().count().max(1)];
        }
    };

    let query = match Query::new(&ts_lang, query_src) {
        Ok(q) => q,
        Err(_) => {
            return vec![Vec::new(); content.lines().count().max(1)];
        }
    };

    let mut parser = Parser::new();
    if parser.set_language(&ts_lang).is_err() {
        return vec![Vec::new(); content.lines().count().max(1)];
    }

    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => {
            return vec![Vec::new(); content.lines().count().max(1)];
        }
    };

    let line_offsets = compute_line_offsets(content);
    let num_lines = line_offsets.len();
    let mut result: Vec<Vec<HighlightRun>> = vec![Vec::new(); num_lines];

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), |node: tree_sitter::Node| {
        let range = node.byte_range();
        let text = content[range.start..range.end.min(content.len())].to_string();
        std::iter::once(text)
    });

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let color = highlight_color_for_capture(capture_name);
            let node = capture.node;
            let start_byte = node.start_byte();
            let end_byte = node.end_byte();
            let start_line = node.start_position().row;
            let end_line = node.end_position().row;

            for line_idx in start_line..=end_line {
                if line_idx >= num_lines {
                    break;
                }

                let line_start = line_offsets[line_idx];
                let line_end = if line_idx + 1 < line_offsets.len() {
                    line_offsets[line_idx + 1]
                } else {
                    content.len()
                };

                let span_start = start_byte.max(line_start) - line_start;
                let span_end = end_byte.min(line_end) - line_start;

                if span_end > span_start {
                    result[line_idx].push(HighlightRun {
                        start: span_start,
                        len: span_end - span_start,
                        color,
                    });
                }
            }
        }
    }

    for runs in &mut result {
        runs.sort_by_key(|r| r.start);
    }

    result
}

fn compute_line_offsets(content: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, ch) in content.char_indices() {
        if ch == '\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}
