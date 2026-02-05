use gpui::*;
use std::time::Duration;

use crate::animations::easings;

#[derive(IntoElement)]
pub struct ContentTransition {
    id: ElementId,
    old_content: Option<AnyElement>,
    new_content: AnyElement,
    duration: Duration,
    transitioning: bool,
}

impl ContentTransition {
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            old_content: None,
            new_content: content.into_any_element(),
            duration: Duration::from_millis(200),
            transitioning: false,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn crossfade_from(mut self, old: impl IntoElement) -> Self {
        self.old_content = Some(old.into_any_element());
        self.transitioning = true;
        self
    }
}

impl RenderOnce for ContentTransition {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let duration = self.duration;

        if self.transitioning {
            let mut container = div().relative().size_full();

            if let Some(old) = self.old_content {
                container = container.child(div().absolute().inset_0().child(old).with_animation(
                    ElementId::Name(format!("{}-old", self.id).into()),
                    Animation::new(duration).with_easing(easings::ease_in_cubic),
                    |el, delta| el.opacity(1.0 - delta),
                ));
            }

            container = container.child(div().size_full().child(self.new_content).with_animation(
                ElementId::Name(format!("{}-new", self.id).into()),
                Animation::new(duration).with_easing(easings::ease_out_cubic),
                |el, delta| el.opacity(delta),
            ));

            container
        } else {
            div().size_full().child(self.new_content)
        }
    }
}

pub struct ContentTransitionState {
    current_key: u64,
}

impl ContentTransitionState {
    pub fn new() -> Self {
        Self { current_key: 0 }
    }

    pub fn set_key(&mut self, key: u64) -> bool {
        if self.current_key != key {
            self.current_key = key;
            true
        } else {
            false
        }
    }

    pub fn key(&self) -> u64 {
        self.current_key
    }
}

impl Default for ContentTransitionState {
    fn default() -> Self {
        Self::new()
    }
}
