use crate::animations::easings;
use gpui::*;
use std::time::Duration;

#[derive(IntoElement)]
pub struct PulseIndicator {
    base: Stateful<Div>,
    color: Hsla,
    dot_size: Pixels,
    speed: Duration,
}

impl PulseIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id.into()),
            color: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
            dot_size: px(8.0),
            speed: Duration::from_secs(2),
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.dot_size = size;
        self
    }

    pub fn speed(mut self, speed: Duration) -> Self {
        self.speed = speed;
        self
    }
}

impl RenderOnce for PulseIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color;
        let dot = self.dot_size;
        let ring_max = dot * 2.5;
        let speed = self.speed;

        self.base
            .flex()
            .items_center()
            .justify_center()
            .size(ring_max)
            .child(
                div()
                    .absolute()
                    .rounded_full()
                    .bg(color.opacity(0.6))
                    .size(dot)
                    .with_animation(
                        "pulse-ring",
                        Animation::new(speed)
                            .repeat()
                            .with_easing(easings::ease_out_cubic),
                        move |this, delta| {
                            let current_size = dot + (ring_max - dot) * delta;
                            let current_opacity = 0.6 * (1.0 - delta);
                            this.size(current_size).opacity(current_opacity)
                        },
                    ),
            )
            .child(div().absolute().rounded_full().bg(color).size(dot))
    }
}

impl Styled for PulseIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for PulseIndicator {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for PulseIndicator {}

impl ParentElement for PulseIndicator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements)
    }
}
