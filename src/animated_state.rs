use gpui::*;
use std::time::Duration;

use crate::animations::{easings, lerp_color, lerp_f32};

const DEFAULT_TRANSITION: Duration = Duration::from_millis(150);

#[derive(Clone, Debug)]
pub struct AnimatedInteraction {
    hover_progress: f32,
    press_progress: f32,
    focus_progress: f32,
    hover_target: f32,
    press_target: f32,
    focus_target: f32,
    transition_duration: Duration,
    easing: fn(f32) -> f32,
}

impl Default for AnimatedInteraction {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimatedInteraction {
    pub fn new() -> Self {
        Self {
            hover_progress: 0.0,
            press_progress: 0.0,
            focus_progress: 0.0,
            hover_target: 0.0,
            press_target: 0.0,
            focus_target: 0.0,
            transition_duration: DEFAULT_TRANSITION,
            easing: easings::ease_out_cubic,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.transition_duration = duration;
        self
    }

    pub fn with_easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hover_target = if hovered { 1.0 } else { 0.0 };
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.press_target = if pressed { 1.0 } else { 0.0 };
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focus_target = if focused { 1.0 } else { 0.0 };
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        let speed = dt / self.transition_duration.as_secs_f32();
        let mut changed = false;

        changed |= Self::approach(&mut self.hover_progress, self.hover_target, speed);
        changed |= Self::approach(&mut self.press_progress, self.press_target, speed);
        changed |= Self::approach(&mut self.focus_progress, self.focus_target, speed);

        changed
    }

    fn approach(current: &mut f32, target: f32, speed: f32) -> bool {
        if (*current - target).abs() < 0.001 {
            if *current != target {
                *current = target;
                return true;
            }
            return false;
        }
        *current += (target - *current) * speed.min(1.0);
        true
    }

    pub fn hover(&self) -> f32 {
        (self.easing)(self.hover_progress)
    }

    pub fn press(&self) -> f32 {
        (self.easing)(self.press_progress)
    }

    pub fn focus(&self) -> f32 {
        (self.easing)(self.focus_progress)
    }

    pub fn is_animating(&self) -> bool {
        (self.hover_progress - self.hover_target).abs() > 0.001
            || (self.press_progress - self.press_target).abs() > 0.001
            || (self.focus_progress - self.focus_target).abs() > 0.001
    }

    pub fn blend_color(&self, normal: Hsla, hover: Hsla, press: Hsla) -> Hsla {
        let base = lerp_color(normal, hover, self.hover());
        lerp_color(base, press, self.press())
    }

    pub fn blend_f32(&self, normal: f32, hover: f32, press: f32) -> f32 {
        let base = lerp_f32(normal, hover, self.hover());
        lerp_f32(base, press, self.press())
    }
}
