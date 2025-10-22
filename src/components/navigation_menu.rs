//! Navigation menu component - Hierarchical navigation with expand/collapse state.

use gpui::{prelude::FluentBuilder as _, *};
use std::sync::Arc;
use std::hash::Hash;
use std::collections::HashSet;

use crate::theme::use_theme;
use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::components::text::{Text, TextVariant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuOrientation {
    Horizontal,
    Vertical,
}

impl Default for NavigationMenuOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

#[derive(Clone)]
pub struct NavigationMenuItem<T: Clone = SharedString> {
    pub id: T,
    pub label: SharedString,
    pub icon: Option<IconSource>,
    pub disabled: bool,
    pub children: Vec<NavigationMenuItem<T>>,
}

impl<T: Clone> NavigationMenuItem<T> {
    pub fn new(id: T, label: impl Into<SharedString>) -> Self {
        Self {
            id,
            label: label.into(),
            icon: None,
            disabled: false,
            children: Vec::new(),
        }
    }

    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_children(mut self, children: Vec<NavigationMenuItem<T>>) -> Self {
        self.children = children;
        self
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

#[derive(IntoElement)]
pub struct NavigationMenu<T: Clone + PartialEq + Eq + Hash + 'static> {
    orientation: NavigationMenuOrientation,
    items: Vec<NavigationMenuItem<T>>,
    selected_id: Option<T>,
    expanded_ids: Vec<T>,
    on_select: Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&T, bool, &mut Window, &mut App) + Send + Sync + 'static>>,
    style: StyleRefinement,
}

impl<T: Clone + PartialEq + Eq + Hash + 'static> NavigationMenu<T> {
    /// Create a new navigation menu
    pub fn new() -> Self {
        Self {
            orientation: NavigationMenuOrientation::default(),
            items: Vec::new(),
            selected_id: None,
            expanded_ids: Vec::new(),
            on_select: None,
            on_toggle: None,
            style: StyleRefinement::default(),
        }
    }

    /// Set the menu orientation
    pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Add a menu item
    pub fn item(mut self, item: NavigationMenuItem<T>) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple menu items
    pub fn items(mut self, items: Vec<NavigationMenuItem<T>>) -> Self {
        self.items = items;
        self
    }

    /// Set the selected item ID
    pub fn selected_id(mut self, id: T) -> Self {
        self.selected_id = Some(id);
        self
    }

    /// Set the expanded item IDs
    pub fn expanded_ids(mut self, ids: Vec<T>) -> Self {
        self.expanded_ids = ids;
        self
    }

    /// Set the selection handler
    pub fn on_select<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_select = Some(Arc::new(f));
        self
    }

    /// Set the toggle (expand/collapse) handler
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, bool, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_toggle = Some(Arc::new(f));
        self
    }
}

impl<T: Clone + PartialEq + Eq + Hash + 'static> Default for NavigationMenu<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + PartialEq + Eq + Hash + 'static> Styled for NavigationMenu<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + PartialEq + Eq + Hash + 'static> RenderOnce for NavigationMenu<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let orientation = self.orientation;

        let expanded_set: HashSet<T> = self.expanded_ids.into_iter().collect();
        let selected_id = self.selected_id;
        let on_select = self.on_select;
        let on_toggle = self.on_toggle;
        let user_style = self.style;

        div()
            .flex()
            .when(orientation == NavigationMenuOrientation::Horizontal, |this: Div| {
                this.flex_row().items_center().gap(px(4.0))
            })
            .when(orientation == NavigationMenuOrientation::Vertical, |this: Div| {
                this.flex_col().gap(px(2.0))
            })
            .children(
                self.items.into_iter().map(move |item| {
                    render_menu_item(
                        item,
                        orientation,
                        &theme,
                        0,
                        &expanded_set,
                        &selected_id,
                        &on_select,
                        &on_toggle,
                    )
                })
            )
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

