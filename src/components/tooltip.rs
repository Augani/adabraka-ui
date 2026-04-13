//! Tooltip component - Tooltip with hover and keyboard support.

use crate::theme::use_theme;
use gpui::{prelude::*, *};
use std::time::Duration;

const TOOLTIP_GROUP: &str = "tooltip";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Default)]
pub struct TooltipState {
    is_visible: bool,
}

impl TooltipState {
    pub fn new() -> Self {
        Self::default()
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
    style: StyleRefinement,
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
            style: StyleRefinement::default(),
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
}

impl Styled for Tooltip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for Tooltip {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let placement = self.placement;
        let user_style = self.style;
        let tooltip = div()
            .occlude()
            .absolute()
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
            .group_hover(TOOLTIP_GROUP, |mut style| {
                style.opacity = Some(1.0);
                style.visibility = Some(gpui::Visibility::Visible);
                style
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(self.content);

        div()
            .relative()
            .group(TOOLTIP_GROUP)
            .when_some(self.child, |this: Div, child| this.child(child))
            .when(!self.disabled, |this: Div| {
                this.child(match placement {
                    TooltipPlacement::Top => tooltip
                        .bottom_full()
                        .left_1_2()
                        .mb(px(8.0))
                        .ml(px(-80.0)),
                    TooltipPlacement::Bottom => tooltip
                        .top_full()
                        .left_1_2()
                        .mt(px(8.0))
                        .ml(px(-80.0)),
                    TooltipPlacement::Left => tooltip
                        .right_full()
                        .top_1_2()
                        .mr(px(8.0))
                        .mt(px(-14.0)),
                    TooltipPlacement::Right => tooltip
                        .left_full()
                        .top_1_2()
                        .ml(px(8.0))
                        .mt(px(-14.0)),
                })
            })
    }
}

pub fn tooltip<E: IntoElement>(child: E, content: impl Into<SharedString>) -> Tooltip {
    Tooltip::new(content).child(child)
}
