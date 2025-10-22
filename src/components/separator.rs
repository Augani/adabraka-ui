//! Separator component - Visual dividers for content sections.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

/// Orientation of the separator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOrientation {
    /// Horizontal separator (default)
    Horizontal,
    /// Vertical separator
    Vertical,
}

impl Default for SeparatorOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Visual divider for separating content
///
/// # Example
///
/// ```rust
/// // Horizontal separator
/// Separator::new()
///
/// // Vertical separator
/// Separator::new()
///     .orientation(SeparatorOrientation::Vertical)
///
/// // With label
/// Separator::new()
///     .label("OR")
/// ```
#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
    label: Option<SharedString>,
    color: Option<Hsla>,
    decorative: bool,
    style: StyleRefinement,
}

impl Separator {
    /// Create a new horizontal separator
    pub fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::default(),
            label: None,
            color: None,
            decorative: true,
            style: StyleRefinement::default(),
        }
    }

    /// Create a horizontal separator
    pub fn horizontal() -> Self {
        Self::new()
    }

    /// Create a vertical separator
    pub fn vertical() -> Self {
        Self::new().orientation(SeparatorOrientation::Vertical)
    }

    /// Set the orientation
    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set an optional label to display on the separator
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set custom color for the separator line
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set whether this is a decorative separator (for accessibility)
    /// Decorative separators are hidden from screen readers
    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Separator {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let line_color = self.color.unwrap_or(theme.tokens.border);
        let user_style = self.style;

        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .relative()
            .when(
                self.orientation == SeparatorOrientation::Horizontal,
                |this| this.w_full().h(px(1.0)),
            )
            .when(
                self.orientation == SeparatorOrientation::Vertical,
                |this| this.h_full().w(px(1.0)),
            )
            .child(
                div()
                    .absolute()
                    .when(
                        self.orientation == SeparatorOrientation::Horizontal,
                        |this| this.w_full().h(px(1.0)),
                    )
                    .when(
                        self.orientation == SeparatorOrientation::Vertical,
                        |this| this.h_full().w(px(1.0)),
                    )
                    .bg(line_color),
            )
            .when_some(self.label, |this, label| {
                this.child(
                    div()
                        .px(px(8.0))
                        .py(px(4.0))
                        .text_xs()
                        .bg(theme.tokens.background)
                        .text_color(theme.tokens.muted_foreground)
                        .child(label),
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}