/// Render a single menu item recursively
fn render_menu_item<T: Clone + PartialEq + Eq + Hash + 'static>(
    item: NavigationMenuItem<T>,
    orientation: NavigationMenuOrientation,
    theme: &crate::theme::Theme,
    depth: usize,
    expanded_set: &HashSet<T>,
    selected_id: &Option<T>,
    on_select: &Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
    on_toggle: &Option<Arc<dyn Fn(&T, bool, &mut Window, &mut App) + Send + Sync + 'static>>,
) -> impl IntoElement {
    let has_children = item.has_children();
    let disabled = item.disabled;
    let is_expanded = expanded_set.contains(&item.id);
    let is_selected = selected_id.as_ref() == Some(&item.id);
    let indent = px(depth as f32 * 16.0);

    div()
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .px(px(8.0))
                .py(px(8.0))
                .pl(when(orientation == NavigationMenuOrientation::Vertical && depth > 0,
                    indent + px(8.0),
                    px(8.0)))
                .rounded(theme.tokens.radius_sm)
                .text_size(px(14.0))
                .when(is_selected, |this: Div| {
                    this.bg(theme.tokens.accent)
                })
                .when(!is_selected && !disabled, |this: Div| {
                    this.hover(|style| {
                        style.bg(theme.tokens.accent.opacity(0.1))
                    })
                })
                .when(has_children, |this: Div| {
                    let item_id = item.id.clone();
                    let on_toggle = on_toggle.clone();
                    let is_expanded_copy = is_expanded;

                    this.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded(px(4.0))
                            .cursor(if disabled {
                                CursorStyle::Arrow
                            } else {
                                CursorStyle::PointingHand
                            })
                            .when(!disabled && !is_selected, |this: Div| {
                                this.hover(|style| {
                                    style.bg(theme.tokens.muted.opacity(0.3))
                                })
                            })
                            .when(!disabled && on_toggle.is_some(), |this: Div| {
                                let on_toggle = on_toggle.unwrap();
                                this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    on_toggle(&item_id, !is_expanded_copy, window, cx);
                                })
                            })
                            .child(
                                Icon::new(if is_expanded { "arrow-down" } else { "arrow-right" })
                                    .size(px(12.0))
                                    .color(if is_selected {
                                        theme.tokens.accent_foreground
                                    } else {
                                        theme.tokens.muted_foreground
                                    })
                            )
                    )
                })
                .when(!has_children, |this: Div| {
                    this.child(div().w(px(20.0)))
                })
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .items_center()
                        .gap(px(8.0))
                        .cursor(if disabled {
                            CursorStyle::Arrow
                        } else {
                            CursorStyle::PointingHand
                        })
                        .when(!disabled, |this: Div| {
                            let item_id = item.id.clone();
                            let on_select = on_select.clone();

                            this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                if let Some(on_select) = on_select.as_ref() {
                                    on_select(&item_id, window, cx);
                                }
                            })
                        })
                        .when_some(item.icon.clone(), |this: Div, icon| {
                            this.child(
                                Icon::new(icon)
                                    .size(px(16.0))
                                    .color(if is_selected {
                                        theme.tokens.accent_foreground
                                    } else if disabled {
                                        theme.tokens.muted_foreground
                                    } else {
                                        theme.tokens.foreground
                                    })
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .when(disabled, |this: Div| this.opacity(0.5))
                                .child(
                                    Text::new(item.label.clone())
                                        .variant(TextVariant::Body)
                                        .weight(if is_selected {
                                            FontWeight::SEMIBOLD
                                        } else {
                                            FontWeight::NORMAL
                                        })
                                        .color(if is_selected {
                                            theme.tokens.accent_foreground
                                        } else if disabled {
                                            theme.tokens.muted_foreground
                                        } else {
                                            theme.tokens.foreground
                                        })
                                )
                        )
                )
        )
        .when(has_children && is_expanded, |this: Div| {
            this.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .when(orientation == NavigationMenuOrientation::Horizontal, |this: Div| {
                        this.absolute()
                            .top_full()
                            .left_0()
                            .mt(px(4.0))
                            .min_w(px(200.0))
                            .bg(theme.tokens.popover)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .rounded(theme.tokens.radius_md)
                            .shadow(vec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.1),
                                offset: point(px(0.0), px(2.0)),
                                blur_radius: px(8.0),
                                spread_radius: px(0.0),
                            }])
                            .p(px(4.0))
                    })
                    .when(orientation == NavigationMenuOrientation::Vertical, |this: Div| {
                        this.mt(px(2.0))
                    })
                    .children(
                        item.children.into_iter().map(|child| {
                            render_menu_item(
                                child,
                                orientation,
                                theme,
                                depth + 1,
                                expanded_set,
                                selected_id,
                                on_select,
                                on_toggle,
                            )
                        })
                    )
            )
        })
}

/// Helper function for conditional values
fn when<T>(condition: bool, true_value: T, false_value: T) -> T {
    if condition { true_value } else { false_value }
}
