//! Context menu component for right-click menus.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;

#[derive(Clone)]
pub struct ContextMenuItem {
    label: SharedString,
    icon: Option<SharedString>,
    disabled: bool,
    divider: bool,
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl ContextMenuItem {
    pub fn new(_id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
            divider: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = divider;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn separator() -> Self {
        Self {
            label: "".into(),
            icon: None,
            disabled: true,
            divider: true,
            on_click: None,
        }
    }
}

#[derive(IntoElement)]
pub struct ContextMenu {
    position: Point<Pixels>,
    items: Vec<ContextMenuItem>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl ContextMenu {
    pub fn new(position: Point<Pixels>) -> Self {
        Self {
            position,
            items: Vec::new(),
            on_close: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: ContextMenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: Vec<ContextMenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Styled for ContextMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let position = self.position;
        let on_close_handler = self.on_close.clone();
        let user_style = self.style;

        div()
            .absolute()
            .inset_0()
            .when(on_close_handler.is_some(), |this: Div| {
                let handler = on_close_handler.clone().unwrap();
                let handler2 = handler.clone();
                this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    handler(window, cx);
                })
                .on_mouse_down(MouseButton::Right, move |_, window, cx| {
                    handler2(window, cx);
                })
            })
            .child(
                div()
                    .absolute()
                    .occlude() // Prevent clicks from passing through
                    .left(position.x)
                    .top(position.y)
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
                    .map(|this| {
                        let mut div = this;
                        div.style().refine(&user_style);
                        div
                    })
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .children(
                        self.items.into_iter().map(|item| {
                            if item.label.is_empty() && item.divider {
                                return div()
                                    .h(px(1.0))
                                    .my(px(4.0))
                                    .bg(theme.tokens.border)
                                    .into_any_element();
                            }

                            let on_close = self.on_close.clone();
                            let handler = item.on_click.clone();
                            let disabled = item.disabled;

                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .px(px(8.0))
                                .py(px(6.0))
                                .rounded(theme.tokens.radius_sm)
                                .text_size(px(14.0))
                                .cursor(if disabled {
                                    CursorStyle::Arrow
                                } else {
                                    CursorStyle::PointingHand
                                })
                                .when(disabled, |this: Div| {
                                    this.text_color(theme.tokens.muted_foreground)
                                        .opacity(0.5)
                                })
                                .when(!disabled, |this: Div| {
                                    this.text_color(theme.tokens.popover_foreground)
                                        .hover(|style| {
                                            style.bg(theme.tokens.accent.opacity(0.1))
                                        })
                                })
                                .when(!disabled && handler.is_some(), |this: Div| {
                                    let handler = handler.unwrap();
                                    let on_close = on_close.clone();
                                    this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                        handler(window, cx);
                                        if let Some(close_handler) = &on_close {
                                            close_handler(window, cx);
                                        }
                                    })
                                })
                                .when_some(item.icon, |this: Div, _icon| {
                                    this
                                })
                                .child(item.label)
                                .into_any_element()
                        })
                    )
            )
    }
}
