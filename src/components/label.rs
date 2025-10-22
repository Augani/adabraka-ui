//! Label component - Form labels with accessibility support.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

/// Form label component with accessibility support
///
/// # Example
///
/// ```rust
/// Label::new("Email Address")
///     .required(true)
///     .helper_text("We'll never share your email")
/// ```
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    for_id: Option<ElementId>,
    required: bool,
    helper_text: Option<SharedString>,
    disabled: bool,
    style: StyleRefinement,
}

impl Label {
    /// Create a new label
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            for_id: None,
            required: false,
            helper_text: None,
            disabled: false,
            style: StyleRefinement::default(),
        }
    }

    /// Associate this label with a form control's element ID
    pub fn for_id(mut self, id: impl Into<ElementId>) -> Self {
        self.for_id = Some(id.into());
        self
    }

    /// Mark this field as required (shows asterisk)
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Add helper text below the label
    pub fn helper_text(mut self, text: impl Into<SharedString>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Label {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .text_sm()
                    .font_family(theme.tokens.font_family.clone())
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(if self.disabled {
                        theme.tokens.muted_foreground
                    } else {
                        theme.tokens.foreground
                    })
                    .line_height(relative(1.0))
                    .child(self.text)
                    .when(self.required, |this| {
                        this.child(
                            div()
                                .text_color(theme.tokens.destructive)
                                .child("*"),
                        )
                    }),
            )
            .when_some(self.helper_text, |this, helper| {
                this.child(
                    div()
                        .text_xs()
                        .font_family(theme.tokens.font_family.clone())
                        .text_color(theme.tokens.muted_foreground)
                        .line_height(relative(1.25))
                        .child(helper),
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

impl From<String> for Label {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<SharedString> for Label {
    fn from(text: SharedString) -> Self {
        Self::new(text)
    }
}
