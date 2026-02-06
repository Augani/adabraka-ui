use crate::animations::lerp_color;
use gpui::{prelude::FluentBuilder as _, *};

#[derive(IntoElement)]
pub struct GradientText {
    text: SharedString,
    start_color: Option<Hsla>,
    end_color: Option<Hsla>,
    text_size: Option<Pixels>,
    font_weight: Option<FontWeight>,
    font_family: Option<SharedString>,
    style: StyleRefinement,
}

impl GradientText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            start_color: None,
            end_color: None,
            text_size: None,
            font_weight: None,
            font_family: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn start_color(mut self, color: Hsla) -> Self {
        self.start_color = Some(color);
        self
    }

    pub fn end_color(mut self, color: Hsla) -> Self {
        self.end_color = Some(color);
        self
    }

    pub fn text_size(mut self, size: Pixels) -> Self {
        self.text_size = Some(size);
        self
    }

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn font_family(mut self, family: impl Into<SharedString>) -> Self {
        self.font_family = Some(family.into());
        self
    }
}

impl Styled for GradientText {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for GradientText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let start = self.start_color.unwrap_or(hsla(0.6, 0.8, 0.6, 1.0));
        let end = self.end_color.unwrap_or(hsla(0.8, 0.8, 0.6, 1.0));

        let chars: Vec<char> = self.text.chars().collect();
        let count = chars.len();
        let max_index = if count > 1 { count - 1 } else { 1 };

        let text_size = self.text_size;
        let font_weight = self.font_weight;
        let font_family = self.font_family;
        let user_style = self.style;

        let mut row = div().flex().flex_row().items_center().map(|mut el| {
            el.style().refine(&user_style);
            el
        });

        for (i, ch) in chars.into_iter().enumerate() {
            let t = i as f32 / max_index as f32;
            let color = lerp_color(start, end, t);
            let s: SharedString = ch.to_string().into();

            let mut span = div().text_color(color).child(s);

            if let Some(size) = text_size {
                span = span.text_size(size);
            }
            if let Some(weight) = font_weight {
                span = span.font_weight(weight);
            }
            if let Some(ref family) = font_family {
                span = span.font_family(family.clone());
            }

            row = row.child(span);
        }

        row
    }
}
