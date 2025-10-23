//! Card - Content container with header, body, and footer sections.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

pub struct Card {
    header: Option<AnyElement>,
    content: Option<AnyElement>,
    footer: Option<AnyElement>,
    style: StyleRefinement,
}

impl Card {
    pub fn new() -> Self {
        Self {
            header: None,
            content: None,
            footer: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for Card {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let user_style = self.style;

        let shadow_sm = BoxShadow {
            offset: theme.tokens.shadow_sm.offset,
            blur_radius: theme.tokens.shadow_sm.blur_radius,
            spread_radius: theme.tokens.shadow_sm.spread_radius,
            color: theme.tokens.shadow_sm.color,
        };

        let mut base = div()
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_lg)
            .shadow(vec![shadow_sm])
            .overflow_hidden();

        if let Some(header) = self.header {
            base = base.child(
                div()
                    .px(px(24.0))
                    .py(px(16.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(header),
            );
        }

        if let Some(content) = self.content {
            base = base.child(
                div()
                    .px(px(24.0))
                    .py(px(16.0))
                    .child(content),
            );
        }

        if let Some(footer) = self.footer {
            base = base.child(
                div()
                    .px(px(24.0))
                    .py(px(16.0))
                    .border_t_1()
                    .border_color(theme.tokens.border)
                    .child(footer),
            );
        }

        base.map(|this| {
            let mut div = this;
            div.style().refine(&user_style);
            div
        })
    }
}
