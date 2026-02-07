//! Noise texture overlay - procedural visual noise using scattered micro-quads.

use gpui::prelude::FluentBuilder as _;
use gpui::*;

use crate::theme::use_theme;

#[derive(IntoElement)]
pub struct Noise {
    density: f32,
    noise_opacity: f32,
    grain_size: Pixels,
    color: Option<Hsla>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            density: 0.3,
            noise_opacity: 0.08,
            grain_size: px(1.0),
            color: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn density(mut self, density: f32) -> Self {
        self.density = density.clamp(0.01, 1.0);
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.noise_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn grain_size(mut self, size: Pixels) -> Self {
        self.grain_size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Noise {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Noise {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

fn hash_position(seed: u32) -> f32 {
    let mut h = seed;
    h ^= h >> 16;
    h = h.wrapping_mul(0x45d9f3b);
    h ^= h >> 16;
    h = h.wrapping_mul(0x45d9f3b);
    h ^= h >> 16;
    (h & 0xFFFF) as f32 / 65535.0
}

impl RenderOnce for Noise {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let density = self.density;
        let opacity = self.noise_opacity;
        let grain_size = self.grain_size;

        let base_color = self.color.unwrap_or(theme.tokens.foreground);
        let grain_color = Hsla {
            a: opacity,
            ..base_color
        };

        div()
            .relative()
            .w_full()
            .h_full()
            .overflow_hidden()
            .child(
                canvas(
                    move |_bounds, _window, _cx| {},
                    move |bounds, _, window, _cx| {
                        let origin_x = bounds.origin.x / px(1.0);
                        let origin_y = bounds.origin.y / px(1.0);
                        let width = bounds.size.width / px(1.0);
                        let height = bounds.size.height / px(1.0);
                        let grain_f32 = grain_size / px(1.0);

                        let step = grain_f32 / density;
                        let cols = (width / step).ceil() as u32;
                        let rows = (height / step).ceil() as u32;

                        for row in 0..rows {
                            for col in 0..cols {
                                let seed =
                                    row.wrapping_mul(65537).wrapping_add(col).wrapping_add(42);
                                let rand_x = hash_position(seed);
                                let rand_y = hash_position(seed.wrapping_add(7919));
                                let rand_a = hash_position(seed.wrapping_add(15887));

                                let px_x = origin_x + col as f32 * step + rand_x * step;
                                let px_y = origin_y + row as f32 * step + rand_y * step;

                                if px_x >= origin_x + width || px_y >= origin_y + height {
                                    continue;
                                }

                                let alpha = grain_color.a * rand_a;
                                let color = Hsla {
                                    a: alpha,
                                    ..grain_color
                                };

                                window.paint_quad(PaintQuad {
                                    bounds: Bounds {
                                        origin: point(px(px_x), px(px_y)),
                                        size: gpui::size(grain_size, grain_size),
                                    },
                                    corner_radii: Corners::all(px(0.0)),
                                    background: color.into(),
                                    border_widths: Edges::default(),
                                    border_color: transparent_black(),
                                    border_style: BorderStyle::default(),
                                    continuous_corners: false,
                                    transform: Default::default(),
                                });
                            }
                        }
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
            .child(div().relative().size_full().children(self.children))
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
