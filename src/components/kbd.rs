use crate::theme::use_theme;
use gpui::*;
use smallvec::smallvec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KBDSize {
    Sm,
    #[default]
    Md,
    Lg,
}

pub struct KBD {
    label: SharedString,
    size: KBDSize,
    style: StyleRefinement,
}

impl KBD {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            size: KBDSize::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: KBDSize) -> Self {
        self.size = size;
        self
    }
}

impl Styled for KBD {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for KBD {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let user_style = self.style;

        let (text_size, px_val, py_val, min_w) = match self.size {
            KBDSize::Sm => (px(10.0), px(4.0), px(1.0), px(16.0)),
            KBDSize::Md => (px(11.0), px(6.0), px(2.0), px(20.0)),
            KBDSize::Lg => (px(12.0), px(8.0), px(3.0), px(24.0)),
        };

        let mut el = div()
            .flex()
            .items_center()
            .justify_center()
            .px(px_val)
            .py(py_val)
            .min_w(min_w)
            .bg(theme.tokens.muted)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_sm)
            .text_size(text_size)
            .font_family(theme.tokens.font_mono.clone())
            .text_color(theme.tokens.muted_foreground)
            .font_weight(FontWeight::MEDIUM)
            .line_height(relative(1.0))
            .shadow(smallvec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.15),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(0.0),
                spread_radius: px(0.0),
                inset: false,
            }])
            .child(self.label);
        el.style().refine(&user_style);
        el
    }
}
