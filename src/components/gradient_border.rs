use crate::theme::use_theme;
use gpui::*;

#[derive(IntoElement)]
pub struct GradientBorder {
    base: Div,
    inner: Div,
    start_color: Option<Hsla>,
    end_color: Option<Hsla>,
    border_width: Pixels,
    angle: f32,
    corner_radius: Option<Pixels>,
}

impl GradientBorder {
    pub fn new() -> Self {
        Self {
            base: div(),
            inner: div(),
            start_color: None,
            end_color: None,
            border_width: px(2.0),
            angle: 135.0,
            corner_radius: None,
        }
    }

    pub fn colors(mut self, start: Hsla, end: Hsla) -> Self {
        self.start_color = Some(start);
        self.end_color = Some(end);
        self
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.border_width = width;
        self
    }

    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub fn rounded(mut self, radius: Pixels) -> Self {
        self.corner_radius = Some(radius);
        self
    }
}

impl Default for GradientBorder {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for GradientBorder {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let start = self.start_color.unwrap_or(theme.tokens.primary);
        let end = self.end_color.unwrap_or(theme.tokens.accent);
        let radius = self.corner_radius.unwrap_or(theme.tokens.radius_lg);
        let border_w = self.border_width;

        self.base
            .bg(gpui::linear_gradient(
                self.angle,
                gpui::linear_color_stop(start, 0.0),
                gpui::linear_color_stop(end, 1.0),
            ))
            .rounded(radius)
            .p(border_w)
            .child(
                self.inner
                    .flex_grow()
                    .bg(theme.tokens.card)
                    .rounded(radius - border_w),
            )
    }
}

impl Styled for GradientBorder {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for GradientBorder {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for GradientBorder {}

impl ParentElement for GradientBorder {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements)
    }
}
