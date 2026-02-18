use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};

const CHART_COLORS: [u32; 8] = [
    0x3b82f6, 0x22c55e, 0xf59e0b, 0xef4444, 0x8b5cf6, 0x06b6d4, 0xf97316, 0xec4899,
];

fn default_color(index: usize) -> Hsla {
    rgb(CHART_COLORS[index % CHART_COLORS.len()]).into()
}

#[derive(Clone)]
pub struct RadarDataset {
    pub label: SharedString,
    pub values: Vec<f64>,
    pub color: Option<Hsla>,
}

impl RadarDataset {
    pub fn new(label: impl Into<SharedString>, values: Vec<f64>) -> Self {
        Self {
            label: label.into(),
            values,
            color: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum RadarChartSize {
    Sm,
    #[default]
    Md,
    Lg,
    Custom(u32),
}

impl RadarChartSize {
    fn to_pixels(self) -> Pixels {
        match self {
            RadarChartSize::Sm => px(200.0),
            RadarChartSize::Md => px(300.0),
            RadarChartSize::Lg => px(400.0),
            RadarChartSize::Custom(s) => px(s as f32),
        }
    }
}

struct PaintData {
    axes: Vec<SharedString>,
    datasets: Vec<RadarDataset>,
    show_grid: bool,
    grid_levels: usize,
    fill_opacity: f32,
    grid_color: Hsla,
    _text_color: Hsla,
    label_padding: f32,
}

#[derive(IntoElement)]
pub struct RadarChart {
    axes: Vec<SharedString>,
    datasets: Vec<RadarDataset>,
    size: RadarChartSize,
    show_grid: bool,
    show_legend: bool,
    grid_levels: usize,
    fill_opacity: f32,
    style: StyleRefinement,
}

impl RadarChart {
    pub fn new() -> Self {
        Self {
            axes: Vec::new(),
            datasets: Vec::new(),
            size: RadarChartSize::default(),
            show_grid: true,
            show_legend: true,
            grid_levels: 5,
            fill_opacity: 0.2,
            style: StyleRefinement::default(),
        }
    }

    pub fn axes(mut self, axes: Vec<impl Into<SharedString>>) -> Self {
        self.axes = axes.into_iter().map(|a| a.into()).collect();
        self
    }

    pub fn dataset(mut self, dataset: RadarDataset) -> Self {
        self.datasets.push(dataset);
        self
    }

    pub fn datasets(mut self, datasets: Vec<RadarDataset>) -> Self {
        self.datasets = datasets;
        self
    }

    pub fn size(mut self, size: RadarChartSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn grid_levels(mut self, levels: usize) -> Self {
        self.grid_levels = levels.max(2);
        self
    }

    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Styled for RadarChart {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn angle_for_axis(index: usize, total: usize) -> f32 {
    -std::f32::consts::FRAC_PI_2 + (index as f32 / total as f32) * std::f32::consts::TAU
}

impl RenderOnce for RadarChart {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let chart_size = self.size.to_pixels();
        let show_legend = self.show_legend && self.datasets.len() > 1;
        let datasets_for_legend = self.datasets.clone();
        let text_color = theme.tokens.muted_foreground;
        let label_padding: f32 = 30.0;

        let n_axes = self.axes.len();

        if n_axes < 3 {
            return div()
                .size(chart_size)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.tokens.muted_foreground)
                        .child("Need at least 3 axes"),
                )
                .map(|this| {
                    let mut d = this;
                    d.style().refine(&user_style);
                    d
                })
                .into_any_element();
        }

        let paint_data = PaintData {
            axes: self.axes.clone(),
            datasets: self.datasets,
            show_grid: self.show_grid,
            grid_levels: self.grid_levels,
            fill_opacity: self.fill_opacity,
            grid_color: theme.tokens.border,
            _text_color: text_color,
            label_padding,
        };

        let axis_labels = self.axes.clone();

        div()
            .flex()
            .flex_col()
            .items_center()
            .map(|this| {
                let mut d = this;
                d.style().refine(&user_style);
                d
            })
            .child(
                div()
                    .size(chart_size)
                    .relative()
                    .child(
                        canvas(
                            move |_bounds, _window, _cx| paint_data,
                            move |bounds, data, window, _cx| {
                                if bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
                                    return;
                                }

                                let n = data.axes.len();
                                if n < 3 {
                                    return;
                                }

                                let cx_f = bounds.left() + bounds.size.width * 0.5;
                                let cy_f = bounds.top() + bounds.size.height * 0.5;
                                let max_radius = (bounds.size.width.min(bounds.size.height) * 0.5)
                                    - px(data.label_padding);

                                if max_radius <= px(0.0) {
                                    return;
                                }

                                if data.show_grid {
                                    for level in 1..=data.grid_levels {
                                        let radius =
                                            max_radius * (level as f32 / data.grid_levels as f32);
                                        let mut builder = PathBuilder::stroke(px(1.0));
                                        for i in 0..=n {
                                            let angle = angle_for_axis(i % n, n);
                                            let pt = point(
                                                cx_f + radius * angle.cos(),
                                                cy_f + radius * angle.sin(),
                                            );
                                            if i == 0 {
                                                builder.move_to(pt);
                                            } else {
                                                builder.line_to(pt);
                                            }
                                        }
                                        if let Ok(path) = builder.build() {
                                            window.paint_path(path, data.grid_color.opacity(0.2));
                                        }
                                    }

                                    for i in 0..n {
                                        let angle = angle_for_axis(i, n);
                                        let mut builder = PathBuilder::stroke(px(1.0));
                                        builder.move_to(point(cx_f, cy_f));
                                        builder.line_to(point(
                                            cx_f + max_radius * angle.cos(),
                                            cy_f + max_radius * angle.sin(),
                                        ));
                                        if let Ok(path) = builder.build() {
                                            window.paint_path(path, data.grid_color.opacity(0.3));
                                        }
                                    }
                                }

                                for (ds_idx, ds) in data.datasets.iter().enumerate() {
                                    if ds.values.is_empty() {
                                        continue;
                                    }
                                    let color = ds.color.unwrap_or_else(|| default_color(ds_idx));

                                    let pts: Vec<Point<Pixels>> = (0..n)
                                        .map(|i| {
                                            let val = ds
                                                .values
                                                .get(i)
                                                .copied()
                                                .unwrap_or(0.0)
                                                .clamp(0.0, 1.0);
                                            let angle = angle_for_axis(i, n);
                                            let radius = max_radius * val as f32;
                                            point(
                                                cx_f + radius * angle.cos(),
                                                cy_f + radius * angle.sin(),
                                            )
                                        })
                                        .collect();

                                    if pts.len() >= 3 {
                                        let mut fill_builder = PathBuilder::fill();
                                        fill_builder.move_to(pts[0]);
                                        for pt in pts.iter().skip(1) {
                                            fill_builder.line_to(*pt);
                                        }
                                        fill_builder.close();
                                        if let Ok(path) = fill_builder.build() {
                                            window
                                                .paint_path(path, color.opacity(data.fill_opacity));
                                        }

                                        let mut stroke_builder = PathBuilder::stroke(px(2.0));
                                        stroke_builder.move_to(pts[0]);
                                        for pt in pts.iter().skip(1) {
                                            stroke_builder.line_to(*pt);
                                        }
                                        stroke_builder.line_to(pts[0]);
                                        if let Ok(path) = stroke_builder.build() {
                                            window.paint_path(path, color);
                                        }

                                        let dot_size = px(6.0);
                                        for pt in &pts {
                                            window.paint_quad(fill(
                                                Bounds::centered_at(*pt, size(dot_size, dot_size)),
                                                color,
                                            ));
                                        }
                                    }
                                }
                            },
                        )
                        .size_full(),
                    )
                    .children(axis_labels.iter().enumerate().map(|(i, label)| {
                        let angle = angle_for_axis(i, n_axes);
                        let label_dist = 0.5 + label_padding / (chart_size / px(1.0));
                        let left_frac = 0.5 + label_dist * angle.cos();
                        let top_frac = 0.5 + label_dist * angle.sin();
                        div()
                            .absolute()
                            .left(relative(left_frac))
                            .top(relative(top_frac))
                            .ml(px(-20.0))
                            .mt(px(-8.0))
                            .text_size(px(11.0))
                            .text_color(text_color)
                            .child(label.clone())
                    })),
            )
            .when(show_legend, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(16.0))
                        .py(px(8.0))
                        .justify_center()
                        .children(datasets_for_legend.iter().enumerate().map(|(i, ds)| {
                            let color = ds.color.unwrap_or_else(|| default_color(i));
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(div().size(px(12.0)).rounded(px(2.0)).bg(color))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_color)
                                        .child(ds.label.clone()),
                                )
                        })),
                )
            })
            .into_any_element()
    }
}
