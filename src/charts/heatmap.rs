use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};

fn lerp_color(low: Hsla, high: Hsla, t: f32) -> Hsla {
    let t = t.clamp(0.0, 1.0);
    hsla(
        low.h + (high.h - low.h) * t,
        low.s + (high.s - low.s) * t,
        low.l + (high.l - low.l) * t,
        low.a + (high.a - low.a) * t,
    )
}

#[derive(IntoElement)]
pub struct Heatmap {
    data: Vec<Vec<f64>>,
    x_labels: Vec<SharedString>,
    y_labels: Vec<SharedString>,
    low_color: Option<Hsla>,
    high_color: Option<Hsla>,
    cell_size: Pixels,
    gap: Pixels,
    show_values: bool,
    show_labels: bool,
    style: StyleRefinement,
}

impl Heatmap {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            x_labels: Vec::new(),
            y_labels: Vec::new(),
            low_color: None,
            high_color: None,
            cell_size: px(40.0),
            gap: px(2.0),
            show_values: false,
            show_labels: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn data(mut self, data: Vec<Vec<f64>>) -> Self {
        self.data = data;
        self
    }

    pub fn x_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.x_labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn y_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.y_labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn color_scale(mut self, low: Hsla, high: Hsla) -> Self {
        self.low_color = Some(low);
        self.high_color = Some(high);
        self
    }

    pub fn cell_size(mut self, size: Pixels) -> Self {
        self.cell_size = size;
        self
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap;
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }
}

impl Styled for Heatmap {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Heatmap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let low_color = self.low_color.unwrap_or_else(|| hsla(0.58, 0.7, 0.92, 1.0));
        let high_color = self
            .high_color
            .unwrap_or_else(|| hsla(0.58, 0.9, 0.35, 1.0));

        let mut global_min = f64::MAX;
        let mut global_max = f64::MIN;
        for row in &self.data {
            for &val in row {
                global_min = global_min.min(val);
                global_max = global_max.max(val);
            }
        }
        if global_min == f64::MAX {
            global_min = 0.0;
            global_max = 1.0;
        }
        if (global_max - global_min).abs() < f64::EPSILON {
            global_max = global_min + 1.0;
        }

        let value_range = global_max - global_min;
        let cell_size = self.cell_size;
        let gap = self.gap;
        let show_values = self.show_values;
        let show_labels = self.show_labels;
        let _text_color = theme.tokens.foreground;
        let label_color = theme.tokens.muted_foreground;

        let has_y_labels = show_labels && !self.y_labels.is_empty();
        let has_x_labels = show_labels && !self.x_labels.is_empty();

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .map(|this| {
                let mut d = this;
                d.style().refine(&user_style);
                d
            })
            .child(
                div()
                    .flex()
                    .gap(px(4.0))
                    .when(has_y_labels, |this| {
                        this.child(div().flex().flex_col().gap(gap).children(
                            self.y_labels.iter().map(|label| {
                                div()
                                    .h(cell_size)
                                    .flex()
                                    .items_center()
                                    .justify_end()
                                    .pr(px(4.0))
                                    .text_size(px(11.0))
                                    .text_color(label_color)
                                    .child(label.clone())
                            }),
                        ))
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(gap)
                            .children(self.data.iter().map(|row| {
                                div().flex().gap(gap).children(row.iter().map(|&val| {
                                    let t = ((val - global_min) / value_range) as f32;
                                    let bg = lerp_color(low_color, high_color, t);

                                    let contrast = if bg.l > 0.5 {
                                        hsla(0.0, 0.0, 0.1, 1.0)
                                    } else {
                                        hsla(0.0, 0.0, 0.95, 1.0)
                                    };

                                    div()
                                        .size(cell_size)
                                        .rounded(px(4.0))
                                        .bg(bg)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .when(show_values, |this| {
                                            this.child(
                                                div()
                                                    .text_size(px(10.0))
                                                    .text_color(contrast)
                                                    .child(format!("{:.0}", val)),
                                            )
                                        })
                                }))
                            })),
                    ),
            )
            .when(has_x_labels, |this| {
                this.child(
                    div()
                        .flex()
                        .gap(gap)
                        .when(has_y_labels, |this| this.pl(px(60.0)))
                        .children(self.x_labels.iter().map(|label| {
                            div()
                                .w(cell_size)
                                .text_size(px(11.0))
                                .text_color(label_color)
                                .text_center()
                                .child(label.clone())
                        })),
                )
            })
    }
}
