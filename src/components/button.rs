//! Button component with multiple variants and sizes.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;
use crate::components::text::{Text, TextVariant};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    Icon,
}
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl Button {
    pub fn new<T: Into<SharedString>>(label: T) -> Self {
        let label = label.into();
        let id = ElementId::Name(SharedString::from(format!("button-{}", label)));

        Self {
            id: id.clone(),
            base: div().flex_shrink_0().id(id),
            label,
            variant: ButtonVariant::Default,
            size: ButtonSize::Md,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
    fn clickable(&self) -> bool {
        !self.disabled && self.on_click.is_some()
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Button {}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let (height, px_h, text_size) = match self.size {
            ButtonSize::Sm => (px(36.0), px(12.0), px(13.0)),
            ButtonSize::Md => (px(40.0), px(16.0), px(14.0)),
            ButtonSize::Lg => (px(44.0), px(20.0), px(15.0)),
            ButtonSize::Icon => (px(40.0), px(10.0), px(14.0)),
        };

        let (bg, fg, border, hover_bg, hover_fg) = match self.variant {
            ButtonVariant::Default => (
                theme.tokens.primary,
                theme.tokens.primary_foreground,
                theme.tokens.primary,
                theme.tokens.primary.opacity(0.9),
                theme.tokens.primary_foreground,
            ),
            ButtonVariant::Secondary => (
                theme.tokens.secondary,
                theme.tokens.secondary_foreground,
                theme.tokens.secondary,
                theme.tokens.secondary.opacity(0.8),
                theme.tokens.secondary_foreground,
            ),
            ButtonVariant::Destructive => (
                theme.tokens.destructive,
                theme.tokens.destructive_foreground,
                theme.tokens.destructive,
                theme.tokens.destructive.opacity(0.9),
                theme.tokens.destructive_foreground,
            ),
            ButtonVariant::Outline => (
                gpui::transparent_black(),
                theme.tokens.foreground,
                theme.tokens.border,
                theme.tokens.accent,
                theme.tokens.accent_foreground,
            ),
            ButtonVariant::Ghost => (
                gpui::transparent_black(),
                theme.tokens.foreground,
                gpui::transparent_black(),
                theme.tokens.accent,
                theme.tokens.accent_foreground,
            ),
            ButtonVariant::Link => (
                gpui::transparent_black(),
                theme.tokens.primary,
                gpui::transparent_black(),
                gpui::transparent_black(),
                theme.tokens.primary.opacity(0.8),
            ),
        };

        let clickable = self.clickable();
        let handler = self.on_click.clone();

        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();

        let label_text = Text::new(self.label.clone())
            .variant(TextVariant::Custom)
            .size(text_size)
            .weight(FontWeight::MEDIUM)
            .font(theme.tokens.font_family.clone())
            .color(fg);

        self.base
            .when(!self.disabled, |this| {
                this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
            })
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .h(height)
            .px(px_h)
            .rounded(theme.tokens.radius_md)
            .text_color(fg)
            .bg(bg)
            .when(self.variant != ButtonVariant::Link && self.variant != ButtonVariant::Ghost, |this| {
                this.shadow(vec![theme.tokens.shadow_xs])
            })
            .when(self.variant == ButtonVariant::Outline, |this| {
                this.border_1().border_color(border)
            })
            .when(self.disabled, |this| {
                this.opacity(0.5)
                    .cursor(CursorStyle::Arrow)
            })
            .when(!self.disabled, |this| {
                let variant = self.variant;
                let shadow_sm = theme.tokens.shadow_sm;
                this.cursor(CursorStyle::PointingHand)
                    .hover(move |style| {
                        let hover_style = style.bg(hover_bg).text_color(hover_fg);
                        if variant != ButtonVariant::Link && variant != ButtonVariant::Ghost {
                            hover_style.shadow(vec![shadow_sm])
                        } else {
                            hover_style
                        }
                    })
            })
            .on_mouse_down(MouseButton::Left, |_, window, _| {
                window.prevent_default();
            })
            .when_some(handler.filter(|_| clickable), |this, on_click| {
                this.on_click(move |event, window, cx| {
                    cx.stop_propagation();
                    (on_click)(event, window, cx);
                })
            })
            .child(
                div()
                    .when(self.variant == ButtonVariant::Link, |this| this.underline())
                    .child(label_text)
            )
    }
}
