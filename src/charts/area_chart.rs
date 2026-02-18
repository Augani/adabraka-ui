use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};

const CHART_COLORS: [u32; 8] = [
    0x3b82f6, 0x22c55e, 0xf59e0b, 0xef4444, 0x8b5cf6, 0x06b6d4, 0xf97316, 0xec4899,
];

fn default_color(index: usize) -> Hsla {
    rgb(CHART_COLORS[index % CHART_COLORS.len()]).into()
}

#[derive(Clone, Debug)]
pub struct AreaChartSeries {
    pub label: SharedString,
    pub points: Vec<(f64, f64)>,
    pub color: Option<Hsla>,
}

impl AreaChartSeries {
    pub fn new(label: impl Into<SharedString>, points: Vec<(f64, f64)>) -> Self {
        Self {
            label: label.into(),
            points,
            color: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum AreaChartSize {
    Sm,
    #[default]
    Md,
    Lg,
    Custom(u32, u32),
}

impl AreaChartSize {
    fn dimensions(self) -> (Pixels, Pixels) {
        match self {
            AreaChartSize::Sm => (px(300.0), px(180.0)),
            AreaChartSize::Md => (px(500.0), px(280.0)),
            AreaChartSize::Lg => (px(700.0), px(400.0)),
            AreaChartSize::Custom(w, h) => (px(w as f32), px(h as f32)),
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum AreaChartMode {
    #[default]
    Overlaid,
    Stacked,
}

struct AreaChartRange {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl AreaChartRange {
    fn from_series(series: &[AreaChartSeries], mode: AreaChartMode) -> Self {
        if series.is_empty() {
            return Self {
                x_min: 0.0,
                x_max: 1.0,
                y_min: 0.0,
                y_max: 1.0,
            };
        }

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let y_min = 0.0_f64;
        let mut y_max = f64::MIN;

        for s in series {
            for &(x, y) in &s.points {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                if mode == AreaChartMode::Overlaid {
                    y_max = y_max.max(y);
                }
            }
        }

        if mode == AreaChartMode::Stacked && !series.is_empty() {
            let max_len = series.iter().map(|s| s.points.len()).max().unwrap_or(0);
            for i in 0..max_len {
                let stacked: f64 = series
                    .iter()
                    .map(|s| s.points.get(i).map(|p| p.1).unwrap_or(0.0))
                    .sum();
                y_max = y_max.max(stacked);
            }
        }

        if x_min == f64::MAX {
            x_min = 0.0;
            x_max = 1.0;
        }
        if y_max == f64::MIN {
            y_max = 1.0;
        }
        if (x_max - x_min).abs() < f64::EPSILON {
            x_max = x_min + 1.0;
        }
        if (y_max - y_min).abs() < f64::EPSILON {
            y_max = y_min + 1.0;
        }

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn normalize_x(&self, x: f64) -> f32 {
        ((x - self.x_min) / (self.x_max - self.x_min)) as f32
    }

    fn normalize_y(&self, y: f64) -> f32 {
        ((y - self.y_min) / (self.y_max - self.y_min)) as f32
    }
}

struct PaintData {
    series: Vec<AreaChartSeries>,
    show_grid: bool,
    _show_x_axis: bool,
    _show_y_axis: bool,
    mode: AreaChartMode,
    _x_labels: Vec<SharedString>,
    y_label_count: usize,
    grid_color: Hsla,
    _text_color: Hsla,
    fill_opacity: f32,
}

#[derive(IntoElement)]
pub struct AreaChart {
    series: Vec<AreaChartSeries>,
    size: AreaChartSize,
    mode: AreaChartMode,
    show_grid: bool,
    show_x_axis: bool,
    show_y_axis: bool,
    show_legend: bool,
    x_labels: Vec<SharedString>,
    y_label_count: usize,
    fill_opacity: f32,
    style: StyleRefinement,
}

impl AreaChart {
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            size: AreaChartSize::default(),
            mode: AreaChartMode::default(),
            show_grid: true,
            show_x_axis: true,
            show_y_axis: true,
            show_legend: true,
            x_labels: Vec::new(),
            y_label_count: 5,
            fill_opacity: 0.25,
            style: StyleRefinement::default(),
        }
    }

    pub fn series(mut self, s: AreaChartSeries) -> Self {
        self.series.push(s);
        self
    }

    pub fn add_series(mut self, s: AreaChartSeries) -> Self {
        self.series.push(s);
        self
    }

    pub fn size(mut self, size: AreaChartSize) -> Self {
        self.size = size;
        self
    }

    pub fn mode(mut self, mode: AreaChartMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn stacked(mut self) -> Self {
        self.mode = AreaChartMode::Stacked;
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    pub fn show_x_axis(mut self, show: bool) -> Self {
        self.show_x_axis = show;
        self
    }

    pub fn show_y_axis(mut self, show: bool) -> Self {
        self.show_y_axis = show;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn x_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.x_labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn y_label_count(mut self, count: usize) -> Self {
        self.y_label_count = count.max(2);
        self
    }

    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Styled for AreaChart {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn format_value(value: f64) -> String {
    if value.abs() >= 1000.0 {
        format!("{:.0}k", value / 1000.0)
    } else if value.abs() >= 1.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

impl RenderOnce for AreaChart {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let (chart_w, chart_h) = self.size.dimensions();

        let padding_left: f32 = if self.show_y_axis { 50.0 } else { 10.0 };
        let padding_right: f32 = 20.0;
        let padding_top: f32 = 20.0;
        let padding_bottom: f32 = if self.show_x_axis { 40.0 } else { 10.0 };

        let series_for_legend = self.series.clone();
        let show_legend = self.show_legend && self.series.len() > 1;

        let range = AreaChartRange::from_series(&self.series, self.mode);
        let y_labels: Vec<String> = if self.show_y_axis {
            (0..=self.y_label_count)
                .map(|i| {
                    let normalized = i as f64 / self.y_label_count as f64;
                    let value = range.y_min + (range.y_max - range.y_min) * (1.0 - normalized);
                    format_value(value)
                })
                .collect()
        } else {
            Vec::new()
        };

        let text_color = theme.tokens.muted_foreground;

        let paint_data = PaintData {
            series: self.series,
            show_grid: self.show_grid,
            _show_x_axis: self.show_x_axis,
            _show_y_axis: self.show_y_axis,
            mode: self.mode,
            _x_labels: self.x_labels.clone(),
            y_label_count: self.y_label_count,
            grid_color: theme.tokens.border,
            _text_color: text_color,
            fill_opacity: self.fill_opacity,
        };

        div()
            .flex()
            .flex_col()
            .w(chart_w)
            .map(|this| {
                let mut d = this;
                d.style().refine(&user_style);
                d
            })
            .child(
                div()
                    .h(chart_h)
                    .w_full()
                    .relative()
                    .child(
                        canvas(
                            move |_bounds, _window, _cx| paint_data,
                            move |bounds, data, window, _cx| {
                                if bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
                                    return;
                                }

                                let range = AreaChartRange::from_series(&data.series, data.mode);

                                let chart_left = bounds.left() + px(padding_left);
                                let chart_right = bounds.right() - px(padding_right);
                                let chart_top = bounds.top() + px(padding_top);
                                let chart_bottom = bounds.bottom() - px(padding_bottom);
                                let chart_width = chart_right - chart_left;
                                let chart_height = chart_bottom - chart_top;

                                if chart_width <= px(0.0) || chart_height <= px(0.0) {
                                    return;
                                }

                                if data.show_grid {
                                    let grid_lines = data.y_label_count;
                                    for i in 0..=grid_lines {
                                        let y = chart_top
                                            + chart_height * (i as f32 / grid_lines as f32);
                                        let mut builder = PathBuilder::stroke(px(1.0));
                                        builder.move_to(point(chart_left, y));
                                        builder.line_to(point(chart_right, y));
                                        if let Ok(path) = builder.build() {
                                            window.paint_path(path, data.grid_color.opacity(0.2));
                                        }
                                    }

                                    for i in 0..=grid_lines {
                                        let x = chart_left
                                            + chart_width * (i as f32 / grid_lines as f32);
                                        let mut builder = PathBuilder::stroke(px(1.0));
                                        builder.move_to(point(x, chart_top));
                                        builder.line_to(point(x, chart_bottom));
                                        if let Ok(path) = builder.build() {
                                            window.paint_path(path, data.grid_color.opacity(0.2));
                                        }
                                    }
                                }

                                match data.mode {
                                    AreaChartMode::Overlaid => {
                                        for (idx, s) in data.series.iter().enumerate().rev() {
                                            if s.points.len() < 2 {
                                                continue;
                                            }
                                            let color =
                                                s.color.unwrap_or_else(|| default_color(idx));

                                            let screen_pts: Vec<Point<Pixels>> = s
                                                .points
                                                .iter()
                                                .map(|&(x, y)| {
                                                    let sx = chart_left
                                                        + chart_width * range.normalize_x(x);
                                                    let sy = chart_bottom
                                                        - chart_height * range.normalize_y(y);
                                                    point(sx, sy)
                                                })
                                                .collect();

                                            let mut fill_builder = PathBuilder::fill();
                                            fill_builder
                                                .move_to(point(screen_pts[0].x, chart_bottom));
                                            fill_builder.line_to(screen_pts[0]);
                                            for pt in screen_pts.iter().skip(1) {
                                                fill_builder.line_to(*pt);
                                            }
                                            fill_builder.line_to(point(
                                                screen_pts.last().unwrap().x,
                                                chart_bottom,
                                            ));
                                            fill_builder.close();
                                            if let Ok(path) = fill_builder.build() {
                                                window.paint_path(
                                                    path,
                                                    color.opacity(data.fill_opacity),
                                                );
                                            }

                                            let mut stroke_builder = PathBuilder::stroke(px(2.0));
                                            stroke_builder.move_to(screen_pts[0]);
                                            for pt in screen_pts.iter().skip(1) {
                                                stroke_builder.line_to(*pt);
                                            }
                                            if let Ok(path) = stroke_builder.build() {
                                                window.paint_path(path, color);
                                            }
                                        }
                                    }
                                    AreaChartMode::Stacked => {
                                        let max_len = data
                                            .series
                                            .iter()
                                            .map(|s| s.points.len())
                                            .max()
                                            .unwrap_or(0);
                                        if max_len < 2 {
                                            return;
                                        }

                                        let mut cumulative = vec![0.0_f64; max_len];
                                        let mut baselines: Vec<Vec<Point<Pixels>>> = Vec::new();

                                        let x_values: Vec<f64> = (0..max_len)
                                            .map(|i| {
                                                data.series
                                                    .iter()
                                                    .find_map(|s| s.points.get(i).map(|p| p.0))
                                                    .unwrap_or(i as f64)
                                            })
                                            .collect();

                                        let base_pts: Vec<Point<Pixels>> = x_values
                                            .iter()
                                            .map(|&x| {
                                                let sx =
                                                    chart_left + chart_width * range.normalize_x(x);
                                                point(sx, chart_bottom)
                                            })
                                            .collect();
                                        baselines.push(base_pts);

                                        for (idx, s) in data.series.iter().enumerate() {
                                            let color =
                                                s.color.unwrap_or_else(|| default_color(idx));

                                            for (i, pt) in s.points.iter().enumerate() {
                                                if i < max_len {
                                                    cumulative[i] += pt.1;
                                                }
                                            }

                                            let top_pts: Vec<Point<Pixels>> = (0..max_len)
                                                .map(|i| {
                                                    let sx = chart_left
                                                        + chart_width
                                                            * range.normalize_x(x_values[i]);
                                                    let sy = chart_bottom
                                                        - chart_height
                                                            * range.normalize_y(cumulative[i]);
                                                    point(sx, sy)
                                                })
                                                .collect();

                                            let bottom_pts = baselines.last().unwrap();

                                            let mut fill_builder = PathBuilder::fill();
                                            fill_builder.move_to(bottom_pts[0]);
                                            for pt in top_pts.iter() {
                                                fill_builder.line_to(*pt);
                                            }
                                            for pt in bottom_pts.iter().rev() {
                                                fill_builder.line_to(*pt);
                                            }
                                            fill_builder.close();
                                            if let Ok(path) = fill_builder.build() {
                                                window.paint_path(
                                                    path,
                                                    color.opacity(data.fill_opacity),
                                                );
                                            }

                                            let mut stroke_builder = PathBuilder::stroke(px(2.0));
                                            stroke_builder.move_to(top_pts[0]);
                                            for pt in top_pts.iter().skip(1) {
                                                stroke_builder.line_to(*pt);
                                            }
                                            if let Ok(path) = stroke_builder.build() {
                                                window.paint_path(path, color);
                                            }

                                            baselines.push(top_pts);
                                        }
                                    }
                                }
                            },
                        )
                        .size_full(),
                    )
                    .when(self.show_y_axis, |this| {
                        this.children(y_labels.iter().enumerate().map(|(i, label)| {
                            let tick_count = y_labels.len().saturating_sub(1).max(1);
                            let top_percent = i as f32 / tick_count as f32;
                            div()
                                .absolute()
                                .left(px(4.0))
                                .top(relative(top_percent))
                                .mt(px(padding_top - 6.0))
                                .text_size(px(11.0))
                                .text_color(text_color)
                                .child(label.clone())
                        }))
                    })
                    .when(self.show_x_axis && !self.x_labels.is_empty(), |this| {
                        let num_labels = self.x_labels.len();
                        this.children(self.x_labels.iter().enumerate().map(|(i, label)| {
                            let left_percent = if num_labels == 1 {
                                0.5
                            } else {
                                i as f32 / (num_labels - 1) as f32
                            };
                            let chart_w_f32 = chart_w / px(1.0);
                            let chart_frac = 1.0 - (padding_left + padding_right) / chart_w_f32;
                            let adjusted_left =
                                padding_left / chart_w_f32 + left_percent * chart_frac;
                            div()
                                .absolute()
                                .bottom(px(8.0))
                                .left(relative(adjusted_left))
                                .ml(px(-15.0))
                                .text_size(px(11.0))
                                .text_color(text_color)
                                .child(label.clone())
                        }))
                    }),
            )
            .when(show_legend, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(16.0))
                        .px(px(padding_left))
                        .py(px(8.0))
                        .children(series_for_legend.iter().enumerate().map(|(i, s)| {
                            let color = s.color.unwrap_or_else(|| default_color(i));
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(div().size(px(12.0)).rounded(px(2.0)).bg(color))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_color)
                                        .child(s.label.clone()),
                                )
                        })),
                )
            })
    }
}
