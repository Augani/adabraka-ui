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

/// Icon size variants - supports named sizes and custom pixel values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    /// Extra small: 12px (size_3 in GPUI)
    XSmall,
    /// Small: 14px (size_3p5 in GPUI)
    Small,
    /// Medium: 16px (size_4 in GPUI) - Default
    Medium,
    /// Large: 24px (size_6 in GPUI)
    Large,
    /// Custom pixel size
    Custom(Pixels),
}

impl Default for IconSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl From<Pixels> for IconSize {
    fn from(pixels: Pixels) -> Self {
        Self::Custom(pixels)
    }
}

impl From<f32> for IconSize {
    fn from(value: f32) -> Self {
        Self::Custom(px(value))
    }
}

impl IconSize {
    /// Convert IconSize to Pixels
    pub fn to_pixels(&self) -> Pixels {
        match self {
            IconSize::XSmall => px(12.0),
            IconSize::Small => px(14.0),
            IconSize::Medium => px(16.0),
            IconSize::Large => px(24.0),
            IconSize::Custom(pixels) => *pixels,
        }
    }
}

fn icon_path_from_name(name: &str) -> String {
    resolve_icon_path(name)
}

pub struct Icon {
    source: IconSource,
    variant: IconVariant,
    size: IconSize,
    color: Option<Hsla>,
    clickable: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + Send + Sync + 'static>>,
    focus_handle: Option<FocusHandle>,
    style: StyleRefinement,
    rotation: Option<Radians>,
}

impl Icon {
    pub fn new(source: impl Into<IconSource>) -> Self {
        Self {
            source: source.into(),
            variant: IconVariant::default(),
            size: IconSize::default(),
            color: None,
            clickable: false,
            disabled: false,
            on_click: None,
            focus_handle: None,
            style: StyleRefinement::default(),
            rotation: None,
        }
    }

    pub fn variant(mut self, variant: IconVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: impl Into<IconSize>) -> Self {
        self.size = size.into();
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

    /// Rotate the icon by the given angle in radians
    pub fn rotate(mut self, radians: impl Into<Radians>) -> Self {
        self.rotation = Some(radians.into());
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

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
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
            let mut base = svg();
            *base.style() = self.style;

            return base
                .flex_shrink_0()
                .when_some(svg_content, |this, svg_string| {
                    this.path(svg_string)
                })
                .size(self.size.to_pixels())
                .text_color(if self.disabled {
                    theme.tokens.muted_foreground
                } else {
                    color
                })
                .when_some(self.rotation, |this, rotation| {
                    this.with_transformation(Transformation::rotate(rotation))
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
                let mut icon_svg = svg();
                *icon_svg.style() = self.style.clone();

                div.child(
                    icon_svg
                        .path(svg_string)
                        .size(self.size.to_pixels())
                        .text_color(if disabled {
                            theme.tokens.muted_foreground
                        } else {
                            color
                        })
                        .when_some(self.rotation, |this, rotation| {
                            this.with_transformation(Transformation::rotate(rotation))
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
