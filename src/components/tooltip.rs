//! Tooltip component - Tooltip with hover and keyboard support.

use gpui::{prelude::*, *};
use crate::theme::use_theme;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for TooltipPlacement {
    fn default() -> Self {
        Self::Top
    }
}

pub struct TooltipState {
    is_visible: bool,
    show_timer: Option<TaskLabel>,
}

impl TooltipState {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            show_timer: None,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

pub struct Tooltip {
    content: SharedString,
    placement: TooltipPlacement,
    show_delay: Duration,
    hide_delay: Duration,
    child: Option<AnyElement>,
    disabled: bool,
    max_width: Option<Pixels>,
}

impl Tooltip {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            placement: TooltipPlacement::default(),
            show_delay: Duration::from_millis(500),
            hide_delay: Duration::from_millis(0),
            child: None,
            disabled: false,
            max_width: Some(px(300.0)),
        }
    }

    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn show_delay(mut self, delay: Duration) -> Self {
        self.show_delay = delay;
        self
    }

    pub fn hide_delay(mut self, delay: Duration) -> Self {
        self.hide_delay = delay;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn max_width(mut self, width: Pixels) -> Self {
        self.max_width = Some(width);
        self
    }

    fn get_offset(&self, placement: TooltipPlacement) -> (Pixels, Pixels) {
        match placement {
            TooltipPlacement::Top => (px(0.0), px(-8.0)),
            TooltipPlacement::Bottom => (px(0.0), px(8.0)),
            TooltipPlacement::Left => (px(-8.0), px(0.0)),
            TooltipPlacement::Right => (px(8.0), px(0.0)),
        }
    }
}

impl IntoElement for Tooltip {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let placement = self.placement;

        // For now, we'll render a simplified version
        // In a full implementation, you'd use a stateful component with timers
        div()
            .relative()
            .group("")
            .when_some(self.child, |this: Div, child| this.child(child))
            .when(!self.disabled, |this: Div| {
                this.child(
                    deferred(
                        anchored()
                            .snap_to_window_with_margin(px(8.0))
                            .anchor(match placement {
                                TooltipPlacement::Top => Corner::BottomLeft,
                                TooltipPlacement::Bottom => Corner::TopLeft,
                                TooltipPlacement::Left => Corner::TopRight,
                                TooltipPlacement::Right => Corner::TopLeft,
                            })
                            .child(
                                div()
                                    .occlude()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .bg(theme.tokens.popover)
                                    .text_color(theme.tokens.popover_foreground)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .rounded(theme.tokens.radius_sm)
                                    .shadow_md()
                                    .text_size(px(12.0))
                                    .font_family(theme.tokens.font_family.clone())
                                    .whitespace_nowrap()
                                    .when_some(self.max_width, |div, width| div.max_w(width))
                                    .opacity(0.0)
                                    .invisible()
                                    .group_hover("", |mut style| {
                                        style.opacity = Some(1.0);
                                        style.visibility = Some(gpui::Visibility::Visible);
                                        style
                                    })
                                    .child(self.content)
                            )
                    )
                    .with_priority(1)
                )
            })
    }
}

pub fn tooltip<E: IntoElement>(child: E, content: impl Into<SharedString>) -> Tooltip {
    Tooltip::new(content).child(child)
}
