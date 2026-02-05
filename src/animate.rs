use gpui::*;
use std::time::Duration;

use crate::animations::easings;

#[derive(Clone)]
pub struct AnimationPreset {
    duration: Duration,
    easing: fn(f32) -> f32,
    delay: Duration,
}

impl AnimationPreset {
    pub fn new(duration: Duration, easing: fn(f32) -> f32) -> Self {
        Self {
            duration,
            easing,
            delay: Duration::ZERO,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_easing(&self) -> fn(f32) -> f32 {
        self.easing
    }

    pub fn get_delay(&self) -> Duration {
        self.delay
    }

    pub fn to_animation(&self) -> Animation {
        Animation::new(self.duration).with_easing(self.easing)
    }
}

pub fn fade_in() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(200), easings::ease_out_cubic)
}

pub fn fade_out() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(200), easings::ease_in_cubic)
}

pub fn slide_up() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(250), easings::ease_out_cubic)
}

pub fn slide_down() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(250), easings::ease_out_cubic)
}

pub fn scale_in() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(200), easings::ease_out_back)
}

pub fn bounce_in() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(400), easings::elastic)
}

pub fn slide_in_left() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(250), easings::ease_out_cubic)
}

pub fn slide_in_right() -> AnimationPreset {
    AnimationPreset::new(Duration::from_millis(250), easings::ease_out_cubic)
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnimationRepeat {
    Once,
    Count(u32),
    Infinite,
}

pub struct KeyframeAnimation {
    id: SharedString,
    keyframes: Vec<(f32, f32)>,
    duration: Duration,
    repeat: AnimationRepeat,
    easing: fn(f32) -> f32,
}

impl KeyframeAnimation {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            keyframes: vec![(0.0, 0.0), (1.0, 1.0)],
            duration: Duration::from_millis(300),
            repeat: AnimationRepeat::Once,
            easing: easings::linear,
        }
    }

    pub fn at(mut self, pct: f32, value: f32) -> Self {
        let pct = pct.clamp(0.0, 1.0);
        if let Some(pos) = self
            .keyframes
            .iter()
            .position(|(p, _)| (*p - pct).abs() < f32::EPSILON)
        {
            self.keyframes[pos] = (pct, value);
        } else {
            self.keyframes.push((pct, value));
            self.keyframes
                .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub fn id(&self) -> &SharedString {
        &self.id
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_repeat(&self) -> &AnimationRepeat {
        &self.repeat
    }

    pub fn interpolate(&self, progress: f32) -> f32 {
        let progress = progress.clamp(0.0, 1.0);

        if self.keyframes.len() < 2 {
            return if self.keyframes.is_empty() {
                0.0
            } else {
                self.keyframes[0].1
            };
        }

        let eased = (self.easing)(progress);

        let mut prev = &self.keyframes[0];
        for kf in &self.keyframes[1..] {
            if eased <= kf.0 {
                let range = kf.0 - prev.0;
                if range <= f32::EPSILON {
                    return kf.1;
                }
                let local_t = (eased - prev.0) / range;
                return prev.1 + (kf.1 - prev.1) * local_t;
            }
            prev = kf;
        }

        self.keyframes.last().map(|kf| kf.1).unwrap_or(1.0)
    }

    pub fn to_animation(&self) -> Animation {
        let anim = Animation::new(self.duration);
        match &self.repeat {
            AnimationRepeat::Once => anim,
            AnimationRepeat::Infinite => anim.repeat(),
            AnimationRepeat::Count(_) => anim.repeat(),
        }
    }
}

#[derive(Clone)]
pub struct StaggerConfig {
    delay_per_child: Duration,
    preset: AnimationPreset,
}

impl StaggerConfig {
    pub fn new() -> Self {
        Self {
            delay_per_child: Duration::from_millis(50),
            preset: fade_in(),
        }
    }

    pub fn delay_per_child(mut self, delay: Duration) -> Self {
        self.delay_per_child = delay;
        self
    }

    pub fn animation(mut self, preset: AnimationPreset) -> Self {
        self.preset = preset;
        self
    }

    pub fn delay_for_index(&self, index: usize) -> Duration {
        self.preset.delay + self.delay_per_child * index as u32
    }

    pub fn preset_for_index(&self, index: usize) -> AnimationPreset {
        let mut preset = self.preset.clone();
        preset.delay = self.delay_for_index(index);
        preset
    }

    pub fn get_preset(&self) -> &AnimationPreset {
        &self.preset
    }
}

impl Default for StaggerConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Transition {
    property: SharedString,
    duration: Duration,
    easing: fn(f32) -> f32,
    delay: Duration,
}

impl Transition {
    pub fn new(property: impl Into<SharedString>) -> Self {
        Self {
            property: property.into(),
            duration: Duration::from_millis(200),
            easing: easings::ease_out_cubic,
            delay: Duration::ZERO,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn get_property(&self) -> &SharedString {
        &self.property
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_easing(&self) -> fn(f32) -> f32 {
        self.easing
    }

    pub fn get_delay(&self) -> Duration {
        self.delay
    }

    pub fn to_animation(&self) -> Animation {
        Animation::new(self.duration).with_easing(self.easing)
    }
}
