//! Parses SVG path data and renders via GPUI PathBuilder.

use gpui::{prelude::FluentBuilder as _, *};

use crate::theme::use_theme;

#[derive(Clone)]
enum SvgCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CurveTo(f32, f32, f32, f32, f32, f32),
    QuadTo(f32, f32, f32, f32),
    Close,
}

fn parse_svg_path(data: &str) -> Vec<SvgCommand> {
    let mut commands = Vec::new();
    let mut chars = data.chars().peekable();
    let mut current_cmd = ' ';

    fn skip_ws_and_commas(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() || c == ',' {
                chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<f32> {
        skip_ws_and_commas(chars);
        let mut s = String::new();
        if let Some(&c) = chars.peek() {
            if c == '-' || c == '+' {
                s.push(c);
                chars.next();
            }
        }
        let mut has_dot = false;
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                chars.next();
            } else if c == '.' && !has_dot {
                has_dot = true;
                s.push(c);
                chars.next();
            } else {
                break;
            }
        }
        if s.is_empty() || s == "-" || s == "+" {
            None
        } else {
            s.parse().ok()
        }
    }

    while chars.peek().is_some() {
        skip_ws_and_commas(&mut chars);
        if let Some(&c) = chars.peek() {
            if c.is_ascii_alphabetic() {
                current_cmd = c;
                chars.next();
            }
        }

        match current_cmd {
            'M' => {
                if let (Some(x), Some(y)) = (parse_number(&mut chars), parse_number(&mut chars)) {
                    commands.push(SvgCommand::MoveTo(x, y));
                    current_cmd = 'L';
                } else {
                    break;
                }
            }
            'L' => {
                if let (Some(x), Some(y)) = (parse_number(&mut chars), parse_number(&mut chars)) {
                    commands.push(SvgCommand::LineTo(x, y));
                } else {
                    break;
                }
            }
            'H' => {
                if let Some(x) = parse_number(&mut chars) {
                    commands.push(SvgCommand::LineTo(x, f32::NAN));
                } else {
                    break;
                }
            }
            'V' => {
                if let Some(y) = parse_number(&mut chars) {
                    commands.push(SvgCommand::LineTo(f32::NAN, y));
                } else {
                    break;
                }
            }
            'C' => {
                if let (Some(x1), Some(y1), Some(x2), Some(y2), Some(x), Some(y)) = (
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                ) {
                    commands.push(SvgCommand::CurveTo(x1, y1, x2, y2, x, y));
                } else {
                    break;
                }
            }
            'Q' => {
                if let (Some(cx1), Some(cy1), Some(x), Some(y)) = (
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                    parse_number(&mut chars),
                ) {
                    commands.push(SvgCommand::QuadTo(cx1, cy1, x, y));
                } else {
                    break;
                }
            }
            'Z' | 'z' => {
                commands.push(SvgCommand::Close);
                current_cmd = 'M';
            }
            _ => {
                chars.next();
            }
        }
    }

    commands
}

#[derive(Clone)]
struct SvgPaintData {
    commands: Vec<SvgCommand>,
    view_box: Bounds<f32>,
    fill_color: Hsla,
    stroke_color: Option<Hsla>,
    stroke_width: f32,
}

#[derive(IntoElement)]
pub struct SVGRenderer {
    path_data: SharedString,
    view_box: Bounds<f32>,
    fill_color: Option<Hsla>,
    stroke_color: Option<Hsla>,
    stroke_width: f32,
    style: StyleRefinement,
}

impl SVGRenderer {
    pub fn new() -> Self {
        Self {
            path_data: SharedString::default(),
            view_box: Bounds::new(point(0.0_f32, 0.0_f32), size(100.0_f32, 100.0_f32)),
            fill_color: None,
            stroke_color: None,
            stroke_width: 1.0,
            style: StyleRefinement::default(),
        }
    }

    pub fn path_data(mut self, data: impl Into<SharedString>) -> Self {
        self.path_data = data.into();
        self
    }

    pub fn view_box(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.view_box = Bounds::new(point(x, y), size(w, h));
        self
    }

    pub fn fill(mut self, color: Hsla) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn stroke(mut self, color: Hsla) -> Self {
        self.stroke_color = Some(color);
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }
}

impl Styled for SVGRenderer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn transform_point(
    vx: f32,
    vy: f32,
    view_box: &Bounds<f32>,
    bounds: &Bounds<Pixels>,
) -> Point<Pixels> {
    let scale_x = bounds.size.width / px(1.0) / view_box.size.width;
    let scale_y = bounds.size.height / px(1.0) / view_box.size.height;
    let scale = scale_x.min(scale_y);

    let offset_x = (bounds.size.width / px(1.0) - view_box.size.width * scale) * 0.5;
    let offset_y = (bounds.size.height / px(1.0) - view_box.size.height * scale) * 0.5;

    point(
        bounds.left() + px(offset_x + (vx - view_box.origin.x) * scale),
        bounds.top() + px(offset_y + (vy - view_box.origin.y) * scale),
    )
}

