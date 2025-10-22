//! Icon button component for icon-only actions with multiple variants.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;
use crate::components::button::ButtonVariant;
use crate::components::icon_source::IconSource;
use crate::icon_config::resolve_icon_path;

fn icon_path_from_name(name: &str) -> String {
    resolve_icon_path(name)
}

#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    base: Stateful<Div>,
    icon_source: IconSource,
    variant: ButtonVariant,
    size: Pixels,
    icon_size: Option<Pixels>,
    disabled: bool,
    no_background: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl IconButton {
    pub fn new(icon: impl Into<IconSource>) -> Self {
        let icon_source = icon.into();

        let id_string = match &icon_source {
            IconSource::Named(name) => format!("icon-button-{}", name),
            IconSource::FilePath(path) => format!("icon-button-{}", path),
        };
        let id = ElementId::Name(SharedString::from(id_string));

        Self {
            id: id.clone(),
            base: div().flex_shrink_0().id(id),
            icon_source,
            variant: ButtonVariant::Default,
            size: px(40.0),
            icon_size: None,
            disabled: false,
            no_background: false,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn icon_size(mut self, size: Pixels) -> Self {
        self.icon_size = Some(size);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn no_background(mut self, no_background: bool) -> Self {
        self.no_background = no_background;
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

    fn get_svg_path(&self) -> Option<SharedString> {
        match &self.icon_source {
            IconSource::FilePath(path) => Some(path.clone()),
            IconSource::Named(name) => {
                Some(SharedString::from(icon_path_from_name(name)))
            }
        }
    }
}

impl Styled for IconButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for IconButton {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for IconButton {}

impl RenderOnce for IconButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let icon_size = self.icon_size.unwrap_or(self.size * 0.5);

        let (bg, fg, border, hover_bg, _hover_fg) = match self.variant {
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
        let svg_path = self.get_svg_path();
        let user_style = self.style;

        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();

        self.base
            .when(!self.disabled, |this| {
                this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
            })
            .flex()
            .items_center()
            .justify_center()
            .size(self.size)
            .rounded(theme.tokens.radius_md)
            .when(!self.no_background, |this| {
                this.bg(bg)
                    .when(self.variant == ButtonVariant::Outline, |this| {
                        this.border_1().border_color(border)
                    })
            })
            .when(self.disabled, |this| {
                this.opacity(0.5).cursor(CursorStyle::Arrow)
            })
            .when(!self.disabled, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .when(!self.no_background, |this| {
                        this.hover(|style| style.bg(hover_bg))
                    })
                    .when(self.no_background, |this| {
                        this.hover(|style| style.opacity(0.7))
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
            .when_some(svg_path, |this, path| {
                this.child(
                    svg()
                        .path(path)
                        .size(icon_size)
                        .text_color(if self.disabled {
                            theme.tokens.muted_foreground
                        } else if self.no_background {
                            theme.tokens.primary
                        } else {
                            fg
                        })
                )
            })
    }
}
