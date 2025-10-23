//! Hover card component with popover on hover.

use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

use crate::theme::use_theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for HoverCardPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAlignment {
    Start,
    Center,
    End,
}

impl Default for HoverCardAlignment {
    fn default() -> Self {
        Self::Center
    }
}

pub struct HoverCard {
    trigger: AnyElement,
    content: AnyElement,
    position: HoverCardPosition,
    alignment: HoverCardAlignment,
    _open_delay: Duration,
    _close_delay: Duration,
    is_open: bool,
    _arrow: bool,
    style: StyleRefinement,
}

impl HoverCard {
    pub fn new() -> Self {
        Self {
            trigger: div().into_any_element(),
            content: div().into_any_element(),
            position: HoverCardPosition::default(),
            alignment: HoverCardAlignment::default(),
            _open_delay: Duration::from_millis(200),
            _close_delay: Duration::from_millis(300),
            is_open: false,
            _arrow: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = trigger.into_any_element();
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    pub fn position(mut self, position: HoverCardPosition) -> Self {
        self.position = position;
        self
    }

    pub fn alignment(mut self, alignment: HoverCardAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn open_delay(mut self, delay: Duration) -> Self {
        self._open_delay = delay;
        self
    }

    pub fn close_delay(mut self, delay: Duration) -> Self {
        self._close_delay = delay;
        self
    }

    pub fn arrow(mut self, show_arrow: bool) -> Self {
        self._arrow = show_arrow;
        self
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }
}

impl Default for HoverCard {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for HoverCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for HoverCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        div()
            .relative()
            .child(self.trigger)
            .when(self.is_open, |this: Div| {
                this.child(
                    div()
                        .absolute()
                        .when(self.position == HoverCardPosition::Bottom, |this: Div| {
                            this.top_full().mt(px(8.0))
                        })
                        .when(self.position == HoverCardPosition::Top, |this: Div| {
                            this.bottom_full().mb(px(8.0))
                        })
                        .when(self.position == HoverCardPosition::Left, |this: Div| {
                            this.right_full().mr(px(8.0))
                        })
                        .when(self.position == HoverCardPosition::Right, |this: Div| {
                            this.left_full().ml(px(8.0))
                        })
                        .when(
                            (self.position == HoverCardPosition::Top || self.position == HoverCardPosition::Bottom)
                                && self.alignment == HoverCardAlignment::Start,
                            |this: Div| this.left_0()
                        )
                        .when(
                            (self.position == HoverCardPosition::Top || self.position == HoverCardPosition::Bottom)
                                && self.alignment == HoverCardAlignment::End,
                            |this: Div| this.right_0()
                        )
                        .child(
                            div()
                                .min_w(px(200.0))
                                .max_w(px(400.0))
                                .bg(theme.tokens.popover)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .rounded(theme.tokens.radius_md)
                                .shadow(vec![BoxShadow {
                                    color: hsla(0.0, 0.0, 0.0, 0.1),
                                    offset: point(px(0.0), px(4.0)),
                                    blur_radius: px(12.0),
                                    spread_radius: px(0.0),
                                }])
                                .map(|this| {
                                    let mut div = this;
                                    div.style().refine(&user_style);
                                    div
                                })
                                .child(self.content)
                        )
                )
            })
    }
}
