//! Avatar component - User profile image with fallback to initials or icon.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Default for AvatarSize {
    fn default() -> Self {
        Self::Md
    }
}

impl AvatarSize {
    fn size_px(&self) -> f32 {
        match self {
            Self::Xs => 24.0,
            Self::Sm => 32.0,
            Self::Md => 40.0,
            Self::Lg => 48.0,
            Self::Xl => 64.0,
        }
    }

    fn text_size_px(&self) -> f32 {
        match self {
            Self::Xs => 10.0,
            Self::Sm => 13.0,
            Self::Md => 16.0,
            Self::Lg => 20.0,
            Self::Xl => 26.0,
        }
    }
}

#[derive(IntoElement)]
pub struct Avatar {
    src: Option<SharedString>,
    name: Option<SharedString>,
    fallback_text: Option<SharedString>,
    size: AvatarSize,
    style: StyleRefinement,
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            src: None,
            name: None,
            fallback_text: None,
            size: AvatarSize::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn fallback_text(mut self, text: impl Into<SharedString>) -> Self {
        self.fallback_text = Some(text.into());
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
    fn extract_initials(name: &str) -> String {
        let words: Vec<&str> = name.split_whitespace().collect();

        if words.len() >= 2 {
            format!(
                "{}{}",
                words[0].chars().next().unwrap_or('?').to_uppercase(),
                words[1].chars().next().unwrap_or('?').to_uppercase()
            )
        } else if let Some(first_word) = words.first() {
            first_word
                .chars()
                .take(2)
                .collect::<String>()
                .to_uppercase()
        } else {
            "??".to_string()
        }
    }

    fn name_to_color(name: &str, _theme: &crate::theme::Theme) -> Hsla {
        const COLOR_PALETTE: &[(f32, f32, f32)] = &[
            (0.0, 0.7, 0.6),
            (120.0, 0.6, 0.5),
            (240.0, 0.7, 0.6),
            (30.0, 0.7, 0.6),
            (180.0, 0.6, 0.5),
            (300.0, 0.7, 0.6),
            (60.0, 0.6, 0.5),
            (330.0, 0.7, 0.6),
        ];

        let hash: u64 = name.bytes().map(|b| b as u64).sum();
        let index = (hash as usize) % COLOR_PALETTE.len();
        let (hue, saturation, lightness) = COLOR_PALETTE[index];

        hsla(hue / 360.0, saturation, lightness, 1.0)
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Avatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Avatar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let size_px = self.size.size_px();
        let text_size_px = self.size.text_size_px();
        let user_style = self.style;

        let (content, bg_color, text_color) = if let Some(src) = self.src {
            (
                img(src)
                    .size(px(size_px))
                    .rounded_full()
                    .object_fit(ObjectFit::Cover)
                    .into_any_element(),
                theme.tokens.muted,
                theme.tokens.foreground,
            )
        } else if let Some(name) = self.name {
            let initials = Self::extract_initials(&name);
            let bg_color = Self::name_to_color(&name, &theme);
            let text_color = theme.tokens.background;

            (
                div()
                    .text_size(px(text_size_px))
                    .font_weight(FontWeight::MEDIUM)
                    .child(initials)
                    .into_any_element(),
                bg_color.opacity(0.8),
                text_color,
            )
        } else if let Some(fallback) = self.fallback_text {
            (
                div()
                    .text_size(px(text_size_px))
                    .font_weight(FontWeight::MEDIUM)
                    .child(fallback)
                    .into_any_element(),
                theme.tokens.muted,
                theme.tokens.foreground,
            )
        } else {
            (
                div()
                    .text_size(px(text_size_px * 0.7))
                    .child("?")
                    .into_any_element(),
                theme.tokens.muted,
                theme.tokens.muted_foreground,
            )
        };

        div()
            .size(px(size_px))
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .rounded_full()
            .overflow_hidden()
            .bg(bg_color)
            .text_color(text_color)
            .font_family(theme.tokens.font_family.clone())
            .border_2()
            .border_color(theme.tokens.background)
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(content)
    }
}
