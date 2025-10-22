//! Icon component - SVG icon rendering with named icon support.

use gpui::{prelude::*, *};
use crate::theme::use_theme;
use crate::icon_config::resolve_icon_path;
use crate::components::icon_source::IconSource;

/// Icon variant - currently for API compatibility, not yet affecting rendering
/// TODO: Implement different icon styles or remove if not needed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconVariant {
    Regular,
    Solid,
}

impl Default for IconVariant {
    fn default() -> Self {
        Self::Regular
    }
}

fn icon_path_from_name(name: &str) -> String {
    resolve_icon_path(name)
}

pub struct Icon {
    source: IconSource,
    variant: IconVariant,
    size: Pixels,
    color: Option<Hsla>,
    clickable: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + Send + Sync + 'static>>,
    focus_handle: Option<FocusHandle>,
}

impl Icon {
    pub fn new(source: impl Into<IconSource>) -> Self {
        Self {
            source: source.into(),
            variant: IconVariant::default(),
            size: px(16.0),
            color: None,
            clickable: false,
            disabled: false,
            on_click: None,
            focus_handle: None,
        }
    }

    pub fn variant(mut self, variant: IconVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(f));
        self.clickable = true;
        self
    }

    fn get_svg_path(&self) -> Option<SharedString> {
        match &self.source {
            IconSource::FilePath(path) => Some(path.clone()),
            IconSource::Named(name) => {
                Some(SharedString::from(icon_path_from_name(name)))
            }
        }
    }
}

impl IntoElement for Icon {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let color = self.color.unwrap_or(theme.tokens.primary);
        let svg_content = self.get_svg_path();

        // For non-clickable icons, return minimal wrapper
        if !self.clickable {
            return svg()
                .flex_shrink_0()
                .when_some(svg_content, |this, svg_string| {
                    this.path(svg_string)
                })
                .size(self.size)
                .text_color(if self.disabled {
                    theme.tokens.muted_foreground
                } else {
                    color
                })
                .into_any_element();
        }

        // For clickable icons, wrap in interactive Div
        let on_click = self.on_click;
        let disabled = self.disabled;

        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .cursor(if disabled {
                CursorStyle::Arrow
            } else {
                CursorStyle::PointingHand
            })
            .when_some(self.focus_handle, |div, handle| {
                div.track_focus(&handle)
            })
            .when(!disabled && on_click.is_some(), |div| {
                div.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    if let Some(ref cb) = on_click {
                        cb(window, cx);
                    }
                })
            })
            .when(!disabled, |div| {
                div.hover(|mut style| {
                    style.opacity = Some(0.7);
                    style
                })
            })
            .when_some(svg_content, |div, svg_string| {
                div.child(
                    svg()
                        .path(svg_string)
                        .size(self.size)
                        .text_color(if disabled {
                            theme.tokens.muted_foreground
                        } else {
                            color
                        })
                )
            })
            .into_any_element()
    }
}

pub fn icon(source: impl Into<IconSource>) -> Icon {
    Icon::new(source)
}

pub fn icon_button<F>(source: impl Into<IconSource>, on_click: F) -> Icon
where
    F: Fn(&mut Window, &mut App) + Send + Sync + 'static,
{
    Icon::new(source).clickable(true).on_click(on_click)
}
