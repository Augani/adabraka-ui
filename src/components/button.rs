//! Button component with multiple variants and sizes.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;
use crate::components::text::{Text, TextVariant};
use crate::components::icon_source::IconSource;
use crate::icon_config::resolve_icon_path;

/// Render an icon from IconSource
fn render_icon(icon_src: IconSource, size: Pixels, color: Hsla) -> impl IntoElement {
    let svg_path = match icon_src {
        IconSource::FilePath(path) => path,
        IconSource::Named(name) => SharedString::from(resolve_icon_path(&name)),
    };

    svg()
        .path(svg_path)
        .size(size)
        .text_color(color)
}

/// Render a loading spinner
fn render_loading_spinner(size: Pixels, color: Hsla) -> impl IntoElement {
    div()
        .size(size)
        .child(
            svg()
                .path("assets/icons/loader.svg")
                .size(size)
                .text_color(color)
        )
}

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
    selected: bool,
    loading: bool,
    icon: Option<IconSource>,
    icon_position: IconPosition,
    tooltip: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IconPosition {
    Start,
    End,
}

impl Button {
    /// Create a new button with a unique ID and label.
    ///
    /// # Example
    /// ```
    /// Button::new("my-button", "Click me")
    /// ```
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let id = id.into();
        let label = label.into();

        Self {
            id: id.clone(),
            base: div().flex_shrink_0().id(id),
            label,
            variant: ButtonVariant::Default,
            size: ButtonSize::Md,
            disabled: false,
            selected: false,
            loading: false,
            icon: None,
            icon_position: IconPosition::Start,
            tooltip: None,
            on_click: None,
            style: StyleRefinement::default(),
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

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn icon_position(mut self, position: IconPosition) -> Self {
        self.icon_position = position;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
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
        !self.disabled && !self.loading && self.on_click.is_some()
    }

}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
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

        let (bg, fg, border, hover_bg, hover_fg, has_shadow) = match self.variant {
            ButtonVariant::Default => (
                theme.tokens.primary,
                theme.tokens.primary_foreground,
                theme.tokens.primary,
                theme.tokens.primary.opacity(0.9),
                theme.tokens.primary_foreground,
                true,
            ),
            ButtonVariant::Secondary => (
                theme.tokens.secondary,
                theme.tokens.secondary_foreground,
                theme.tokens.secondary,
                theme.tokens.secondary.opacity(0.8),
                theme.tokens.secondary_foreground,
                true,
            ),
            ButtonVariant::Destructive => (
                theme.tokens.destructive,
                theme.tokens.destructive_foreground,
                theme.tokens.destructive,
                theme.tokens.destructive.opacity(0.9),
                theme.tokens.destructive_foreground,
                true,
            ),
            ButtonVariant::Outline => (
                gpui::transparent_black(),
                theme.tokens.foreground,
                theme.tokens.border,
                theme.tokens.accent,
                theme.tokens.accent_foreground,
                false,
            ),
            ButtonVariant::Ghost => (
                gpui::transparent_black(),
                theme.tokens.foreground,
                gpui::transparent_black(),
                theme.tokens.accent,
                theme.tokens.accent_foreground,
                false,
            ),
            ButtonVariant::Link => (
                gpui::transparent_black(),
                theme.tokens.primary,
                gpui::transparent_black(),
                gpui::transparent_black(),
                theme.tokens.primary.opacity(0.8),
                false,
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

        let icon_size = text_size * 1.2;
        let icon = self.icon.clone();
        let icon_pos = self.icon_position;
        let is_loading = self.loading;
        let is_selected = self.selected;
        let user_style = self.style;

        self.base
            .when(!self.disabled && !is_loading, |this| {
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
            .when(has_shadow, |this| {
                this.shadow(vec![theme.tokens.shadow_xs])
            })
            .when(self.variant == ButtonVariant::Outline, |this| {
                this.border_1().border_color(border)
            })
            .when(is_selected && !self.disabled, |this| {
                this.bg(theme.tokens.accent)
                    .text_color(theme.tokens.accent_foreground)
                    .border_color(theme.tokens.accent)
            })
            .when(is_loading, |this| {
                this.opacity(0.7)
                    .cursor(CursorStyle::Arrow)
            })
            .when(self.disabled && !is_loading, |this| {
                this.opacity(0.5)
                    .cursor(CursorStyle::Arrow)
            })
            .when(!self.disabled && !is_loading, |this| {
                let shadow_sm = theme.tokens.shadow_sm;
                this.cursor(CursorStyle::PointingHand)
                    .hover(move |style| {
                        let hover_style = style.bg(hover_bg).text_color(hover_fg);
                        if has_shadow {
                            hover_style.shadow(vec![shadow_sm])
                        } else {
                            hover_style
                        }
                    })
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
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
                    .flex()
                    .items_center()
                    .gap_2()
                    .when(icon_pos == IconPosition::Start && !is_loading, |this| {
                        this.when_some(icon.clone(), |this, icon_src| {
                            this.child(render_icon(icon_src, icon_size, fg))
                        })
                    })
                    .when(is_loading && icon_pos == IconPosition::Start, |this| {
                        this.child(render_loading_spinner(icon_size, fg))
                    })
                    .child(
                        div()
                            .when(self.variant == ButtonVariant::Link, |this| this.underline())
                            .child(label_text)
                    )
                    .when(icon_pos == IconPosition::End && !is_loading, |this| {
                        this.when_some(icon.clone(), |this, icon_src| {
                            this.child(render_icon(icon_src, icon_size, fg))
                        })
                    })
                    .when(is_loading && icon_pos == IconPosition::End, |this| {
                        this.child(render_loading_spinner(icon_size, fg))
                    })
            )
    }
}