impl RenderOnce for SVGRenderer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let fill_color = self.fill_color.unwrap_or(theme.tokens.foreground);
        let commands = parse_svg_path(&self.path_data);

        let paint_data = SvgPaintData {
            commands,
            view_box: self.view_box,
            fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
        };

        div()
            .size_full()
            .child(
                canvas(
                    move |_, _, _| paint_data,
                    move |bounds, data, window, _cx| {
                        if data.commands.is_empty() {
                            return;
                        }

                        let mut current_x = 0.0_f32;
                        let mut current_y = 0.0_f32;

                        if let Some(stroke_color) = data.stroke_color {
                            let mut builder = PathBuilder::stroke(px(data.stroke_width));
                            let mut started = false;

                            for cmd in &data.commands {
                                match cmd {
                                    SvgCommand::MoveTo(x, y) => {
                                        let pt = transform_point(*x, *y, &data.view_box, &bounds);
                                        if started {
                                            if let Ok(path) = builder.build() {
                                                window.paint_path(path, stroke_color);
                                            }
                                            builder = PathBuilder::stroke(px(data.stroke_width));
                                        }
                                        builder.move_to(pt);
                                        current_x = *x;
                                        current_y = *y;
                                        started = true;
                                    }
                                    SvgCommand::LineTo(x, y) => {
                                        let fx = if x.is_nan() { current_x } else { *x };
                                        let fy = if y.is_nan() { current_y } else { *y };
                                        let pt = transform_point(fx, fy, &data.view_box, &bounds);
                                        builder.line_to(pt);
                                        current_x = fx;
                                        current_y = fy;
                                    }
                                    SvgCommand::CurveTo(x1, y1, x2, y2, x, y) => {
                                        let _cp1 =
                                            transform_point(*x1, *y1, &data.view_box, &bounds);
                                        let cp2 =
                                            transform_point(*x2, *y2, &data.view_box, &bounds);
                                        let end = transform_point(*x, *y, &data.view_box, &bounds);
                                        builder.curve_to(cp2, end);
                                        current_x = *x;
                                        current_y = *y;
                                    }
                                    SvgCommand::QuadTo(cx1, cy1, x, y) => {
                                        let cp =
                                            transform_point(*cx1, *cy1, &data.view_box, &bounds);
                                        let end = transform_point(*x, *y, &data.view_box, &bounds);
                                        builder.curve_to(cp, end);
                                        current_x = *x;
                                        current_y = *y;
                                    }
                                    SvgCommand::Close => {
                                        builder.close();
                                    }
                                }
                            }

                            if started {
                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, stroke_color);
                                }
                            }
                        }

                        {
                            let mut builder = PathBuilder::fill();
                            let mut started = false;
                            current_x = 0.0;
                            current_y = 0.0;

                            for cmd in &data.commands {
                                match cmd {
                                    SvgCommand::MoveTo(x, y) => {
                                        if started {
                                            builder.close();
                                            if let Ok(path) = builder.build() {
                                                window.paint_path(path, data.fill_color);
                                            }
                                            builder = PathBuilder::fill();
                                        }
                                        let pt = transform_point(*x, *y, &data.view_box, &bounds);
                                        builder.move_to(pt);
                                        current_x = *x;
                                        current_y = *y;
                                        started = true;
                                    }
                                    SvgCommand::LineTo(x, y) => {
                                        let fx = if x.is_nan() { current_x } else { *x };
                                        let fy = if y.is_nan() { current_y } else { *y };
                                        let pt = transform_point(fx, fy, &data.view_box, &bounds);
                                        builder.line_to(pt);
                                        current_x = fx;
                                        current_y = fy;
                                    }
                                    SvgCommand::CurveTo(x1, y1, x2, y2, x, y) => {
                                        let _cp1 =
                                            transform_point(*x1, *y1, &data.view_box, &bounds);
                                        let cp2 =
                                            transform_point(*x2, *y2, &data.view_box, &bounds);
                                        let end = transform_point(*x, *y, &data.view_box, &bounds);
                                        builder.curve_to(cp2, end);
                                        current_x = *x;
                                        current_y = *y;
                                    }
                                    SvgCommand::QuadTo(cx1, cy1, x, y) => {
                                        let cp =
                                            transform_point(*cx1, *cy1, &data.view_box, &bounds);
                                        let end = transform_point(*x, *y, &data.view_box, &bounds);
                                        builder.curve_to(cp, end);
                                        current_x = *x;
                                        current_y = *y;
                                    }
                                    SvgCommand::Close => {
                                        builder.close();
                                    }
                                }
                            }

                            if started {
                                builder.close();
                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, data.fill_color);
                                }
                            }
                        }
                    },
                )
                .size_full(),
            )
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
