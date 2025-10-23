//! Badge component - Status labels and tags.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Warning,
}

pub struct Badge {
    label: SharedString,
    variant: BadgeVariant,
    style: StyleRefinement,
}

impl Badge {
    pub fn new<T: Into<SharedString>>(label: T) -> Self {
        Self {
            label: label.into(),
            variant: BadgeVariant::Default,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Styled for Badge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for Badge {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let user_style = self.style;

        let (bg_color, fg_color, border_color) = match self.variant {
            BadgeVariant::Default => (
                theme.tokens.primary,
                theme.tokens.primary_foreground,
                gpui::transparent_black(),
            ),
            BadgeVariant::Secondary => (
                theme.tokens.secondary,
                theme.tokens.secondary_foreground,
                gpui::transparent_black(),
            ),
            BadgeVariant::Destructive => (
                theme.tokens.destructive,
                theme.tokens.destructive_foreground,
                gpui::transparent_black(),
            ),
            BadgeVariant::Outline => (
                gpui::transparent_black(),
                theme.tokens.foreground,
                theme.tokens.border,
            ),
            BadgeVariant::Warning => (
                gpui::hsla(38.0 / 360.0, 0.92, 0.55, 1.0),
                gpui::hsla(0.0, 0.0, 0.0, 1.0),
                gpui::transparent_black(),
            ),
        };

        div()
            .flex()
            .items_center()
            .px(px(10.0))
            .py(px(2.0))
            .rounded_full()
            .text_size(px(12.0))
            .font_family(theme.tokens.font_family.clone())
            .font_weight(FontWeight::MEDIUM)
            .bg(bg_color)
            .text_color(fg_color)
            .when(self.variant == BadgeVariant::Outline, |el| {
                el.border_1().border_color(border_color)
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(self.label)
    }
}
