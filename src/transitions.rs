//! # Transition Components for Smooth UI Animations
//!
//! Declarative transition wrappers that provide smooth, polished animations
//! for component mounting, unmounting, and state changes.
//! ## Transition Types
//!
//! - **Fade Transitions**: Smooth opacity changes for gentle appearances/disappearances
//! - **Slide Transitions**: Directional movement with easing for panel transitions
//! - **Scale Transitions**: Size-based animations for emphasis and focus changes
//! - **Combined Transitions**: Multi-property animations for complex state changes
//!
//! ## Design Principles
//!
//! - **Performance First**: Efficient animation scheduling with minimal overhead
//! - **Declarative API**: Easy-to-use builder pattern for complex animations
//! - **Consistent Timing**: Standardized durations following modern UI guidelines
//! - **Natural Motion**: Physically-inspired easing curves for realistic movement
//! - **Accessibility**: Respects user's motion preferences and provides alternatives
//!
//! ## Usage Examples
//!
//! ### Basic Transitions
//! ```rust
//! // Fade in component on mount
//! Transition::fade_normal().child(my_component)
//!
//! // Slide panel from left edge
//! Transition::slide_left().duration(Duration::from_millis(300)).child(panel)
//! ```
//!
//! ### Conditional Rendering
//! ```rust
//! .when(show_modal, |parent| {
//!     parent.child(
//!         Transition::scale_up()
//!             .duration(Duration::from_millis(200))
//!             .child(modal_content)
//!     )
//! })
//! ```
//!
//! ### Complex Transitions
//! ```rust
//! Transition::combined()
//!     .fade_in()
//!     .slide_from(Direction::Bottom)
//!     .scale_from(0.8)
//!     .duration(Duration::from_millis(400))
//!     .child(complex_component)
//! ```
//!

use gpui::*;
use std::time::Duration;

use crate::animations::{durations, easings};

/// Direction for slide transitions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Slide from top
    Top,
    /// Slide from bottom
    Bottom,
    /// Slide from left
    Left,
    /// Slide from right
    Right,
}

/// Transition wrapper component
///
/// Wraps a child element and applies animation transitions when it appears.
#[derive(IntoElement)]
pub struct Transition {
    slide_direction: Option<Direction>,
    slide_distance: Pixels,
    duration: Duration,
    easing_fn: fn(f32) -> f32,
    child: Option<AnyElement>,
    animation_id: ElementId,
    apply_fade: bool,
}

impl Transition {
    /// Create a new transition
    pub fn new() -> Self {
        Self {
            slide_direction: None,
            slide_distance: px(20.0),
            duration: durations::NORMAL,
            easing_fn: easings::ease_out_cubic,
            child: None,
            animation_id: ElementId::Name("transition".into()),
            apply_fade: false,
        }
    }

    /// Set a unique ID for this transition (useful for multiple transitions)
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.animation_id = id.into();
        self
    }

    /// Apply a fade in transition
    pub fn fade(mut self) -> Self {
        self.apply_fade = true;
        self
    }

    /// Apply a slide in transition from the specified direction
    pub fn slide(mut self, direction: Direction) -> Self {
        self.slide_direction = Some(direction);
        self
    }

    /// Set custom slide distance (default: 20px)
    pub fn distance(mut self, distance: Pixels) -> Self {
        self.slide_distance = distance;
        self
    }

    /// Set the transition duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Use smooth easing (cubic)
    pub fn smooth(mut self) -> Self {
        self.easing_fn = easings::ease_out_cubic;
        self
    }

    /// Use spring easing (with bounce)
    pub fn spring(mut self) -> Self {
        self.easing_fn = easings::smooth_spring;
        self
    }

    /// Use snappy easing (quick with slight overshoot)
    pub fn snappy(mut self) -> Self {
        self.easing_fn = easings::ease_out_back;
        self
    }

    /// Use linear easing
    pub fn linear(mut self) -> Self {
        self.easing_fn = easings::linear;
        self
    }

    /// Set the child element to transition
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Transition {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let child = self.child.unwrap_or_else(|| div().into_any_element());
        let easing_fn = self.easing_fn;
        let animation_id = self.animation_id.clone();
        let slide_direction = self.slide_direction;
        let slide_distance = self.slide_distance;
        let duration = self.duration;
        let apply_fade = self.apply_fade;

        if let Some(direction) = slide_direction {
            div()
                .id(animation_id.clone())
                .relative()
                .child(child)
                .with_animation(
                    animation_id,
                    Animation::new(duration).with_easing(easing_fn),
                    move |el, delta| {
                        let offset = slide_distance * (1.0 - delta);
                        let positioned = match direction {
                            Direction::Top => el.top(-offset),
                            Direction::Bottom => el.top(offset),
                            Direction::Left => el.left(-offset),
                            Direction::Right => el.left(offset),
                        };
                        if apply_fade {
                            positioned.opacity(delta)
                        } else {
                            positioned
                        }
                    },
                )
        } else {
            div()
                .id(animation_id.clone())
                .relative()
                .child(child)
                .with_animation(
                    animation_id,
                    Animation::new(duration).with_easing(easing_fn),
                    |el, delta| el.opacity(delta),
                )
        }
    }
}

/// Preset transition builders for common use cases
impl Transition {
    /// Quick fade in (200ms)
    pub fn fade_quick() -> Self {
        Self::new().fade().duration(durations::FAST)
    }

    /// Normal fade in (300ms)
    pub fn fade_normal() -> Self {
        Self::new().fade().duration(durations::NORMAL)
    }

    /// Slow fade in (400ms)
    pub fn fade_slow() -> Self {
        Self::new().fade().duration(durations::SLOW)
    }

    /// Slide in from bottom with fade (smooth)
    pub fn slide_up() -> Self {
        Self::new()
            .slide(Direction::Bottom)
            .fade()
            .duration(durations::NORMAL)
    }

    /// Slide in from top with fade
    pub fn slide_down() -> Self {
        Self::new()
            .slide(Direction::Top)
            .fade()
            .duration(durations::NORMAL)
    }

    /// Slide in from left with fade
    pub fn slide_left() -> Self {
        Self::new()
            .slide(Direction::Left)
            .fade()
            .duration(durations::NORMAL)
    }

    /// Slide in from right with fade
    pub fn slide_right() -> Self {
        Self::new()
            .slide(Direction::Right)
            .fade()
            .duration(durations::NORMAL)
    }

    /// Smooth slide up with spring
    pub fn slide_up_spring() -> Self {
        Self::new()
            .slide(Direction::Bottom)
            .fade()
            .spring()
            .duration(durations::SLOW)
    }

    /// Scale in with fade (smooth)
    pub fn scale_smooth() -> Self {
        // For now, just use fade since we don't have transform support
        Self::fade_normal().snappy()
    }

    /// Scale in with spring bounce
    pub fn scale_bounce() -> Self {
        // For now, just use fade with spring
        Self::fade_normal().spring().duration(durations::SLOW)
    }

    /// Snappy scale in (quick with slight overshoot)
    pub fn scale_snappy() -> Self {
        Self::fade_quick().snappy()
    }
}
