use super::{Symbol, SymbolKind};
use gpui::*;

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: SymbolKind,
    pub insert_text: String,
}

impl From<Symbol> for CompletionItem {
    fn from(sym: Symbol) -> Self {
        Self {
            label: sym.name.clone(),
            kind: sym.kind,
            insert_text: sym.name,
        }
    }
}

pub struct CompletionState {
    items: Vec<CompletionItem>,
    filtered_indices: Vec<usize>,
    selected_index: usize,
    visible: bool,
    filter_prefix: String,
    trigger_col: usize,
    trigger_line: usize,
    anchor_position: Point<Pixels>,
}

impl CompletionState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            filtered_indices: Vec::new(),
            selected_index: 0,
            visible: false,
            filter_prefix: String::new(),
            trigger_col: 0,
            trigger_line: 0,
            anchor_position: Point::default(),
        }
    }

    pub fn show(
        &mut self,
        items: Vec<CompletionItem>,
        trigger_line: usize,
        trigger_col: usize,
        anchor: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.items = items;
        self.trigger_line = trigger_line;
        self.trigger_col = trigger_col;
        self.anchor_position = anchor;
        self.filter_prefix.clear();
        self.selected_index = 0;
        self.update_filtered();
        self.visible = !self.filtered_indices.is_empty();
        cx.notify();
    }

    pub fn set_filter(&mut self, prefix: &str, cx: &mut Context<Self>) {
        self.filter_prefix = prefix.to_string();
        self.update_filtered();
        self.selected_index = 0;
        self.visible = !self.filtered_indices.is_empty();
        cx.notify();
    }

    fn update_filtered(&mut self) {
        if self.filter_prefix.is_empty() {
            self.filtered_indices = (0..self.items.len()).collect();
            return;
        }

        let prefix = &self.filter_prefix;
        let prefix_lower = prefix.to_lowercase();

        let mut scored: Vec<(usize, u8)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                let label = &item.label;
                let label_lower = label.to_lowercase();

                let score = if label.starts_with(prefix) {
                    0
                } else if label_lower.starts_with(&prefix_lower) {
                    1
                } else if label_lower.contains(&prefix_lower) {
                    2
                } else {
                    return None;
                };

                Some((i, score))
            })
            .collect();

        scored.sort_by(|a, b| {
            a.1.cmp(&b.1).then_with(|| {
                self.items[a.0]
                    .label
                    .len()
                    .cmp(&self.items[b.0].label.len())
            })
        });

        self.filtered_indices = scored.into_iter().map(|(i, _)| i).collect();
    }

    pub fn move_up(&mut self, cx: &mut Context<Self>) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.filtered_indices.len() - 1;
        }
        cx.notify();
    }

    pub fn move_down(&mut self, cx: &mut Context<Self>) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.selected_index < self.filtered_indices.len() - 1 {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
        cx.notify();
    }

    pub fn selected_item(&self) -> Option<&CompletionItem> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|&i| self.items.get(i))
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        self.items.clear();
        self.filtered_indices.clear();
        self.filter_prefix.clear();
        cx.notify();
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn filtered_items(&self) -> impl Iterator<Item = (usize, &CompletionItem)> {
        self.filtered_indices
            .iter()
            .enumerate()
            .filter_map(|(display_idx, &item_idx)| {
                self.items.get(item_idx).map(|item| (display_idx, item))
            })
    }

    pub fn selected_display_index(&self) -> usize {
        self.selected_index
    }

    pub fn anchor_position(&self) -> Point<Pixels> {
        self.anchor_position
    }

    pub fn trigger_col(&self) -> usize {
        self.trigger_col
    }

    pub fn trigger_line(&self) -> usize {
        self.trigger_line
    }

    #[allow(dead_code)]
    pub fn filter_prefix(&self) -> &str {
        &self.filter_prefix
    }

    pub fn update_anchor(&mut self, anchor: Point<Pixels>) {
        self.anchor_position = anchor;
    }
}
