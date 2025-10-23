//! Tree navigation component with hierarchical data support.

use gpui::{prelude::*, *};
use std::sync::Arc;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use crate::theme::use_theme;
use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;

#[derive(Clone)]
pub struct TreeNode<T: Clone> {
    pub id: T,
    pub label: SharedString,
    pub children: Vec<TreeNode<T>>,
    pub icon: Option<IconSource>,
    pub icon_color: Option<Hsla>,
    pub disabled: bool,
    pub has_lazy_children: bool,
}

impl<T: Clone> TreeNode<T> {
    pub fn new(id: T, label: impl Into<SharedString>) -> Self {
        Self {
            id,
            label: label.into(),
            children: Vec::new(),
            icon: None,
            icon_color: None,
            disabled: false,
            has_lazy_children: false,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn with_children(mut self, children: Vec<TreeNode<T>>) -> Self {
        self.children = children;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_lazy_children(mut self, has_lazy: bool) -> Self {
        self.has_lazy_children = has_lazy;
        self
    }
}

#[derive(Clone)]
struct FlatTreeNode<T: Clone> {
    node: TreeNode<T>,
    level: usize,
    node_id: T,
}

#[derive(Clone)]
struct FilteredNode<T: Clone> {
    node: TreeNode<T>,
    match_ranges: Vec<(usize, usize)>,
    children: Vec<FilteredNode<T>>,
}

fn filter_tree<T: Clone>(
    nodes: &[TreeNode<T>],
    filter: &str,
) -> Vec<FilteredNode<T>> {
    if filter.is_empty() {
        return nodes.iter().map(|node| FilteredNode {
            node: node.clone(),
            match_ranges: Vec::new(),
            children: filter_tree(&node.children, filter),
        }).collect();
    }

    let filter_lower = filter.to_lowercase();
    let mut filtered = Vec::new();

    for node in nodes {
        let label_lower = node.label.to_lowercase();

        let (matches, match_ranges) = find_matches(&label_lower, &filter_lower);

        let filtered_children = filter_tree(&node.children, filter);
        if matches || !filtered_children.is_empty() {
            filtered.push(FilteredNode {
                node: node.clone(),
                match_ranges,
                children: filtered_children,
            });
        }
    }

    filtered
}

fn find_matches(text: &str, filter: &str) -> (bool, Vec<(usize, usize)>) {
    if filter.is_empty() {
        return (false, Vec::new());
    }

    let mut match_ranges = Vec::new();
    let mut start = 0;

    while let Some(pos) = text[start..].find(filter) {
        let absolute_pos = start + pos;
        match_ranges.push((absolute_pos, absolute_pos + filter.len()));
        start = absolute_pos + 1;
    }

    if !match_ranges.is_empty() {
        return (true, match_ranges);
    }

    let filter_chars: Vec<char> = filter.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    let mut filter_idx = 0;
    let mut current_match_start = None;
    let mut fuzzy_ranges = Vec::new();

    for (text_idx, &text_char) in text_chars.iter().enumerate() {
        if filter_idx < filter_chars.len() && text_char == filter_chars[filter_idx] {
            if current_match_start.is_none() {
                current_match_start = Some(text_idx);
            }
            filter_idx += 1;

            if filter_idx == filter_chars.len() {
                if let Some(start) = current_match_start {
                    fuzzy_ranges.push((start, text_idx + 1));
                }
                return (true, fuzzy_ranges);
            }
        }
    }

    (false, Vec::new())
}

fn flatten_filtered_tree<T: Clone + PartialEq + Eq + Hash>(
    filtered_nodes: &[FilteredNode<T>],
    expanded_ids: &HashSet<T>,
    level: usize,
    auto_expand_matches: bool,
) -> Vec<(FlatTreeNode<T>, Vec<(usize, usize)>)> {
    let mut flat = Vec::new();

    for filtered_node in filtered_nodes {
        flat.push((
            FlatTreeNode {
                node: filtered_node.node.clone(),
                level,
                node_id: filtered_node.node.id.clone(),
            },
            filtered_node.match_ranges.clone(),
        ));

        let should_expand = expanded_ids.contains(&filtered_node.node.id) ||
                          (auto_expand_matches && !filtered_node.children.is_empty());

        if should_expand {
            let children = flatten_filtered_tree(
                &filtered_node.children,
                expanded_ids,
                level + 1,
                auto_expand_matches,
            );
            flat.extend(children);
        }
    }

    flat
}

fn flatten_tree<T: Clone + PartialEq + Eq + Hash>(
    nodes: &[TreeNode<T>],
    expanded_ids: &HashSet<T>,
    level: usize,
) -> Vec<FlatTreeNode<T>> {
    let mut flat = Vec::new();

    for node in nodes {
        flat.push(FlatTreeNode {
            node: node.clone(),
            level,
            node_id: node.id.clone(),
        });

        if !node.children.is_empty() && expanded_ids.contains(&node.id) {
            let children = flatten_tree(&node.children, expanded_ids, level + 1);
            flat.extend(children);
        }
    }

    flat
}

const ROW_HEIGHT: f32 = 32.0;

#[derive(IntoElement)]
pub struct TreeList<T: Clone + PartialEq + Eq + Hash + 'static> {
    nodes: Vec<TreeNode<T>>,
    selected_id: Option<T>,
    expanded_ids: Vec<T>,
    filter: Option<String>,
    auto_expand_matches: bool,
    highlight_matches: bool,
    on_select: Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&T, bool, &mut Window, &mut App) + Send + Sync + 'static>>,
    on_right_click: Option<Arc<dyn Fn(&T, &MouseDownEvent, &mut Window, &mut App) + Send + Sync + 'static>>,
    style: StyleRefinement,
}

impl<T: Clone + PartialEq + Eq + Hash + 'static> TreeList<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            selected_id: None,
            expanded_ids: Vec::new(),
            filter: None,
            auto_expand_matches: false,
            highlight_matches: true,
            on_select: None,
            on_toggle: None,
            on_right_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn nodes(mut self, nodes: Vec<TreeNode<T>>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn selected_id(mut self, id: T) -> Self {
        self.selected_id = Some(id);
        self
    }

    pub fn expanded_ids(mut self, ids: Vec<T>) -> Self {
        self.expanded_ids = ids;
        self
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        let filter_str = filter.into();
        self.filter = if filter_str.is_empty() {
            None
        } else {
            Some(filter_str)
        };
        self
    }

    pub fn auto_expand_matches(mut self, auto_expand: bool) -> Self {
        self.auto_expand_matches = auto_expand;
        self
    }

    pub fn highlight_matches(mut self, highlight: bool) -> Self {
        self.highlight_matches = highlight;
        self
    }

    pub fn on_select<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_select = Some(Arc::new(f));
        self
    }

    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, bool, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_toggle = Some(Arc::new(f));
        self
    }

    pub fn on_right_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &MouseDownEvent, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_right_click = Some(Arc::new(f));
        self
    }

    fn render_highlighted_text(
        &self,
        text: &str,
        match_ranges: &[(usize, usize)],
        theme: &crate::theme::Theme,
        is_selected: bool,
    ) -> impl IntoElement {
        if match_ranges.is_empty() || !self.highlight_matches {
            return div().child(text.to_string()).into_any_element();
        }

        let mut parts = Vec::new();
        let mut last_end = 0;
        let text_chars: Vec<char> = text.chars().collect();

        let mut sorted_ranges = match_ranges.to_vec();
        sorted_ranges.sort_by_key(|r| r.0);

        for (start, end) in sorted_ranges {
            if last_end < start {
                let part: String = text_chars[last_end..start].iter().collect();
                parts.push((part, false));
            }

            let highlighted: String = text_chars[start..end.min(text_chars.len())].iter().collect();
            parts.push((highlighted, true));
            last_end = end.min(text_chars.len());
        }

        if last_end < text_chars.len() {
            let part: String = text_chars[last_end..].iter().collect();
            parts.push((part, false));
        }

        div()
            .flex()
            .children(parts.into_iter().map(|(text, is_match)| {
                if is_match {
                    div()
                        .bg(if is_selected {
                            theme.tokens.accent_foreground.opacity(0.3)
                        } else {
                            theme.tokens.accent.opacity(0.3)
                        })
                        .rounded_sm()
                        .px(px(1.0))
                        .child(text)
                        .into_any_element()
                } else {
                    div().child(text).into_any_element()
                }
            }))
            .into_any_element()
    }

}

