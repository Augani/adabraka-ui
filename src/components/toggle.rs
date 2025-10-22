//! Toggle component - Toggle/Switch component with animations.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSize {
    Sm,
    Md,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelSide {
    Left,
    Right,
}

#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    base: Stateful<Div>,
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    label_side: LabelSide,
    on_click: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    size: ToggleSize,
    style: StyleRefinement,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            base: div().id(id),
            checked: false,
            disabled: false,
            label: None,
            label_side: LabelSide::Right,
            on_click: None,
            size: ToggleSize::Md,
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label<T: Into<SharedString>>(mut self, label: T) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn label_side(mut self, side: LabelSide) -> Self {
        self.label_side = side;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&bool, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }
}

impl Styled for Toggle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for Toggle {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Toggle {}

impl RenderOnce for Toggle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let (bg_width, bg_height, bar_width, inset) = match self.size {
            ToggleSize::Sm => (px(28.0), px(16.0), px(12.0), px(2.0)),
            ToggleSize::Md => (px(36.0), px(20.0), px(16.0), px(2.0)),
        };

        let checked = self.checked;

        let (bg, toggle_bg) = if self.disabled {
            let bg_color = if checked {
                theme.tokens.primary.opacity(0.5)
            } else {
                theme.tokens.muted
            };
            (bg_color, theme.tokens.background.opacity(0.35))
        } else if checked {
            (theme.tokens.primary, theme.tokens.background)
        } else {
            (theme.tokens.muted, theme.tokens.background)
        };

        let radius = bg_height;

        self.base
            .flex()
            .items_center()
            .gap(px(8.0))
            .when(self.label_side == LabelSide::Left, |this| this.flex_row_reverse())
            .child(
                div()
                    .w(bg_width)
                    .h(bg_height)
                    .rounded(radius)
                    .flex()
                    .items_center()
                    .bg(bg)
                    .cursor(if self.disabled {
                        CursorStyle::Arrow
                    } else {
                        CursorStyle::PointingHand
                    })
                    .when(!self.disabled, |this| {
                        this.hover(|style| {
                            if checked {
                                style.bg(theme.tokens.primary.opacity(0.9))
                            } else {
                                style.bg(theme.tokens.muted.opacity(0.8))
                            }
                        })
                    })
                    .child(toggle_thumb(
                        self.id.clone(),
                        checked,
                        toggle_bg,
                        bg_width,
                        bar_width,
                        inset,
                        radius,
                        self.disabled,
                        window,
                        cx,
                    ))
            )
            .when_some(self.label, |this, label| {
                this.child(
                    div()
                        .text_size(match self.size {
                            ToggleSize::Sm => px(13.0),
                            ToggleSize::Md => px(14.0),
                        })
                        .font_family(theme.tokens.font_family.clone())
                        .text_color(if self.disabled {
                            theme.tokens.muted_foreground
                        } else {
                            theme.tokens.foreground
                        })
                        .cursor(if self.disabled {
                            CursorStyle::Arrow
                        } else {
                            CursorStyle::PointingHand
                        })
                        .child(label)
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .when(!self.disabled, |this| {
                this.when_some(self.on_click, |this, on_click| {
                    let on_click = on_click.clone();
                    this.on_click(move |_, window, cx| {
                        let new_checked = !checked;
                        (on_click)(&new_checked, window, cx);
                    })
                })
            })
    }
}

fn toggle_thumb(
    id: ElementId,
    checked: bool,
    color: Hsla,
    bg_width: Pixels,
    bar_width: Pixels,
    inset: Pixels,
    radius: Pixels,
    disabled: bool,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let toggle_state = window.use_keyed_state(id.clone(), cx, |_, _| checked);

    div()
        .rounded(radius)
        .bg(color)
        .shadow_md()
        .size(bar_width)
        .relative()
        .map(|this| {
            let prev_checked = *toggle_state.read(cx);

            if !disabled && prev_checked != checked {
                let duration = std::time::Duration::from_millis(150);
                cx.spawn({
                    let toggle_state = toggle_state.clone();
                    async move |cx| {
                        cx.background_executor().timer(duration).await;
                        _ = toggle_state.update(cx, |state, _| *state = checked);
                    }
                })
                .detach();

                this.with_animation(
                    ElementId::NamedInteger("toggle-slide".into(), checked as u64),
                    Animation::new(duration),
                    move |this, delta| {
                        let max_x = bg_width - bar_width - inset * 2.0;
                        let x = if checked {
                            inset + max_x * delta
                        } else {
                            inset + max_x - max_x * delta
                        };
                        this.left(x)
                    },
                )
                .into_any_element()
            } else {
                let max_x = bg_width - bar_width - inset * 2.0;
                let x = if checked { inset + max_x } else { inset };
                this.left(x).into_any_element()
            }
        })
}
