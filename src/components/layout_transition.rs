//! Layout transition - container that applies staggered entry animations to children.

use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

use crate::animations::{durations, easings};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LayoutAnimation {
    #[default]
    FadeUp,
    FadeDown,
    SlideLeft,
    SlideRight,
    Scale,
}

#[derive(IntoElement)]
pub struct LayoutTransition {
    id: ElementId,
    children: Vec<AnyElement>,
    duration: Duration,
    stagger: Duration,
    animation: LayoutAnimation,
    version: usize,
    style: StyleRefinement,
}

impl LayoutTransition {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            duration: durations::NORMAL,
            stagger: Duration::from_millis(50),
            animation: LayoutAnimation::default(),
            version: 0,
            style: StyleRefinement::default(),
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn stagger(mut self, stagger: Duration) -> Self {
        self.stagger = stagger;
        self
    }

    pub fn animation(mut self, animation: LayoutAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn version(mut self, version: usize) -> Self {
        self.version = version;
        self
    }
}

impl Styled for LayoutTransition {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for LayoutTransition {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for LayoutTransition {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let user_style = self.style;
        let duration = self.duration;
        let stagger = self.stagger;
        let animation = self.animation;
        let version = self.version;

        div()
            .id(self.id)
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(move |(idx, child)| {
                        let delay = Duration::from_millis(stagger.as_millis() as u64 * idx as u64);
                        let total_duration = duration + delay;
                        let delay_fraction = if total_duration.as_secs_f32() > 0.0 {
                            delay.as_secs_f32() / total_duration.as_secs_f32()
                        } else {
                            0.0
                        };

                        div()
                            .id(ElementId::Name(
                                format!("lt-child-{}-{}", idx, version).into(),
                            ))
                            .child(child)
                            .with_animation(
                                ElementId::Name(format!("lt-anim-{}-{}", idx, version).into()),
                                Animation::new(total_duration).with_easing(easings::ease_out_cubic),
                                move |el, raw_delta| {
                                    let delta = if raw_delta <= delay_fraction {
                                        0.0
                                    } else {
                                        ((raw_delta - delay_fraction) / (1.0 - delay_fraction))
                                            .min(1.0)
                                    };

                                    match animation {
                                        LayoutAnimation::FadeUp => {
                                            el.opacity(delta).mt(px(-12.0 * (1.0 - delta)))
                                        }
                                        LayoutAnimation::FadeDown => {
                                            el.opacity(delta).mt(px(12.0 * (1.0 - delta)))
                                        }
                                        LayoutAnimation::SlideLeft => {
                                            el.opacity(delta).ml(px(20.0 * (1.0 - delta)))
                                        }
                                        LayoutAnimation::SlideRight => {
                                            el.opacity(delta).ml(px(-20.0 * (1.0 - delta)))
                                        }
                                        LayoutAnimation::Scale => {
                                            let scale_val = 0.8 + 0.2 * delta;
                                            el.opacity(delta)
                                                .w(relative(scale_val))
                                                .h(relative(scale_val))
                                        }
                                    }
                                },
                            )
                    }),
            )
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
