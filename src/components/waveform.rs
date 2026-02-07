//! Audio waveform visualization using vertical bars.

use gpui::{prelude::FluentBuilder as _, *};

struct WaveformPaintData {
    data: Vec<f32>,
    bar_width: f32,
    gap: f32,
    color: Hsla,
    active_color: Hsla,
    playback_position: f32,
}

#[derive(IntoElement)]
pub struct Waveform {
    data: Vec<f32>,
    bar_width: Pixels,
    gap: Pixels,
    color: Option<Hsla>,
    active_color: Option<Hsla>,
    playback_position: f32,
    style: StyleRefinement,
}

impl Waveform {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            bar_width: px(3.0),
            gap: px(2.0),
            color: None,
            active_color: None,
            playback_position: 0.0,
            style: StyleRefinement::default(),
        }
    }

    pub fn data(mut self, data: &[f32]) -> Self {
        self.data = data.to_vec();
        self
    }

    pub fn bar_width(mut self, width: Pixels) -> Self {
        self.bar_width = width;
        self
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn active_color(mut self, color: Hsla) -> Self {
        self.active_color = Some(color);
        self
    }

    pub fn playback_position(mut self, position: f32) -> Self {
        self.playback_position = position.clamp(0.0, 1.0);
        self
    }
}

impl Default for Waveform {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Waveform {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Waveform {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = crate::theme::use_theme();
        let user_style = self.style;

        let default_color = theme.tokens.muted_foreground.opacity(0.4);
        let default_active = theme.tokens.primary;

        let paint_data = WaveformPaintData {
            data: self.data,
            bar_width: self.bar_width / px(1.0),
            gap: self.gap / px(1.0),
            color: self.color.unwrap_or(default_color),
            active_color: self.active_color.unwrap_or(default_active),
            playback_position: self.playback_position,
        };

        div()
            .relative()
            .when(user_style.size.width.is_none(), |this| this.w_full())
            .when(user_style.size.height.is_none(), |this| this.h(px(48.0)))
            .child(
                canvas(
                    move |_bounds, _window, _cx| paint_data,
                    move |bounds, data, window, _cx| {
                        paint_waveform(bounds, &data, window);
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}

fn paint_waveform(bounds: Bounds<Pixels>, data: &WaveformPaintData, window: &mut Window) {
    if data.data.is_empty() || bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
        return;
    }

    let bar_w = data.bar_width;
    let gap_w = data.gap;
    let step = bar_w + gap_w;

    if step <= 0.0 {
        return;
    }

    let available_width = bounds.size.width / px(1.0);
    let max_bars = (available_width / step).floor() as usize;

    if max_bars == 0 {
        return;
    }

    let bar_count = max_bars.min(data.data.len());
    let active_bar_boundary = (data.playback_position * bar_count as f32).floor() as usize;
    let height_f = bounds.size.height / px(1.0);

    for i in 0..bar_count {
        let sample_idx = if bar_count < data.data.len() {
            (i as f32 / bar_count as f32 * data.data.len() as f32) as usize
        } else {
            i
        };

        let amplitude = data
            .data
            .get(sample_idx)
            .copied()
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        let bar_height = (amplitude * height_f).max(2.0);

        let x = bounds.left() + px(i as f32 * step);
        let y = bounds.top() + px((height_f - bar_height) * 0.5);

        let bar_color = if i < active_bar_boundary {
            data.active_color
        } else {
            data.color
        };

        window.paint_quad(PaintQuad {
            bounds: Bounds {
                origin: point(x, y),
                size: gpui::size(px(bar_w), px(bar_height)),
            },
            corner_radii: Corners::all(px(bar_w * 0.5)),
            background: bar_color.into(),
            border_widths: Edges::default(),
            border_color: transparent_black(),
            border_style: BorderStyle::default(),
            continuous_corners: false,
            transform: Default::default(),
            blend_mode: Default::default(),
        });
    }
}
