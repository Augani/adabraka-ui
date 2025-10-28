//! Text component - Typography with theming and semantic variants.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

/// Text variants for semantic typography
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextVariant {
    /// Extra large heading (32px, bold)
    H1,
    /// Large heading (28px, semibold)
    H2,
    /// Medium heading (24px, semibold)
    H3,
    /// Small heading (20px, semibold)
    H4,
    /// Extra small heading (18px, medium)
    H5,
    /// Tiny heading (16px, medium)
    H6,
    /// Body text - large (16px, regular)
    BodyLarge,
    /// Body text - default (14px, regular)
    Body,
    /// Body text - small (13px, regular)
    BodySmall,
    /// Caption text (12px, regular)
    Caption,
    /// Label text (14px, medium)
    Label,
    /// Label text - small (12px, medium)
    LabelSmall,
    /// Code/monospace text (14px, mono font)
    Code,
    /// Code/monospace - small (12px, mono font)
    CodeSmall,
    /// Custom - use size(), weight(), etc. for full control
    Custom,
}

impl TextVariant {
    /// Get the text size for this variant
    pub fn size(&self) -> Pixels {
        match self {
            Self::H1 => px(32.0),
            Self::H2 => px(28.0),
            Self::H3 => px(24.0),
            Self::H4 => px(20.0),
            Self::H5 => px(18.0),
            Self::H6 => px(16.0),
            Self::BodyLarge => px(16.0),
            Self::Body => px(14.0),
            Self::BodySmall => px(13.0),
            Self::Caption => px(12.0),
            Self::Label => px(14.0),
            Self::LabelSmall => px(12.0),
            Self::Code => px(14.0),
            Self::CodeSmall => px(12.0),
            Self::Custom => px(14.0), // Default for custom
        }
    }

    /// Get the font weight for this variant
    pub fn weight(&self) -> FontWeight {
        match self {
            Self::H1 => FontWeight::BOLD,
            Self::H2 | Self::H3 | Self::H4 => FontWeight::SEMIBOLD,
            Self::H5 | Self::H6 | Self::Label | Self::LabelSmall => FontWeight::MEDIUM,
            Self::BodyLarge | Self::Body | Self::BodySmall | Self::Caption => FontWeight::NORMAL,
            Self::Code | Self::CodeSmall => FontWeight::NORMAL,
            Self::Custom => FontWeight::NORMAL,
        }
    }

    /// Check if this variant uses monospace font
    pub fn is_mono(&self) -> bool {
        matches!(self, Self::Code | Self::CodeSmall)
    }

    /// Get line height multiplier for this variant
    pub fn line_height(&self) -> f32 {
        match self {
            Self::H1 | Self::H2 | Self::H3 | Self::H4 => 1.2,
            Self::H5 | Self::H6 => 1.3,
            Self::BodyLarge | Self::Body | Self::BodySmall => 1.5,
            Self::Caption | Self::Label | Self::LabelSmall => 1.4,
            Self::Code | Self::CodeSmall => 1.6,
            Self::Custom => 1.5,
        }
    }
}

/// Text component with automatic theming and typography
#[derive(IntoElement)]
pub struct Text {
    content: SharedString,
    variant: TextVariant,
    size: Option<Pixels>,
    weight: Option<FontWeight>,
    color: Option<Hsla>,
    font: Option<SharedString>,
    line_height: Option<f32>,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    wrap: bool,
    truncate: bool,
    style: StyleRefinement,
}

impl Text {
    /// Create new text with content
    pub fn new<S: Into<SharedString>>(content: S) -> Self {
        Self {
            content: content.into(),
            variant: TextVariant::Body,
            size: None,
            weight: None,
            color: None,
            font: None,
            line_height: None,
            italic: false,
            underline: false,
            strikethrough: false,
            wrap: true,
            truncate: false,
            style: StyleRefinement::default(),
        }
    }

    /// Set the text variant (heading, body, etc.)
    pub fn variant(mut self, variant: TextVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set custom font size (overrides variant size)
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = Some(size);
        self
    }

