use crate::animations::easings;
use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

#[derive(IntoElement)]
pub struct TextHighlight {
    id: ElementId,
    color: Option<Hsla>,
    duration: Duration,
    delay: Duration,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl TextHighlight {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            color: None,
            duration: Duration::from_millis(600),
            delay: Duration::from_millis(0),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}

impl Styled for TextHighlight {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TextHighlight {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TextHighlight {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let highlight_color = self.color.unwrap_or(hsla(0.15, 0.9, 0.6, 0.3));
        let anim_id: ElementId = ElementId::Name(format!("{}-sweep", self.id).into());
        let total_duration =
            Duration::from_millis(self.duration.as_millis() as u64 + self.delay.as_millis() as u64);
        let delay = self.delay;
        let user_style = self.style;

        let overlay = div()
            .id(anim_id.clone())
            .absolute()
            .top_0()
            .left_0()
            .h_full()
            .bg(highlight_color)
            .with_animation(
                anim_id,
                Animation::new(total_duration).with_easing(easings::ease_out_cubic),
                move |el, delta| {
                    let t = if total_duration.as_millis() > 0 {
                        let delay_frac =
                            delay.as_millis() as f32 / total_duration.as_millis() as f32;
                        ((delta - delay_frac) / (1.0 - delay_frac)).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    el.w(relative(t))
                },
            );

        let mut container = div().relative().child(overlay).map(|mut el| {
            el.style().refine(&user_style);
            el
        });

        for child in self.children {
            container = container.child(div().relative().child(child));
        }

        container
    }
}
