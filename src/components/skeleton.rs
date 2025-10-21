//! Skeleton component - Loading placeholder with pulsing animation effect.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rect,
}

impl Default for SkeletonVariant {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(IntoElement)]
pub struct Skeleton {
    base: Div,
    variant: SkeletonVariant,
    secondary: bool,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            base: div(),
            variant: SkeletonVariant::default(),
            secondary: false,
        }
    }

    pub fn variant(mut self, variant: SkeletonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn secondary(mut self, secondary: bool) -> Self {
        self.secondary = secondary;
        self
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let base_color = if self.secondary {
            theme.tokens.muted.opacity(0.5)
        } else {
            theme.tokens.muted
        };

        self.base
            .when(
                self.variant == SkeletonVariant::Text,
                |this| this.w_full().h(px(16.0)).rounded(theme.tokens.radius_md),
            )
            .when(
                self.variant == SkeletonVariant::Circle,
                |this| this.rounded_full(),
            )
            .when(
                self.variant == SkeletonVariant::Rect,
                |this| this.rounded(theme.tokens.radius_md),
            )
            .bg(base_color)
            .with_animation(
                "skeleton-pulse",
                Animation::new(Duration::from_secs(2))
                    .repeat()
                    .with_easing(ease_in_out),
                move |this, delta| {
                    let opacity = 1.0 - (delta * 0.3);
                    this.opacity(opacity)
                },
            )
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveElement for Skeleton {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Skeleton {}

impl ParentElement for Skeleton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements)
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}