    /// Set custom font weight (overrides variant weight)
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Set text color (overrides theme foreground)
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Set custom font family (overrides theme font)
    pub fn font(mut self, font: impl Into<SharedString>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Set custom line height multiplier
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(line_height);
        self
    }

    /// Make text italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Add underline
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Add strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Disable text wrapping (single line)
    pub fn no_wrap(mut self) -> Self {
        self.wrap = false;
        self
    }

    /// Enable text truncation with ellipsis
    pub fn truncate(mut self) -> Self {
        self.truncate = true;
        self.wrap = false; // Truncate requires no wrap
        self
    }

    /// Get the effective text size
    fn effective_size(&self) -> Pixels {
        self.size.unwrap_or_else(|| self.variant.size())
    }

    /// Get the effective font weight
    fn effective_weight(&self) -> FontWeight {
        self.weight.unwrap_or_else(|| self.variant.weight())
    }

    /// Get the effective line height
    fn effective_line_height(&self) -> f32 {
        self.line_height.unwrap_or_else(|| self.variant.line_height())
    }
}

impl Styled for Text {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let size = self.effective_size();
        let weight = self.effective_weight();
        let line_height = self.effective_line_height();

        let font_family = if let Some(font) = self.font {
            font
        } else if self.variant.is_mono() {
            theme.tokens.font_mono.clone()
        } else {
            theme.tokens.font_family.clone()
        };

        let text_color = self.color.unwrap_or(theme.tokens.foreground);

        let mut base = div();
        *base.style() = self.style;

        let needs_highlights = self.italic || self.strikethrough;
        let styled_text = if needs_highlights {
            let mut highlight_style = HighlightStyle::default();

            if self.italic {
                highlight_style.font_style = Some(FontStyle::Italic);
            }
            if self.strikethrough {
                highlight_style.strikethrough = Some(StrikethroughStyle {
                    color: Some(text_color),
                    thickness: px(1.0),
                });
            }

            let text_len = self.content.len();
            StyledText::new(self.content.clone())
                .with_highlights(vec![(0..text_len, highlight_style)])
        } else {
            StyledText::new(self.content.clone())
        };

        base
            .font_family(font_family)
            .text_size(size)
            .font_weight(weight)
            .text_color(text_color)
            .line_height(relative(line_height))
            .when(self.underline, |this| this.underline())
            .when(!self.wrap, |this| this.whitespace_nowrap())
            .when(self.truncate, |this| this.overflow_hidden().text_ellipsis())
            .child(styled_text)
    }
}

/// Builder-style shortcuts for common text patterns

/// Create heading 1 text
pub fn h1<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::H1)
}

/// Create heading 2 text
pub fn h2<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::H2)
}

/// Create heading 3 text
pub fn h3<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::H3)
}

/// Create heading 4 text
pub fn h4<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::H4)
}

/// Create heading 5 text
pub fn h5<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::H5)
}

/// Create heading 6 text
pub fn h6<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::H6)
}

/// Create body text (default)
pub fn body<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::Body)
}

/// Create large body text
pub fn body_large<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::BodyLarge)
}

/// Create small body text
pub fn body_small<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::BodySmall)
}

/// Create caption text
pub fn caption<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::Caption)
}

/// Create label text
pub fn label<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::Label)
}

/// Create small label text
pub fn label_small<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::LabelSmall)
}

/// Create code/monospace text
pub fn code<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::Code)
}

/// Create small code text
pub fn code_small<S: Into<SharedString>>(content: S) -> Text {
    Text::new(content).variant(TextVariant::CodeSmall)
}

/// Create muted text (secondary color)
pub fn muted<S: Into<SharedString>>(content: S) -> Text {
    let theme = use_theme();
    Text::new(content)
        .variant(TextVariant::Body)
        .color(theme.tokens.muted_foreground)
}

/// Create muted small text
pub fn muted_small<S: Into<SharedString>>(content: S) -> Text {
    let theme = use_theme();
    Text::new(content)
        .variant(TextVariant::BodySmall)
        .color(theme.tokens.muted_foreground)
}
