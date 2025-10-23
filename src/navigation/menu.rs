//! Menu system for dropdown and context menus.

use gpui::{prelude::FluentBuilder as _, InteractiveElement, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::{
        text::{body, caption},
        icon::Icon,
        icon_source::IconSource,
    },
};

#[derive(Clone, Debug)]
pub enum MenuItemKind {
    Action,
    Checkbox { checked: bool },
    Radio { checked: bool },
    Submenu,
    Separator,
}

#[derive(Clone)]
pub struct MenuItem {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconSource>,
    pub shortcut: Option<SharedString>,
    pub kind: MenuItemKind,
    pub disabled: bool,
    pub on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            kind: MenuItemKind::Action,
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn separator() -> Self {
        Self {
            id: SharedString::from("separator"),
            label: SharedString::from(""),
            icon: None,
            shortcut: None,
            kind: MenuItemKind::Separator,
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn checkbox(id: impl Into<SharedString>, label: impl Into<SharedString>, checked: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            kind: MenuItemKind::Checkbox { checked },
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn submenu(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            kind: MenuItemKind::Submenu,
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn with_children(mut self, children: Vec<MenuItem>) -> Self {
        self.children = children;
        self
    }
}

#[derive(IntoElement)]
pub struct Menu {
    items: Vec<MenuItem>,
    min_width: Pixels,
    max_height: Option<Pixels>,
    style: StyleRefinement,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            min_width: px(200.0),
            max_height: Some(px(400.0)),
            style: StyleRefinement::default(),
        }
    }

    pub fn min_width(mut self, width: Pixels) -> Self {
        self.min_width = width;
        self
    }

    pub fn max_height(mut self, height: Option<Pixels>) -> Self {
        self.max_height = height;
        self
    }
}

impl Styled for Menu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Menu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        div()
            .min_w(self.min_width)
            .when_some(self.max_height, |div, h| div.max_h(h))
            .flex()
            .flex_col()
            .bg(theme.tokens.popover)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_md)
            .shadow_lg()
            .p(px(4.0))
            .children(self.items.into_iter().map(|item| {
                render_menu_item(item)
            }))
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

fn render_menu_item(item: MenuItem) -> impl IntoElement {
    let theme = use_theme();

    match item.kind {
        MenuItemKind::Separator => {
            div()
                .h(px(1.0))
                .bg(theme.tokens.border)
                .my(px(4.0))
                .mx(px(8.0))
        }
        _ => {
            let is_checked = matches!(
                item.kind,
                MenuItemKind::Checkbox { checked: true } | MenuItemKind::Radio { checked: true }
            );
            let has_submenu = matches!(item.kind, MenuItemKind::Submenu);

            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .px(px(12.0))
                .py(px(8.0))
                .rounded(theme.tokens.radius_sm)
                .cursor(if item.disabled {
                    CursorStyle::Arrow
                } else {
                    CursorStyle::PointingHand
                })
                .when(item.disabled, |div| div.opacity(0.5))
                .when(!item.disabled, |div| {
                    div.hover(|style| style.bg(theme.tokens.accent))
                })
                .when_some(item.on_click.filter(|_| !item.disabled), |div, handler| {
                    div.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        handler(window, cx);
                    })
                })
                .child(
                    div()
                        .w(px(16.0))
                        .h(px(16.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(is_checked, |div| {
                            div.child(
                                Icon::new("check")
                                    .size(px(12.0))
                                    .color(theme.tokens.foreground)
                            )
                        })
                )
                .when_some(item.icon, |div, icon| {
                    div.child(
                        Icon::new(icon)
                            .size(px(16.0))
                            .color(if item.disabled {
                                theme.tokens.muted_foreground
                            } else {
                                theme.tokens.foreground
                            })
                    )
                })
                .child(
                    div()
                        .flex_1()
                        .child(body(item.label).color(
                            if item.disabled {
                                theme.tokens.muted_foreground
                            } else {
                                theme.tokens.foreground
                            }
                        ))
                )
                .when_some(item.shortcut, |div, shortcut| {
                    div.child(
                        caption(shortcut)
                            .color(theme.tokens.muted_foreground)
                            .no_wrap()
                    )
                })
                .when(has_submenu, |div| {
                    div.child(
                        Icon::new("chevron-right")
                            .size(px(14.0))
                            .color(theme.tokens.muted_foreground)
                    )
                })
        }
    }
}

#[derive(Clone)]
pub struct MenuBarItem {
    pub id: SharedString,
    pub label: SharedString,
    pub menu_items: Vec<MenuItem>,
}

impl MenuBarItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            menu_items: Vec::new(),
        }
    }

    pub fn with_items(mut self, items: Vec<MenuItem>) -> Self {
        self.menu_items = items;
        self
    }
}

pub struct MenuBar {
    items: Vec<MenuBarItem>,
    active_menu: Option<usize>,
}

impl MenuBar {
    pub fn new(items: Vec<MenuBarItem>) -> Self {
        Self {
            items,
            active_menu: None,
        }
    }
}

impl Render for MenuBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .flex()
            .items_center()
            .h(px(40.0))
            .px(px(8.0))
            .gap(px(2.0))
            .bg(theme.tokens.background)
            .border_b_1()
            .border_color(theme.tokens.border)
            .children(self.items.iter().enumerate().map(|(idx, item)| {
                let is_active = self.active_menu == Some(idx);
                let label = item.label.clone();

                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(theme.tokens.radius_sm)
                    .cursor(CursorStyle::PointingHand)
                    .when(is_active, |div| div.bg(theme.tokens.accent))
                    .when(!is_active, |div| {
                        div.hover(|style| style.bg(theme.tokens.muted))
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                        this.active_menu = if this.active_menu == Some(idx) {
                            None
                        } else {
                            Some(idx)
                        };
                        cx.notify();
                    }))
                    .child(body(label).color(theme.tokens.foreground))
            }))
    }
}

#[derive(IntoElement)]
pub struct ContextMenu {
    items: Vec<MenuItem>,
    position: Point<Pixels>,
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>, position: Point<Pixels>) -> Self {
        Self { items, position }
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        anchored()
            .snap_to_window_with_margin(px(8.0))
            .anchor(Corner::TopLeft)
            .position(self.position)
            .child(
                div()
                    .min_w(px(200.0))
                    .max_h(px(400.0))
                    .flex()
                    .flex_col()
                    .bg(theme.tokens.popover)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .shadow_lg()
                    .p(px(4.0))
                    .children(self.items.into_iter().map(|item| {
                        render_menu_item(item)
                    }))
            )
    }
}
