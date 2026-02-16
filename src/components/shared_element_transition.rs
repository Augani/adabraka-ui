//! Shared element (hero) transition between views.

use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

use crate::animations::{durations, easings, lerp_pixels};

#[derive(Clone, Copy, Debug)]
struct ElementBounds {
    x: Pixels,
    y: Pixels,
    width: Pixels,
    height: Pixels,
}

impl Default for ElementBounds {
    fn default() -> Self {
        Self {
            x: px(0.0),
            y: px(0.0),
            width: px(0.0),
            height: px(0.0),
        }
    }
}

pub struct SharedElementState {
    source_bounds: Option<ElementBounds>,
    target_bounds: Option<ElementBounds>,
    is_transitioning: bool,
    progress: f32,
    version: usize,
    duration: Duration,
}

impl SharedElementState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            source_bounds: None,
            target_bounds: None,
            is_transitioning: false,
            progress: 0.0,
            version: 0,
            duration: durations::SLOW,
        }
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn set_source_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.source_bounds = Some(ElementBounds {
            x: bounds.origin.x,
            y: bounds.origin.y,
            width: bounds.size.width,
            height: bounds.size.height,
        });
    }

    pub fn set_target_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.target_bounds = Some(ElementBounds {
            x: bounds.origin.x,
            y: bounds.origin.y,
            width: bounds.size.width,
            height: bounds.size.height,
        });
    }

    pub fn transition_to(&mut self, cx: &mut Context<Self>) {
        if self.source_bounds.is_none() || self.target_bounds.is_none() {
            return;
        }

        self.is_transitioning = true;
        self.progress = 0.0;
        self.version += 1;
        cx.notify();

        let duration = self.duration;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(duration).await;
            _ = this.update(cx, |state, cx| {
                state.is_transitioning = false;
                state.progress = 1.0;
                state.source_bounds = state.target_bounds;
                cx.notify();
            });
        })
        .detach();
    }

    pub fn is_transitioning(&self) -> bool {
        self.is_transitioning
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn version(&self) -> usize {
        self.version
    }
}

#[derive(IntoElement)]
pub struct SharedElementTransition {
    id: ElementId,
    state: Entity<SharedElementState>,
    content: Option<AnyElement>,
    style: StyleRefinement,
}

impl SharedElementTransition {
    pub fn new(id: impl Into<ElementId>, state: Entity<SharedElementState>) -> Self {
        Self {
            id: id.into(),
            state,
            content: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn content(mut self, element: impl IntoElement) -> Self {
        self.content = Some(element.into_any_element());
        self
    }
}

impl Styled for SharedElementTransition {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SharedElementTransition {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let user_style = self.style;
        let state = self.state.read(cx);
        let is_transitioning = state.is_transitioning;
        let version = state.version;
        let source = state.source_bounds.unwrap_or_default();
        let target = state.target_bounds.unwrap_or_default();
        let duration = state.duration;
        let content = self.content;

        if is_transitioning {
            div()
                .id(self.id.clone())
                .absolute()
                .left(source.x)
                .top(source.y)
                .w(source.width)
                .h(source.height)
                .children(content)
                .map(|this| {
                    let mut el = this;
                    el.style().refine(&user_style);
                    el
                })
                .with_animation(
                    ElementId::Name(format!("set-{}-{}", self.id, version).into()),
                    Animation::new(duration).with_easing(easings::ease_in_out_cubic),
                    move |el, delta| {
                        let curr_x = lerp_pixels(source.x, target.x, delta);
                        let curr_y = lerp_pixels(source.y, target.y, delta);
                        let curr_w = lerp_pixels(source.width, target.width, delta);
                        let curr_h = lerp_pixels(source.height, target.height, delta);

                        el.left(curr_x).top(curr_y).w(curr_w).h(curr_h)
                    },
                )
                .into_any_element()
        } else {
            div()
                .id(self.id)
                .children(content)
                .map(|this| {
                    let mut el = this;
                    el.style().refine(&user_style);
                    el
                })
                .into_any_element()
        }
    }
}
