//! Checkbox component with validation and indeterminate state support.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::icon::{Icon, IconSize as IconSizeEnum},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckboxSize {
    Sm,
    Md,
}
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    base: Stateful<Div>,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    label: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    size: CheckboxSize,
    style: StyleRefinement,
    // Icon customization
    checked_icon: SharedString,
    indeterminate_icon: SharedString,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            base: div().id(id),
            checked: false,
            indeterminate: false,
            disabled: false,
            label: None,
            on_click: None,
            size: CheckboxSize::Md,
            style: StyleRefinement::default(),
            checked_icon: "check".into(),
            indeterminate_icon: "minus".into(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
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

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&bool, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn checked_icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.checked_icon = icon.into();
        self
    }

    pub fn indeterminate_icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.indeterminate_icon = icon.into();
        self
    }
}

impl Styled for Checkbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for Checkbox {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Checkbox {}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let size = match self.size {
            CheckboxSize::Sm => px(16.0),
            CheckboxSize::Md => px(20.0),
        };

        let border_radius = theme.tokens.radius_sm;
        let checked = self.checked;
        let indeterminate = self.indeterminate;

        let (bg, border, fg) = if self.disabled {
            (
                theme.tokens.muted,
                theme.tokens.muted_foreground.opacity(0.3),
                theme.tokens.muted_foreground,
            )
        } else if checked || indeterminate {
            (
                theme.tokens.primary,
                theme.tokens.primary,
                theme.tokens.primary_foreground,
            )
        } else {
            (
                theme.tokens.background,
                theme.tokens.border,
                theme.tokens.primary_foreground,
            )
        };

        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();

        let user_style = self.style;

        self.base
            .when(!self.disabled, |this| {
                this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
            })
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .size(size)
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(border_radius)
                    .cursor(if self.disabled {
                        CursorStyle::Arrow
                    } else {
                        CursorStyle::PointingHand
                    })
                    .when(self.disabled, |this| this.opacity(0.6))
                    .when(!self.disabled && !checked && !indeterminate, |this| {
                        this.hover(|style| {
                            style.border_color(theme.tokens.primary.opacity(0.5))
                        })
                    })
                    .child(checkbox_icon(
                        self.id.clone(),
                        checked,
                        indeterminate,
                        fg,
                        self.size,
                        self.checked_icon.clone(),
                        self.indeterminate_icon.clone(),
                        window,
                        cx,
                    ))
            )
            .when_some(self.label, |this, label| {
                this.child(
                    div()
                        .text_size(match self.size {
                            CheckboxSize::Sm => px(13.0),
                            CheckboxSize::Md => px(14.0),
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
            .on_mouse_down(MouseButton::Left, |_, window, _| {
                window.prevent_default();
            })
            .when(!self.disabled, |this| {
                this.when_some(self.on_click, |this, on_click| {
                    let on_click = on_click.clone();
                    this.on_click(move |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .on_click(move |_, window, cx| {
                        let new_checked = !checked;
                        (on_click)(&new_checked, window, cx);
                    })
                })
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

fn checkbox_icon(
    id: ElementId,
    checked: bool,
    indeterminate: bool,
    color: Hsla,
    size: CheckboxSize,
    checked_icon: SharedString,
    indeterminate_icon: SharedString,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let toggle_state = window.use_keyed_state(id.clone(), cx, |_, _| (checked, indeterminate));

    let icon_size = match size {
        CheckboxSize::Sm => px(10.0),
        CheckboxSize::Md => px(14.0),
    };

    let (prev_checked, prev_indeterminate) = *toggle_state.read(cx);

    let needs_animation = prev_checked != checked || prev_indeterminate != indeterminate;

    if needs_animation {
        let duration = std::time::Duration::from_millis(150);
        cx.spawn({
            let toggle_state = toggle_state.clone();
            async move |cx| {
                cx.background_executor().timer(duration).await;
                _ = toggle_state.update(cx, |state, _| {
                    *state = (checked, indeterminate);
                });
            }
        })
        .detach();
    }

    let opacity = if needs_animation {
        if checked || indeterminate { 0.0 } else { 1.0 }
    } else {
        if checked || indeterminate { 1.0 } else { 0.0 }
    };

    let icon_name = if checked && !indeterminate {
        Some(checked_icon)
    } else if indeterminate {
        Some(indeterminate_icon)
    } else {
        None
    };

    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .opacity(opacity)
        .when_some(icon_name, |this, icon| {
            this.child(
                Icon::new(icon.as_ref())
                    .size(IconSizeEnum::Custom(icon_size))
                    .color(color)
            )
        })
}
