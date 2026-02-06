use gpui::*;
use std::time::Duration;

use crate::animations::{durations, easings, lerp_pixels};

pub struct AnimatedPresenceState {
    is_visible: bool,
    is_animating_out: bool,
    version: usize,
    exit_duration: Duration,
}

impl AnimatedPresenceState {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            is_animating_out: false,
            version: 0,
            exit_duration: durations::NORMAL,
        }
    }

    pub fn set_visible(&mut self, visible: bool, cx: &mut Context<Self>) {
        if visible {
            if self.is_visible && !self.is_animating_out {
                return;
            }
            self.is_visible = true;
            self.is_animating_out = false;
            self.version += 1;
            cx.notify();
        } else {
            if !self.is_visible || self.is_animating_out {
                return;
            }
            self.is_animating_out = true;
            self.version += 1;
            cx.notify();

            let duration = self.exit_duration;
            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(duration).await;
                _ = this.update(cx, |state, cx| {
                    if state.is_animating_out {
                        state.is_visible = false;
                        state.is_animating_out = false;
                        cx.notify();
                    }
                });
            })
            .detach();
        }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn is_animating_out(&self) -> bool {
        self.is_animating_out
    }
}

impl Default for AnimatedPresenceState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(IntoElement)]
pub struct AnimatedPresence {
    base: Div,
    id: ElementId,
    state: Entity<AnimatedPresenceState>,
    show: Option<bool>,
    enter_duration: Duration,
    exit_duration: Duration,
}

impl AnimatedPresence {
    pub fn new(id: impl Into<ElementId>, state: Entity<AnimatedPresenceState>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            state,
            show: None,
            enter_duration: durations::NORMAL,
            exit_duration: durations::FAST,
        }
    }

    pub fn show(mut self, visible: bool) -> Self {
        self.show = Some(visible);
        self
    }

    pub fn enter_duration(mut self, duration: Duration) -> Self {
        self.enter_duration = duration;
        self
    }

    pub fn exit_duration(mut self, duration: Duration) -> Self {
        self.exit_duration = duration;
        self
    }
}

impl RenderOnce for AnimatedPresence {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if let Some(desired) = self.show {
            let state = self.state.read(cx);
            let needs_show = desired && !state.is_visible;
            let needs_hide = !desired && state.is_visible && !state.is_animating_out;

            if needs_show || needs_hide {
                let exit_dur = self.exit_duration;
                self.state.update(cx, |state, cx| {
                    state.exit_duration = exit_dur;
                    state.set_visible(desired, cx);
                });
            }
        }

        let state = self.state.read(cx);
        let is_visible = state.is_visible;
        let is_animating_out = state.is_animating_out;
        let version = state.version;

        if !is_visible && !is_animating_out {
            return div().into_any_element();
        }

        let enter_dur = self.enter_duration;
        let exit_dur = self.exit_duration;
        let slide_offset = px(8.0);

        if is_animating_out {
            self.base
                .with_animation(
                    ElementId::Name(format!("{}-exit-{}", self.id, version).into()),
                    Animation::new(exit_dur).with_easing(easings::ease_in_cubic),
                    move |el, delta| {
                        el.opacity(1.0 - delta)
                            .mt(lerp_pixels(px(0.0), slide_offset, delta))
                    },
                )
                .into_any_element()
        } else {
            self.base
                .with_animation(
                    ElementId::Name(format!("{}-enter-{}", self.id, version).into()),
                    Animation::new(enter_dur).with_easing(easings::ease_out_cubic),
                    move |el, delta| {
                        el.opacity(delta)
                            .mt(lerp_pixels(slide_offset, px(0.0), delta))
                    },
                )
                .into_any_element()
        }
    }
}

impl Styled for AnimatedPresence {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for AnimatedPresence {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for AnimatedPresence {}

impl ParentElement for AnimatedPresence {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements)
    }
}
