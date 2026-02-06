//! Squarified treemap chart for hierarchical data visualization.

use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};

const CHART_COLORS: [u32; 8] = [
    0x3b82f6, 0x22c55e, 0xf59e0b, 0xef4444, 0x8b5cf6, 0x06b6d4, 0xf97316, 0xec4899,
];

fn default_color(index: usize) -> Hsla {
    rgb(CHART_COLORS[index % CHART_COLORS.len()]).into()
}

fn pixels_to_f32(p: Pixels) -> f32 {
    p / px(1.0)
}

#[derive(Clone)]
pub struct TreeMapNode {
    pub label: SharedString,
    pub value: f64,
    pub color: Option<Hsla>,
    pub children: Vec<TreeMapNode>,
}

impl TreeMapNode {
    pub fn new(label: impl Into<SharedString>, value: f64) -> Self {
        Self {
            label: label.into(),
            value: value.max(0.0),
            children: Vec::new(),
            color: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn children(mut self, children: Vec<TreeMapNode>) -> Self {
        self.children = children;
        self
    }

    fn total_value(&self) -> f64 {
        if self.children.is_empty() {
            self.value
        } else {
            self.children.iter().map(|c| c.total_value()).sum()
        }
    }
}

#[derive(Clone)]
struct FlatRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Hsla,
    label: SharedString,
}

fn squarify_layout(
    nodes: &[TreeMapNode],
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color_scale: &[Hsla],
    depth_index: &mut usize,
    padding: f32,
    min_cell: f32,
    out: &mut Vec<FlatRect>,
) {
    if nodes.is_empty() || w <= 0.0 || h <= 0.0 {
        return;
    }

    let total: f64 = nodes.iter().map(|n| n.total_value()).sum();
    if total <= 0.0 {
        return;
    }

    let area = (w as f64) * (h as f64);
    let mut sorted: Vec<&TreeMapNode> = nodes.iter().collect();
    sorted.sort_by(|a, b| {
        b.total_value()
            .partial_cmp(&a.total_value())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut rects: Vec<(f32, f32, f32, f32, usize)> = Vec::new();
    layout_strip(&sorted, x, y, w, h, total, area, &mut rects);

    for (rx, ry, rw, rh, idx) in rects {
        let node = sorted[idx];
        let color = node.color.unwrap_or_else(|| {
            if !color_scale.is_empty() {
                color_scale[*depth_index % color_scale.len()]
            } else {
                default_color(*depth_index)
            }
        });
        *depth_index += 1;

        if rw < min_cell || rh < min_cell {
            continue;
        }

        if !node.children.is_empty() {
            let inner_x = rx + padding;
            let inner_y = ry + padding;
            let inner_w = (rw - 2.0 * padding).max(0.0);
            let inner_h = (rh - 2.0 * padding).max(0.0);
            squarify_layout(
                &node.children,
                inner_x,
                inner_y,
                inner_w,
                inner_h,
                color_scale,
                depth_index,
                padding,
                min_cell,
                out,
            );
        } else {
            out.push(FlatRect {
                x: rx,
                y: ry,
                w: rw,
                h: rh,
                color,
                label: node.label.clone(),
            });
        }
    }
}

fn layout_strip(
    nodes: &[&TreeMapNode],
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    total: f64,
    area: f64,
    out: &mut Vec<(f32, f32, f32, f32, usize)>,
) {
    if nodes.is_empty() {
        return;
    }

    if nodes.len() == 1 {
        out.push((x, y, w, h, 0));
        return;
    }

    let horizontal = w >= h;
    let mut row: Vec<usize> = Vec::new();
    let mut row_sum = 0.0f64;
    let mut best_worst = f64::MAX;
    let mut split_at = nodes.len();

    let short_side = if horizontal { h as f64 } else { w as f64 };

    for i in 0..nodes.len() {
        let val = nodes[i].total_value();
        let test_sum = row_sum + val;
        let test_count = row.len() + 1;

        let strip_area = test_sum / total * area;
        let strip_side = if short_side > 0.0 {
            strip_area / short_side
        } else {
            0.0
        };

        let mut worst = 0.0f64;
        for &ri in row.iter().chain(std::iter::once(&i)) {
            let cell_area = nodes[ri].total_value() / total * area;
            let cell_side = if strip_side > 0.0 {
                cell_area / strip_side
            } else {
                0.0
            };
            let ratio = if cell_side > 0.0 && strip_side > 0.0 {
                (strip_side / cell_side).max(cell_side / strip_side)
            } else {
                f64::MAX
            };
            worst = worst.max(ratio);
        }

        if test_count >= 2 && worst > best_worst {
            split_at = i;
            break;
        }

        best_worst = worst;
        row_sum = test_sum;
        row.push(i);
    }

    let row_fraction = row_sum / total;

    if horizontal {
        let row_w = (w as f64 * row_fraction) as f32;
        let mut cy = y;
        for &ri in &row {
            let frac = nodes[ri].total_value() / row_sum;
            let cell_h = (h as f64 * frac) as f32;
            out.push((x, cy, row_w, cell_h, ri));
            cy += cell_h;
        }

        if split_at < nodes.len() {
            let remaining: Vec<&TreeMapNode> = nodes[split_at..].iter().copied().collect();
            let mut sub_out = Vec::new();
            layout_strip(
                &remaining,
                x + row_w,
                y,
                w - row_w,
                h,
                total - row_sum,
                area * (1.0 - row_fraction),
                &mut sub_out,
            );
            for (sx, sy, sw, sh, si) in sub_out {
                out.push((sx, sy, sw, sh, si + split_at));
            }
        }
    } else {
        let row_h = (h as f64 * row_fraction) as f32;
        let mut cx = x;
        for &ri in &row {
            let frac = nodes[ri].total_value() / row_sum;
            let cell_w = (w as f64 * frac) as f32;
            out.push((cx, y, cell_w, row_h, ri));
            cx += cell_w;
        }

        if split_at < nodes.len() {
            let remaining: Vec<&TreeMapNode> = nodes[split_at..].iter().copied().collect();
            let mut sub_out = Vec::new();
            layout_strip(
                &remaining,
                x,
                y + row_h,
                w,
                h - row_h,
                total - row_sum,
                area * (1.0 - row_fraction),
                &mut sub_out,
            );
            for (sx, sy, sw, sh, si) in sub_out {
                out.push((sx, sy, sw, sh, si + split_at));
            }
        }
    }
}

#[derive(IntoElement)]
pub struct TreeMap {
    data: Vec<TreeMapNode>,
    color_scale: Vec<Hsla>,
    show_labels: bool,
    padding: Pixels,
    min_cell_size: Pixels,
    style: StyleRefinement,
}

impl TreeMap {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            color_scale: Vec::new(),
            show_labels: true,
            padding: px(2.0),
            min_cell_size: px(20.0),
            style: StyleRefinement::default(),
        }
    }

    pub fn data(mut self, data: Vec<TreeMapNode>) -> Self {
        self.data = data;
        self
    }

    pub fn color_scale(mut self, colors: Vec<Hsla>) -> Self {
        self.color_scale = colors;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn padding(mut self, padding: Pixels) -> Self {
        self.padding = padding;
        self
    }

    pub fn min_cell_size(mut self, size: Pixels) -> Self {
        self.min_cell_size = size;
        self
    }
}

impl Styled for TreeMap {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TreeMap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let data = self.data;
        let color_scale = self.color_scale;
        let show_labels = self.show_labels;
        let pad = pixels_to_f32(self.padding);
        let min_cell = pixels_to_f32(self.min_cell_size);
        let border_color = theme.tokens.background;

        div()
            .w_full()
            .h(px(300.0))
            .overflow_hidden()
            .bg(theme.tokens.background)
            .rounded(px(6.0))
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
            .child(
                canvas(
                    move |_bounds, _window, _cx| {},
                    move |bounds, _, window, cx| {
                        let bx = pixels_to_f32(bounds.origin.x);
                        let by = pixels_to_f32(bounds.origin.y);
                        let bw = pixels_to_f32(bounds.size.width);
                        let bh = pixels_to_f32(bounds.size.height);

                        let mut rects = Vec::new();
                        let mut depth_index = 0usize;
                        squarify_layout(
                            &data,
                            bx + pad,
                            by + pad,
                            bw - 2.0 * pad,
                            bh - 2.0 * pad,
                            &color_scale,
                            &mut depth_index,
                            pad,
                            min_cell,
                            &mut rects,
                        );

                        for rect in &rects {
                            window.paint_quad(PaintQuad {
                                bounds: Bounds {
                                    origin: point(px(rect.x), px(rect.y)),
                                    size: gpui::size(px(rect.w), px(rect.h)),
                                },
                                corner_radii: Corners::all(px(3.0)),
                                background: rect.color.into(),
                                border_widths: Edges::all(px(1.0)),
                                border_color: border_color.into(),
                                border_style: BorderStyle::default(),
                                continuous_corners: false,
                            });
                        }

                        if show_labels {
                            for rect in &rects {
                                if rect.w < 40.0 || rect.h < 18.0 {
                                    continue;
                                }

                                let label_text = rect.label.clone();
                                let contrast = if rect.color.l > 0.5 {
                                    hsla(0.0, 0.0, 0.1, 1.0)
                                } else {
                                    hsla(0.0, 0.0, 0.95, 1.0)
                                };

                                let font_size = if rect.w > 80.0 && rect.h > 30.0 {
                                    12.0
                                } else {
                                    10.0
                                };

                                let text_style = window.text_style();
                                let font = text_style.font();
                                let label_len = label_text.len();
                                let font_px = px(font_size);

                                let shaped = window.text_system().shape_line(
                                    label_text,
                                    font_px,
                                    &[TextRun {
                                        len: label_len,
                                        font,
                                        color: contrast,
                                        background_color: None,
                                        underline: None,
                                        strikethrough: None,
                                    }],
                                    None,
                                );

                                let text_w = pixels_to_f32(shaped.width);
                                let max_w = rect.w - 6.0;
                                if text_w <= max_w {
                                    let tx = rect.x + 4.0;
                                    let ty = rect.y + 4.0;
                                    let _ =
                                        shaped.paint(point(px(tx), px(ty)), font_px, window, cx);
                                }
                            }
                        }
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
    }
}