impl<T: Clone + PartialEq + Eq + Hash + 'static> Styled for TreeList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + PartialEq + Eq + Hash + 'static> RenderOnce for TreeList<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let expanded_set: HashSet<T> = self.expanded_ids.iter().cloned().collect();

        let (flat_nodes, match_ranges_map): (Vec<FlatTreeNode<T>>, HashMap<T, Vec<(usize, usize)>>) =
            if let Some(ref filter) = self.filter {
                let filtered = filter_tree(&self.nodes, filter);
                let flat_with_ranges = flatten_filtered_tree(
                    &filtered,
                    &expanded_set,
                    0,
                    self.auto_expand_matches
                );

                let mut nodes = Vec::new();
                let mut ranges_map = HashMap::new();

                for (node, ranges) in flat_with_ranges {
                    ranges_map.insert(node.node_id.clone(), ranges);
                    nodes.push(node);
                }

                (nodes, ranges_map)
            } else {
                (flatten_tree(&self.nodes, &expanded_set, 0), HashMap::new())
            };

        let total_items = flat_nodes.len();

        let _item_sizes: Rc<Vec<Size<Pixels>>> = Rc::new(
            (0..total_items)
                .map(|_| Size {
                    width: px(0.), // Width will be determined by container
                    height: px(ROW_HEIGHT),
                })
                .collect()
        );

        let flat_nodes_rc = Rc::new(flat_nodes);
        let match_ranges_rc = Rc::new(match_ranges_map);
        let selected_id = self.selected_id.clone();
        let expanded_ids_rc = Rc::new(expanded_set);
        let on_select = self.on_select.clone();
        let on_toggle = self.on_toggle.clone();
        let on_right_click = self.on_right_click.clone();
        let highlight_matches = self.highlight_matches;
        let user_style = self.style.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.tokens.background)
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            })
            .child(
                div()
                    .w_full()
                    .children(
                                        flat_nodes_rc
                                            .iter()
                                            .enumerate()
                                            .map(|(_abs_idx, flat_node)| {

                                                let is_selected = selected_id.as_ref() == Some(&flat_node.node_id);
                                                let is_expanded = expanded_ids_rc.contains(&flat_node.node_id);
                                                let has_children = !flat_node.node.children.is_empty() || flat_node.node.has_lazy_children;
                                                let indent = px((flat_node.level as f32) * 16.0);

                                                div()
                                                    .w_full()
                                                    .h(px(ROW_HEIGHT))
                                                    .flex()
                                                    .items_center()
                                                    .px(px(8.0))
                                                    .pl(indent + px(8.0))
                                                    .cursor(if flat_node.node.disabled {
                                                        CursorStyle::Arrow
                                                    } else {
                                                        CursorStyle::PointingHand
                                                    })
                                                    .bg(if is_selected {
                                                        theme.tokens.accent
                                                    } else {
                                                        gpui::transparent_black()
                                                    })
                                                    .text_color(if is_selected {
                                                        theme.tokens.accent_foreground
                                                    } else if flat_node.node.disabled {
                                                        theme.tokens.muted_foreground
                                                    } else {
                                                        theme.tokens.primary
                                                    })
                                                    .when(!flat_node.node.disabled && !is_selected, |div| {
                                                        div.hover(|mut style| {
                                                            style.background = Some(theme.tokens.accent.opacity(0.5).into());
                                                            style
                                                        })
                                                    })
                                                    .when(!flat_node.node.disabled, {
                                                        let on_select = on_select.clone();
                                                        let on_toggle = on_toggle.clone();
                                                        let node_id = flat_node.node_id.clone();

                                                        move |this| {
                                                            this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                                if let Some(on_select) = on_select.clone() {
                                                                    on_select(&node_id, window, cx);
                                                                }

                                                                if has_children {
                                                                    if let Some(on_toggle) = on_toggle.clone() {
                                                                        on_toggle(&node_id, !is_expanded, window, cx);
                                                                    }
                                                                }
                                                            })
                                                        }
                                                    })
                                                    .when(!flat_node.node.disabled, {
                                                        let on_right_click = on_right_click.clone();
                                                        let node_id = flat_node.node_id.clone();

                                                        move |this| {
                                                            this.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                                                                eprintln!("TreeList: Right mouse button down on node");
                                                                if let Some(on_right_click) = on_right_click.clone() {
                                                                    eprintln!("TreeList: Calling on_right_click handler");
                                                                    on_right_click(&node_id, event, window, cx);
                                                                } else {
                                                                    eprintln!("TreeList: No on_right_click handler!");
                                                                }
                                                            })
                                                        }
                                                    })
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(8.0))
                                                            .children(
                                                                flat_node.node.icon.as_ref().map(|icon| {
                                                                    Icon::new(icon.clone())
                                                                        .size(px(16.0))
                                                                        .color(if is_selected {
                                                                            theme.tokens.accent_foreground
                                                                        } else if flat_node.node.disabled {
                                                                            theme.tokens.muted_foreground
                                                                        } else {
                                                                            theme.tokens.primary
                                                                        })
                                                                })
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .text_size(px(14.0))
                                                                    .font_family(theme.tokens.font_family.clone())
                                                                    .font_weight(if is_selected {
                                                                        FontWeight::SEMIBOLD
                                                                    } else {
                                                                        FontWeight::NORMAL
                                                                    })
                                                                    .child({
                                                                        let ranges = match_ranges_rc.get(&flat_node.node_id)
                                                                            .map(|r| r.as_slice())
                                                                            .unwrap_or(&[]);

                                                                        if !ranges.is_empty() && highlight_matches {
                                                                            self.render_highlighted_text(
                                                                                &flat_node.node.label,
                                                                                ranges,
                                                                                &theme,
                                                                                is_selected
                                                                            ).into_any_element()
                                                                        } else {
                                                                            div().child(flat_node.node.label.clone()).into_any_element()
                                                                        }
                                                                    })
                                                            )
                                                            .children(
                                                                if has_children {
                                                                    Some(
                                                                        div()
                                                                            .w(px(16.0))
                                                                            .h(px(16.0))
                                                                            .flex()
                                                                            .items_center()
                                                                            .justify_center()
                                                                            .child(
                                                                                Icon::new(if is_expanded { "arrow-down" } else { "arrow-right" })
                                                                                    .size(px(12.0))
                                                                                    .color(theme.tokens.primary)
                                                                            )
                                                                    )
                                                                } else {
                                                                    None
                                                                }
                                                            )
                                                    )
                                            })
                                    )
            )
    }
}

