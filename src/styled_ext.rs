use gpui::*;

pub trait StyledExt: Styled + Sized {
    fn center(self) -> Self {
        self.flex().items_center().justify_center()
    }

    fn stack(self) -> Self {
        self.flex().flex_col()
    }

    fn row(self) -> Self {
        self.flex().flex_row()
    }

    fn glass(self, opacity: f32) -> Self {
        let bg_color = hsla(0.0, 0.0, 1.0, 0.08 * opacity);
        let border_color = hsla(0.0, 0.0, 1.0, 0.12 * opacity);
        self.bg(bg_color).border_1().border_color(border_color)
    }

    fn elevated(self, level: u8) -> Self {
        let shadow = match level {
            0 => return self,
            1 => BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.05),
            },
            2 => BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            3 => BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            4 => BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            _ => BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
        };
        self.shadow(vec![shadow])
    }

    fn ring(self, color: Hsla, width: Pixels) -> Self {
        let shadow = BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: width,
            inset: false,
            color,
        };
        self.shadow(vec![shadow])
    }
}

impl<T: Styled + Sized> StyledExt for T {}
