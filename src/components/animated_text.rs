use crate::animations::easings;
use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAnimation {
    FadeUp,
    FadeDown,
    SlideUp,
    SlideDown,
    Scale,
    Wave,
}

#[derive(IntoElement)]
pub struct AnimatedText {
    id: ElementId,
    text: SharedString,
    animation: TextAnimation,
    stagger: Duration,
    duration: Duration,
    text_size: Option<Pixels>,
    font_weight: Option<FontWeight>,
    text_color: Option<Hsla>,
    style: StyleRefinement,
}

impl AnimatedText {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            animation: TextAnimation::FadeUp,
            stagger: Duration::from_millis(30),
            duration: Duration::from_millis(400),
            text_size: None,
            font_weight: None,
            text_color: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn animation(mut self, animation: TextAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn stagger(mut self, stagger: Duration) -> Self {
        self.stagger = stagger;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn text_size(mut self, size: Pixels) -> Self {
        self.text_size = Some(size);
        self
    }

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl Styled for AnimatedText {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AnimatedText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let chars: Vec<char> = self.text.chars().collect();
        let animation_type = self.animation;
        let stagger = self.stagger;
        let duration = self.duration;
        let user_style = self.style;

        let mut row = div().flex().flex_row().flex_wrap().map(|mut el| {
            el.style().refine(&user_style);
            el
        });

        if let Some(size) = self.text_size {
            row = row.text_size(size);
        }
        if let Some(weight) = self.font_weight {
            row = row.font_weight(weight);
        }
        if let Some(color) = self.text_color {
            row = row.text_color(color);
        }

        for (i, ch) in chars.into_iter().enumerate() {
            let s: SharedString = ch.to_string().into();
            let char_id: ElementId = ElementId::Name(format!("{}-char-{}", self.id, i).into());
            let delay = stagger.as_millis() as u64 * i as u64;
            let total_duration = Duration::from_millis(duration.as_millis() as u64 + delay);

            let char_el = div().id(char_id.clone()).child(s);

            let anim_el = char_el.with_animation(
                char_id,
                Animation::new(total_duration).with_easing(easings::ease_out_cubic),
                move |el, delta| {
                    let char_t = if total_duration.as_millis() > 0 {
                        let delay_fraction = delay as f32 / total_duration.as_millis() as f32;
                        ((delta - delay_fraction) / (1.0 - delay_fraction)).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };

                    match animation_type {
                        TextAnimation::FadeUp => {
                            let offset = (1.0 - char_t) * 12.0;
                            el.opacity(char_t).mt(px(offset))
                        }
                        TextAnimation::FadeDown => {
                            let offset = (1.0 - char_t) * -12.0;
                            el.opacity(char_t).mt(px(offset))
                        }
                        TextAnimation::SlideUp => {
                            let offset = (1.0 - char_t) * 20.0;
                            el.mt(px(offset))
                        }
                        TextAnimation::SlideDown => {
                            let offset = (1.0 - char_t) * -20.0;
                            el.mt(px(offset))
                        }
                        TextAnimation::Scale => {
                            let size_px = 8.0 + char_t * 8.0;
                            el.opacity(char_t).text_size(px(size_px))
                        }
                        TextAnimation::Wave => {
                            let wave_offset =
                                (char_t * std::f32::consts::PI * 2.0 + i as f32 * 0.5).sin()
                                    * 6.0
                                    * (1.0 - char_t * 0.5);
                            el.mt(px(wave_offset))
                        }
                    }
                },
            );

            row = row.child(anim_el);
        }

        row
    }
}