#[derive(Clone)]
pub struct ListItem<T: Clone> {
    pub id: T,
    pub label: SharedString,
    pub icon: Option<IconSource>,
    pub badge: Option<SharedString>,
    pub disabled: bool,
}

impl<T: Clone> ListItem<T> {
    pub fn new(id: T, label: impl Into<SharedString>) -> Self {
        Self {
            id,
            label: label.into(),
            icon: None,
            badge: None,
            disabled: false,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(IntoElement)]
pub struct List<T: Clone + PartialEq + 'static> {
    items: Vec<ListItem<T>>,
    selected_id: Option<T>,
    on_select: Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
}

impl<T: Clone + PartialEq + 'static> List<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_id: None,
            on_select: None,
        }
    }

    pub fn items(mut self, items: Vec<ListItem<T>>) -> Self {
        self.items = items;
        self
    }

    pub fn selected_id(mut self, id: T) -> Self {
        self.selected_id = Some(id);
        self
    }

    pub fn on_select<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_select = Some(Arc::new(f));
        self
    }

    fn is_selected(&self, item_id: &T) -> bool {
        self.selected_id.as_ref() == Some(item_id)
    }

    fn render_item(&self, item: &ListItem<T>, theme: &crate::theme::Theme) -> impl IntoElement {
        let is_selected = self.is_selected(&item.id);

        let base = div()
            .flex()
            .items_center()
            .w_full()
            .px(px(12.0))
            .py(px(8.0))
            .cursor(if item.disabled {
                CursorStyle::Arrow
            } else {
                CursorStyle::PointingHand
            });

        let styled = base
            .bg(if is_selected {
                theme.tokens.accent
            } else {
                gpui::transparent_black()
            })
            .text_color(if is_selected {
                theme.tokens.accent_foreground
            } else if item.disabled {
                theme.tokens.muted_foreground
            } else {
                theme.tokens.primary
            })
            .when(!item.disabled && !is_selected, |div| {
                div.hover(|mut style| {
                    style.background = Some(theme.tokens.accent.opacity(0.5).into());
                    style
                })
            });

        let element = if let Some(icon) = item.icon.as_ref() {
            styled.child(
                div()
                    .mr(px(10.0))
                    .child(
                        Icon::new(icon.clone())
                            .size(px(18.0))
                            .color(if is_selected {
                                theme.tokens.accent_foreground
                            } else if item.disabled {
                                theme.tokens.muted_foreground
                            } else {
                                theme.tokens.primary
                            }),
                    )
            )
        } else {
            styled
        };

        let with_label = element.child(
            div()
                .flex_1()
                .text_size(px(14.0))
                .font_family(theme.tokens.font_family.clone())
                .font_weight(if is_selected {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::NORMAL
                })
                .child(item.label.clone())
        );

        let with_badge = with_label.when_some(item.badge.as_ref(), |parent, badge| {
            parent.child(
                div()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded(px(10.0))
                    .bg(if is_selected {
                        theme.tokens.accent_foreground.opacity(0.2)
                    } else {
                        theme.tokens.muted
                    })
                    .text_size(px(11.0))
                    .font_family(theme.tokens.font_family.clone())
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(if is_selected {
                        theme.tokens.accent_foreground
                    } else {
                        theme.tokens.muted_foreground
                    })
                    .child(badge.clone())
            )
        });

        with_badge.when(!item.disabled, |this| {
            let on_select = self.on_select.clone();
            let item_id = item.id.clone();

            this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                if let Some(on_select) = on_select.clone() {
                    on_select(&item_id, window, cx);
                }
            })
        })
    }
}

impl<T: Clone + PartialEq + 'static> RenderOnce for List<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.tokens.background)
            .children(self.items.iter().map(|item| {
                self.render_item(item, &theme).into_any_element()
            }))
    }
}
