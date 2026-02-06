use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

use crate::animations::easings;
use crate::components::progress::{ProgressSize, ProgressVariant};
use crate::theme::use_theme;

#[derive(IntoElement)]
pub struct AnimatedProgress {
    id: ElementId,
    base: Div,
    value: f32,
    variant: ProgressVariant,
    size: ProgressSize,
    shimmer: bool,
    color: Option<Hsla>,
    duration: Duration,
}

impl AnimatedProgress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            value: 0.0,
            variant: ProgressVariant::Default,
            size: ProgressSize::Md,
            shimmer: false,
            color: None,
            duration: Duration::from_millis(500),
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    pub fn shimmer(mut self, shimmer: bool) -> Self {
        self.shimmer = shimmer;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

impl Styled for AnimatedProgress {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for AnimatedProgress {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let height = match self.size {
            ProgressSize::Sm => px(4.0),
            ProgressSize::Md => px(8.0),
            ProgressSize::Lg => px(12.0),
        };

        let bar_color = self.color.unwrap_or_else(|| match self.variant {
            ProgressVariant::Default => theme.tokens.primary,
            ProgressVariant::Success => rgb(0x22c55e).into(),
            ProgressVariant::Warning => rgb(0xf59e0b).into(),
            ProgressVariant::Destructive => theme.tokens.destructive,
        });

        let target_value = self.value;
        let duration = self.duration;
        let shimmer_enabled = self.shimmer;
        let value_key = (target_value * 10000.0) as u32;

        self.base.w_full().child(
            div()
                .relative()
                .w_full()
                .h(height)
                .rounded(theme.tokens.radius_lg)
                .bg(theme.tokens.muted)
                .overflow_hidden()
                .child(
                    div()
                        .id(self.id.clone())
                        .absolute()
                        .top_0()
                        .left_0()
                        .h_full()
                        .bg(bar_color)
                        .rounded(theme.tokens.radius_lg)
                        .overflow_hidden()
                        .when(shimmer_enabled, |el| {
                            el.child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .bottom_0()
                                    .w(px(120.0))
                                    .bg(gpui::linear_gradient(
                                        90.0,
                                        gpui::linear_color_stop(gpui::transparent_black(), 0.0),
                                        gpui::linear_color_stop(hsla(0.0, 0.0, 1.0, 0.2), 1.0),
                                    ))
                                    .with_animation(
                                        "shimmer-sweep",
                                        Animation::new(Duration::from_millis(1500))
                                            .repeat()
                                            .with_easing(gpui::linear),
                                        move |el, delta| {
                                            let start = px(-120.0);
                                            let end = px(600.0);
                                            let pos = start + (end - start) * delta;
                                            el.left(pos)
                                        },
                                    ),
                            )
                        })
                        .with_animation(
                            ("progress-fill", value_key),
                            Animation::new(duration).with_easing(easings::ease_out_cubic),
                            move |el, delta| {
                                let width = target_value * delta;
                                el.w(relative(width))
                            },
                        ),
                ),
        )
    }
}
