//! QR code generator rendering as colored quads.

use gpui::{prelude::FluentBuilder as _, *};
use qrcode::types::EcLevel;
use qrcode::QrCode;

use crate::theme::use_theme;

#[derive(Clone)]
struct QRPaintData {
    modules: Vec<Vec<bool>>,
    fg_color: Hsla,
    bg_color: Hsla,
}

#[derive(IntoElement)]
pub struct QRCodeComponent {
    data: SharedString,
    size: Pixels,
    fg_color: Option<Hsla>,
    bg_color: Option<Hsla>,
    error_correction: EcLevel,
    style: StyleRefinement,
}

impl QRCodeComponent {
    pub fn new(data: impl Into<SharedString>) -> Self {
        Self {
            data: data.into(),
            size: px(200.0),
            fg_color: None,
            bg_color: None,
            error_correction: EcLevel::M,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn fg_color(mut self, color: Hsla) -> Self {
        self.fg_color = Some(color);
        self
    }

    pub fn bg_color(mut self, color: Hsla) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn error_correction(mut self, level: EcLevel) -> Self {
        self.error_correction = level;
        self
    }
}

impl Styled for QRCodeComponent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn generate_modules(data: &str, ec_level: EcLevel) -> Vec<Vec<bool>> {
    match QrCode::with_error_correction_level(data, ec_level) {
        Ok(code) => {
            let width = code.width();
            let colors = code.into_colors();
            let mut grid = Vec::with_capacity(width);
            for row in 0..width {
                let mut row_vec = Vec::with_capacity(width);
                for col in 0..width {
                    let idx = row * width + col;
                    row_vec.push(colors[idx] == qrcode::Color::Dark);
                }
                grid.push(row_vec);
            }
            grid
        }
        Err(_) => vec![vec![false; 21]; 21],
    }
}

impl RenderOnce for QRCodeComponent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let fg = self.fg_color.unwrap_or(theme.tokens.foreground);
        let bg = self.bg_color.unwrap_or(theme.tokens.background);

        let modules = generate_modules(&self.data, self.error_correction);
        let paint_data = QRPaintData {
            modules,
            fg_color: fg,
            bg_color: bg,
        };

        let qr_size = self.size;

        div()
            .w(qr_size)
            .h(qr_size)
            .child(
                canvas(
                    move |_, _, _| paint_data,
                    move |bounds, data, window, _cx| {
                        if data.modules.is_empty() {
                            return;
                        }

                        let module_count = data.modules.len();
                        let module_size_w = bounds.size.width / px(1.0) / module_count as f32;
                        let module_size_h = bounds.size.height / px(1.0) / module_count as f32;
                        let module_size = module_size_w.min(module_size_h);

                        let total_w = module_size * module_count as f32;
                        let total_h = module_size * module_count as f32;
                        let offset_x = (bounds.size.width / px(1.0) - total_w) * 0.5;
                        let offset_y = (bounds.size.height / px(1.0) - total_h) * 0.5;

                        window.paint_quad(fill(bounds, data.bg_color));

                        for (row_idx, row) in data.modules.iter().enumerate() {
                            for (col_idx, &is_dark) in row.iter().enumerate() {
                                if is_dark {
                                    let x =
                                        bounds.left() + px(offset_x + col_idx as f32 * module_size);
                                    let y =
                                        bounds.top() + px(offset_y + row_idx as f32 * module_size);
                                    let cell_bounds = Bounds::new(
                                        point(x, y),
                                        size(px(module_size), px(module_size)),
                                    );
                                    window.paint_quad(fill(cell_bounds, data.fg_color));
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
