//! Collapsible component - Expandable/collapsible section with trigger and content.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;

#[derive(IntoElement)]
pub struct Collapsible {
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    is_open: bool,
    disabled: bool,
    show_icon: bool,
    on_toggle: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl Collapsible {
    pub fn new() -> Self {
        Self {
            trigger: None,
            content: None,
            is_open: false,
            disabled: false,
            show_icon: true,
            on_toggle: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    pub fn on_toggle<F>(mut self, handler: F) -> Self
    where
        F: Fn(bool, &mut Window, &mut App) + 'static,
    {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl Default for Collapsible {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Collapsible {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let Collapsible {
            trigger,
            content,
            is_open,
            disabled,
            show_icon,
            on_toggle,
            style: _,
        } = self;

        div()
            .flex()
            .flex_col()
            .w_full()
            .when_some(trigger, |this: Div, trigger| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .cursor(if disabled {
                            CursorStyle::Arrow
                        } else {
                            CursorStyle::PointingHand
                        })
                        .when(!disabled, |this: Div| {
                            this.hover(|style| {
                                style.bg(theme.tokens.muted.opacity(0.5))
                            })
                        })
                        .when(disabled, |this: Div| {
                            this.opacity(0.5)
                        })
                        .when(!disabled && on_toggle.is_some(), |this: Div| {
                            let handler = on_toggle.clone().unwrap();
                            this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                handler(!is_open, window, cx);
                            })
                        })
                        .when(show_icon, |this: Div| {
                            this.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .when(is_open, |this: Div| {
                                        this.child("▼")
                                    })
                                    .when(!is_open, |this: Div| {
                                        this.child("▶")
                                    })
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .child(trigger)
                        )
                )
            })
            .when(is_open, |this: Div| {
                this.when_some(content, |this: Div, content| {
                    this.child(
                        div()
                            .overflow_hidden()
                            .child(content)
                    )
                })
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
