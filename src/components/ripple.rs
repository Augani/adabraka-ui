use gpui::*;
use std::time::Duration;

use crate::animations::easings;

#[derive(IntoElement)]
pub struct Ripple {
    id: ElementId,
    origin: Point<Pixels>,
    color: Hsla,
    duration: Duration,
    max_size: Pixels,
}

impl Ripple {
    pub fn new(id: impl Into<ElementId>, origin: Point<Pixels>, color: Hsla) -> Self {
        Self {
            id: id.into(),
            origin,
            color,
            duration: Duration::from_millis(400),
            max_size: px(150.0),
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn max_size(mut self, size: Pixels) -> Self {
        self.max_size = size;
        self
    }
}

impl RenderOnce for Ripple {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let origin = self.origin;
        let max_size = self.max_size;
        let color = self.color;

        div()
            .absolute()
            .overflow_hidden()
            .size_full()
            .top_0()
            .left_0()
            .child(
                div()
                    .id(self.id)
                    .absolute()
                    .rounded_full()
                    .bg(color.opacity(0.2))
                    .with_animation(
                        "ripple-expand",
                        Animation::new(self.duration).with_easing(easings::ease_out_cubic),
                        move |el, delta| {
                            let size = max_size * delta;
                            el.size(size)
                                .left(origin.x - size / 2.0)
                                .top(origin.y - size / 2.0)
                                .opacity((1.0 - delta) * 0.6)
                        },
                    ),
            )
    }
}
