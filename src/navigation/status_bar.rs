//! Status bar component with customizable sections.

use gpui::{prelude::FluentBuilder as _, InteractiveElement, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::{
        text::caption,
        icon::Icon,
        icon_source::IconSource,
        badge::{Badge, BadgeVariant},
    },
};

#[derive(Clone)]
pub struct StatusItem {
    pub icon: Option<IconSource>,
    pub text: Option<SharedString>,
    pub badge: Option<SharedString>,
    pub badge_variant: BadgeVariant,
    pub on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    pub disabled: bool,
    pub tooltip: Option<SharedString>,
}

impl StatusItem {
    pub fn text(text: impl Into<SharedString>) -> Self {
        Self {
            icon: None,
            text: Some(text.into()),
            badge: None,
            badge_variant: BadgeVariant::Default,
            on_click: None,
            disabled: false,
            tooltip: None,
        }
    }

    pub fn icon(icon: impl Into<IconSource>) -> Self {
        Self {
            icon: Some(icon.into()),
            text: None,
            badge: None,
            badge_variant: BadgeVariant::Default,
            on_click: None,
            disabled: false,
            tooltip: None,
        }
    }

    pub fn icon_text(icon: impl Into<IconSource>, text: impl Into<SharedString>) -> Self {
        Self {
            icon: Some(icon.into()),
            text: Some(text.into()),
            badge: None,
            badge_variant: BadgeVariant::Default,
            on_click: None,
            disabled: false,
            tooltip: None,
        }
    }

    pub fn badge(text: impl Into<SharedString>, tooltip: impl Into<SharedString>) -> Self {
        Self {
            icon: None,
            text: None,
            badge: Some(text.into()),
            badge_variant: BadgeVariant::Default,
            on_click: None,
            disabled: false,
            tooltip: Some(tooltip.into()),
        }
    }

    pub fn icon_badge(icon: impl Into<IconSource>, badge: impl Into<SharedString>) -> Self {
        Self {
            icon: Some(icon.into()),
            text: None,
            badge: Some(badge.into()),
            badge_variant: BadgeVariant::Default,
            on_click: None,
            disabled: false,
            tooltip: None,
        }
    }

    pub fn badge_variant(mut self, variant: BadgeVariant) -> Self {
        self.badge_variant = variant;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusBarSection {
    Left,
    Center,
    Right,
}

pub struct StatusBar {
    left_items: Vec<StatusItem>,
    center_items: Vec<StatusItem>,
    right_items: Vec<StatusItem>,
    height: Pixels,
    style: StyleRefinement,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            left_items: Vec::new(),
            center_items: Vec::new(),
            right_items: Vec::new(),
            height: px(28.0),
            style: StyleRefinement::default(),
        }
    }

    pub fn left(mut self, items: Vec<StatusItem>) -> Self {
        self.left_items = items;
        self
    }

    pub fn center(mut self, items: Vec<StatusItem>) -> Self {
        self.center_items = items;
        self
    }

    pub fn right(mut self, items: Vec<StatusItem>) -> Self {
        self.right_items = items;
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }

    pub fn add_left(mut self, item: StatusItem) -> Self {
        self.left_items.push(item);
        self
    }

    pub fn add_center(mut self, item: StatusItem) -> Self {
        self.center_items.push(item);
        self
    }

    pub fn add_right(mut self, item: StatusItem) -> Self {
        self.right_items.push(item);
        self
    }
}

impl Styled for StatusBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style.clone();

        div()
            .flex()
            .items_center()
            .justify_between()
            .h(self.height)
            .px(px(12.0))
            .py(px(6.0))
            .gap(px(12.0))
            .bg(theme.tokens.card)
            .border_t_1()
            .border_color(theme.tokens.border)
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .children(self.left_items.iter().map(|item| {
                        render_status_item(item.clone())
                    }))
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .children(self.center_items.iter().map(|item| {
                        render_status_item(item.clone())
                    }))
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .children(self.right_items.iter().map(|item| {
                        render_status_item(item.clone())
                    }))
            )
    }
}

fn render_status_item(item: StatusItem) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .items_center()
        .gap(px(6.0))
        .px(px(8.0))
        .py(px(4.0))
        .rounded(theme.tokens.radius_sm)
        .when(!item.disabled && item.on_click.is_some(), |div| {
            div.cursor(CursorStyle::PointingHand)
                .hover(|style| style.bg(theme.tokens.muted))
        })
        .when(item.disabled, |div| div.opacity(0.5))
        .when_some(item.on_click.filter(|_| !item.disabled), |div, handler| {
            div.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                handler(window, cx);
            })
        })
        .when_some(item.icon, |div, icon| {
            div.child(
                Icon::new(icon)
                    .size(px(14.0))
                    .color(if item.disabled {
                        theme.tokens.muted_foreground
                    } else {
                        theme.tokens.foreground
                    })
            )
        })
        .when_some(item.text, |div, text| {
            div.child(
                caption(text).color(if item.disabled {
                    theme.tokens.muted_foreground
                } else {
                    theme.tokens.foreground
                })
            )
        })
        .when_some(item.badge, |div, badge_text| {
            div.child(
                Badge::new(badge_text)
                    .variant(item.badge_variant)
            )
        })
}
