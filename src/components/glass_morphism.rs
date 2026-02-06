use gpui::prelude::FluentBuilder as _;
use gpui::*;

use crate::theme::use_theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlassIntensity {
    Light,
    #[default]
    Medium,
    Heavy,
}

#[derive(IntoElement)]
pub struct GlassMorphism {
    base: Div,
    opacity: f32,
    tint: Option<Hsla>,
    show_border: bool,
    intensity: GlassIntensity,
    noise: bool,
}

impl GlassMorphism {
    pub fn new() -> Self {
        Self {
            base: div(),
            opacity: 0.8,
            tint: None,
            show_border: true,
            intensity: GlassIntensity::default(),
            noise: false,
        }
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn tint(mut self, color: Hsla) -> Self {
        self.tint = Some(color);
        self
    }

    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    pub fn intensity(mut self, intensity: GlassIntensity) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn noise(mut self, noise: bool) -> Self {
        self.noise = noise;
        self
    }
}

impl RenderOnce for GlassMorphism {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let (bg_alpha, border_alpha) = match self.intensity {
            GlassIntensity::Light => (0.05, 0.08),
            GlassIntensity::Medium => (0.10, 0.15),
            GlassIntensity::Heavy => (0.20, 0.25),
        };

        let bg_alpha = bg_alpha * self.opacity;
        let border_alpha = border_alpha * self.opacity;

        let bg_color = if let Some(tint) = self.tint {
            hsla(tint.h, tint.s, tint.l, bg_alpha)
        } else {
            hsla(0.0, 0.0, 1.0, bg_alpha)
        };

        let border_color = hsla(0.0, 0.0, 1.0, border_alpha);
        let highlight_color = hsla(0.0, 0.0, 1.0, border_alpha * 0.5);

        let shadow = BoxShadow {
            offset: point(px(0.0), px(4.0)),
            blur_radius: px(12.0),
            spread_radius: px(0.0),
            inset: false,
            color: hsla(0.0, 0.0, 0.0, 0.08 * self.opacity),
        };

        let mut element = self
            .base
            .bg(bg_color)
            .rounded(theme.tokens.radius_lg)
            .shadow(smallvec::smallvec![shadow]);

        if self.show_border {
            element = element
                .border_1()
                .border_color(border_color)
                .border_t_1()
                .border_color(highlight_color);
        }

        let noise = self.noise;
        let noise_opacity = self.opacity;
        element.relative().when(noise, |el| {
            let dot_color = hsla(0.0, 0.0, 1.0, 0.03 * noise_opacity);
            let mut noise_layer = div().absolute().inset_0().overflow_hidden();
            let positions: [(f32, f32); 24] = [
                (0.07, 0.13),
                (0.23, 0.05),
                (0.41, 0.19),
                (0.59, 0.08),
                (0.77, 0.22),
                (0.91, 0.11),
                (0.15, 0.37),
                (0.33, 0.44),
                (0.52, 0.31),
                (0.68, 0.47),
                (0.84, 0.39),
                (0.03, 0.55),
                (0.19, 0.63),
                (0.45, 0.57),
                (0.61, 0.69),
                (0.79, 0.61),
                (0.93, 0.73),
                (0.11, 0.81),
                (0.29, 0.87),
                (0.47, 0.79),
                (0.65, 0.91),
                (0.83, 0.85),
                (0.37, 0.95),
                (0.71, 0.03),
            ];
            for (px_pct, py_pct) in positions {
                noise_layer = noise_layer.child(
                    div()
                        .absolute()
                        .left(relative(px_pct))
                        .top(relative(py_pct))
                        .w(px(1.0))
                        .h(px(1.0))
                        .rounded_full()
                        .bg(dot_color),
                );
            }
            el.child(noise_layer)
        })
    }
}

impl Default for GlassMorphism {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for GlassMorphism {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for GlassMorphism {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for GlassMorphism {}

impl ParentElement for GlassMorphism {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements)
    }
}
