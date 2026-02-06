//! Aurora background - animated overlapping blobs creating organic color-shifting effect.

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::time::Duration;

use crate::theme::use_theme;

#[derive(IntoElement)]
pub struct Aurora {
    colors: Vec<Hsla>,
    speed: f32,
    blur_amount: Pixels,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Aurora {
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            speed: 1.0,
            blur_amount: px(80.0),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn colors(mut self, colors: Vec<Hsla>) -> Self {
        self.colors = colors;
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    pub fn blur_amount(mut self, blur: Pixels) -> Self {
        self.blur_amount = blur;
        self
    }
}

impl Default for Aurora {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Aurora {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Aurora {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

struct BlobConfig {
    x_pct: f32,
    y_pct: f32,
    w_pct: f32,
    h_pct: f32,
    duration_ms: u64,
    x_drift: f32,
    y_drift: f32,
}

const BLOB_CONFIGS: [BlobConfig; 5] = [
    BlobConfig {
        x_pct: -0.1,
        y_pct: -0.2,
        w_pct: 0.7,
        h_pct: 0.6,
        duration_ms: 8000,
        x_drift: 0.15,
        y_drift: 0.1,
    },
    BlobConfig {
        x_pct: 0.3,
        y_pct: 0.1,
        w_pct: 0.6,
        h_pct: 0.7,
        duration_ms: 10000,
        x_drift: -0.1,
        y_drift: 0.15,
    },
    BlobConfig {
        x_pct: 0.5,
        y_pct: -0.1,
        w_pct: 0.5,
        h_pct: 0.5,
        duration_ms: 12000,
        x_drift: -0.12,
        y_drift: -0.1,
    },
    BlobConfig {
        x_pct: -0.05,
        y_pct: 0.4,
        w_pct: 0.65,
        h_pct: 0.55,
        duration_ms: 9000,
        x_drift: 0.2,
        y_drift: -0.08,
    },
    BlobConfig {
        x_pct: 0.4,
        y_pct: 0.3,
        w_pct: 0.55,
        h_pct: 0.65,
        duration_ms: 11000,
        x_drift: -0.08,
        y_drift: 0.12,
    },
];

impl RenderOnce for Aurora {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let default_colors = vec![
            Hsla {
                a: 0.3,
                ..theme.tokens.primary
            },
            Hsla {
                a: 0.25,
                ..theme.tokens.accent
            },
            hsla(280.0 / 360.0, 0.7, 0.6, 0.2),
            hsla(200.0 / 360.0, 0.8, 0.5, 0.25),
            hsla(160.0 / 360.0, 0.6, 0.5, 0.2),
        ];

        let colors = if self.colors.is_empty() {
            default_colors
        } else {
            self.colors
        };

        let speed = self.speed;
        let blur = self.blur_amount;
        let blur_f32 = blur / px(1.0);

        let mut container = div().relative().w_full().h_full().overflow_hidden();

        for (idx, config) in BLOB_CONFIGS.iter().enumerate() {
            let color = colors[idx % colors.len()];
            let duration_ms = (config.duration_ms as f32 / speed) as u64;
            let duration = Duration::from_millis(duration_ms.max(500));

            let x_pct = config.x_pct;
            let y_pct = config.y_pct;
            let w_pct = config.w_pct;
            let h_pct = config.h_pct;
            let x_drift = config.x_drift;
            let y_drift = config.y_drift;

            container = container.child(
                div()
                    .absolute()
                    .left(relative(x_pct))
                    .top(relative(y_pct))
                    .w(relative(w_pct))
                    .h(relative(h_pct))
                    .rounded(px(blur_f32 * 2.0))
                    .bg(color)
                    .with_animation(
                        ElementId::Name(format!("aurora-blob-{idx}").into()),
                        Animation::new(duration)
                            .repeat()
                            .with_easing(gpui::ease_in_out),
                        move |el, delta| {
                            let phase = if delta < 0.5 {
                                delta * 2.0
                            } else {
                                2.0 - delta * 2.0
                            };
                            let offset_x = x_drift * phase;
                            let offset_y = y_drift * phase;
                            el.left(relative(x_pct + offset_x))
                                .top(relative(y_pct + offset_y))
                                .opacity(0.6 + 0.4 * phase)
                        },
                    ),
            );
        }

        container
            .child(div().relative().size_full().children(self.children))
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
