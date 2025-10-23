//! Popover menu component with positioned menu items.

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::rc::Rc;
use crate::theme::use_theme;
use crate::components::icon::Icon;

pub struct PopoverMenuItem {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<SharedString>,
    pub on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    pub disabled: bool,
}

impl PopoverMenuItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            on_click: None,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
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
}

#[derive(IntoElement)]
pub struct PopoverMenu {
    position: Point<Pixels>,
    items: Vec<PopoverMenuItem>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl PopoverMenu {
    pub fn new(position: Point<Pixels>, items: Vec<PopoverMenuItem>) -> Self {
        Self {
            position,
            items,
            on_close: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Styled for PopoverMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PopoverMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let on_close_backdrop = self.on_close.clone();
        let user_style = self.style;

        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                if let Some(ref handler) = on_close_backdrop {
                    handler(window, cx);
                }
            })
            .child(
                deferred(
                    anchored()
                        .snap_to_window()
                        .position(self.position)
                        .child(
                            div()
                                .occlude()
                                .child(
                                    div()
                                        .min_w(px(200.0))
                                        .max_w(px(300.0))
                                        .flex()
                                        .flex_col()
                                        .bg(theme.tokens.popover)
                                        .text_color(theme.tokens.popover_foreground)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_md)
                                        .shadow_lg()
                                        .p(px(4.0))
                                        .map(|this| {
                                            let mut div = this;
                                            div.style().refine(&user_style);
                                            div
                                        })
                                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                            cx.stop_propagation();
                                        })
                                        .children(self.items.into_iter().map(|item| {
                            let on_click = item.on_click;
                            let disabled = item.disabled;

                            div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(4.0))
                            .cursor(if disabled {
                                CursorStyle::Arrow
                            } else {
                                CursorStyle::PointingHand
                            })
                            .when(!disabled, |this| {
                                this.hover(|style| {
                                    style.bg(theme.tokens.accent.opacity(0.1))
                                })
                            })
                            .when(disabled, |this| {
                                this.opacity(0.5)
                            })
                            .when_some(item.icon, |this, icon_name| {
                                this.child(
                                    Icon::new(icon_name)
                                        .size(px(16.0))
                                        .color(theme.tokens.foreground)
                                )
                            })
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(item.label)
                            )
                            .when(!disabled && on_click.is_some(), |this| {
                                this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    if let Some(ref handler) = on_click {
                                        handler(window, cx);
                                    }
                                    cx.stop_propagation();
                                })
                            })
                    }))
                                )
                        )
                ).with_priority(1)
            )
    }
}
