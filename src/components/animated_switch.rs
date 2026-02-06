use gpui::*;
use std::time::Duration;

use crate::animations::easings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimatedSwitchTransition {
    #[default]
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
}

#[derive(IntoElement)]
pub struct AnimatedSwitch {
    id: ElementId,
    active: usize,
    children: Vec<(usize, AnyElement)>,
    previous: Option<(usize, AnyElement)>,
    transition: AnimatedSwitchTransition,
    duration: Duration,
}

impl AnimatedSwitch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            active: 0,
            children: Vec::new(),
            previous: None,
            transition: AnimatedSwitchTransition::default(),
            duration: Duration::from_millis(300),
        }
    }

    pub fn active(mut self, key: usize) -> Self {
        self.active = key;
        self
    }

    pub fn transition(mut self, transition: AnimatedSwitchTransition) -> Self {
        self.transition = transition;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn child(mut self, key: usize, content: impl IntoElement) -> Self {
        self.children.push((key, content.into_any_element()));
        self
    }

    pub fn previous(mut self, key: usize, content: impl IntoElement) -> Self {
        self.previous = Some((key, content.into_any_element()));
        self
    }
}

impl RenderOnce for AnimatedSwitch {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active_child = self
            .children
            .into_iter()
            .find(|(key, _)| *key == self.active);

        let has_previous = self.previous.is_some();
        let transition = self.transition;
        let duration = self.duration;
        let id = self.id;

        let mut container = div().relative().size_full().overflow_hidden();

        if let Some((prev_key, prev_content)) = self.previous {
            let exit_id = ElementId::Name(format!("{}-exit-{}", id, prev_key).into());

            container = container.child(
                div()
                    .absolute()
                    .inset_0()
                    .child(prev_content)
                    .with_animation(
                        exit_id,
                        Animation::new(duration).with_easing(easings::ease_in_cubic),
                        move |el, delta| apply_exit_transform(el, delta, transition),
                    ),
            );
        }

        if let Some((active_key, active_content)) = active_child {
            if has_previous {
                let enter_id = ElementId::Name(format!("{}-enter-{}", id, active_key).into());

                container =
                    container.child(div().size_full().child(active_content).with_animation(
                        enter_id,
                        Animation::new(duration).with_easing(easings::ease_out_cubic),
                        move |el, delta| apply_enter_transform(el, delta, transition),
                    ));
            } else {
                container = container.child(div().size_full().child(active_content));
            }
        }

        container
    }
}

fn apply_exit_transform(el: Div, delta: f32, transition: AnimatedSwitchTransition) -> Div {
    let slide_distance = 100.0;
    match transition {
        AnimatedSwitchTransition::Fade => el.opacity(1.0 - delta),
        AnimatedSwitchTransition::SlideLeft => {
            el.opacity(1.0 - delta).left(px(-slide_distance * delta))
        }
        AnimatedSwitchTransition::SlideRight => {
            el.opacity(1.0 - delta).left(px(slide_distance * delta))
        }
        AnimatedSwitchTransition::SlideUp => {
            el.opacity(1.0 - delta).top(px(-slide_distance * delta))
        }
        AnimatedSwitchTransition::SlideDown => {
            el.opacity(1.0 - delta).top(px(slide_distance * delta))
        }
    }
}

fn apply_enter_transform(el: Div, delta: f32, transition: AnimatedSwitchTransition) -> Div {
    let slide_distance = 100.0;
    let inverse = 1.0 - delta;
    match transition {
        AnimatedSwitchTransition::Fade => el.opacity(delta),
        AnimatedSwitchTransition::SlideLeft => el.opacity(delta).left(px(slide_distance * inverse)),
        AnimatedSwitchTransition::SlideRight => {
            el.opacity(delta).left(px(-slide_distance * inverse))
        }
        AnimatedSwitchTransition::SlideUp => el.opacity(delta).top(px(slide_distance * inverse)),
        AnimatedSwitchTransition::SlideDown => el.opacity(delta).top(px(-slide_distance * inverse)),
    }
}
