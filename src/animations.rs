//! # Animation Utilities and Presets
//!
//! Professional animation system providing smooth, polished easing functions and reusable
//! animation configurations for desktop application interfaces.
//! ## Features
//!
//! - **Easing Functions**: Mathematical easing curves for natural motion
//! - **Duration Presets**: Standardized timing values following UI guidelines
//! - **Animation Presets**: Ready-to-use animations for common interactions
//! - **Spring Physics**: Realistic bouncy animations with configurable parameters
//! - **Performance**: Optimized calculations with minimal runtime overhead
//!
//! ## Easing Categories
//!
//! - **Linear**: Constant velocity (rarely natural for UI)
//! - **Quadratic/Cubic/Quartic**: Smooth acceleration/deceleration
//! - **Exponential**: Dramatic acceleration (good for entrances)
//! - **Spring**: Natural bouncy motion with physics simulation
//! - **Back**: Slight overshoot for emphasis (subtle bounce effect)
//!
//! ## Usage Examples
//!
//! ### Basic Animation
//! ```rust
//! use adabraka_ui::animations::*;
//!
//! // Fade in with smooth easing
//! div()
//!     .with_animation(
//!         "fade-in",
//!         fade_in(Duration::from_millis(300)),
//!         |el, delta| el.opacity(delta)
//!     )
//! ```
//!
//! ### Spring Animation
//! ```rust
//! // Natural slide with bounce
//! div()
//!     .with_animation(
//!         "slide-spring",
//!         spring_slide(Duration::from_millis(400)),
//!         |el, delta| el.ml(px(-100.0 * (1.0 - delta)))
//!     )
//! ```
//!
//! ### Preset Usage
//! ```rust
//! // Use predefined animations
//! div().with_animation(
//!     "bounce",
//!     presets::bounce_in(),
//!     |el, delta| el.scale(delta)
//! )
//! ```
//!
//! ## Design Decisions
//!
//! - **Performance First**: All calculations are lightweight and cache-friendly
//! - **Natural Motion**: Easing curves based on real-world physics observations
//! - **Consistency**: Standardized durations and easing across the library
//! - **Extensibility**: Easy to add custom easing functions and presets
//! - **GPUI Integration**: Seamless integration with GPUI's animation system
//!

use gpui::*;
use std::time::Duration;

/// Standard animation durations following modern UI guidelines
pub mod durations {
    use std::time::Duration;

    /// Ultra fast (100ms) - for micro-interactions
    pub const ULTRA_FAST: Duration = Duration::from_millis(100);

    /// Very fast (150ms) - for subtle state changes
    pub const FASTEST: Duration = Duration::from_millis(150);

    /// Fast (200ms) - for quick transitions
    pub const FAST: Duration = Duration::from_millis(200);

    /// Normal (300ms) - default for most animations
    pub const NORMAL: Duration = Duration::from_millis(300);

    /// Slow (400ms) - for emphasis
    pub const SLOW: Duration = Duration::from_millis(400);

    /// Very slow (500ms) - for dramatic effects
    pub const SLOWEST: Duration = Duration::from_millis(500);

    /// Extra slow (600ms) - for very dramatic effects
    pub const EXTRA_SLOW: Duration = Duration::from_millis(600);
}

/// Professional easing functions for smooth animations
/// Based on CSS cubic-bezier curves and spring physics
pub mod easings {
    /// Linear easing (no acceleration)
    pub fn linear(t: f32) -> f32 {
        t
    }

    /// Ease in quad - starts slow, accelerates
    pub fn ease_in_quad(t: f32) -> f32 {
        t * t
    }

    /// Ease out quad - starts fast, decelerates
    pub fn ease_out_quad(t: f32) -> f32 {
        t * (2.0 - t)
    }

    /// Ease in-out quad - smooth acceleration and deceleration
    pub fn ease_in_out_quad(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    /// Ease in cubic - stronger acceleration
    pub fn ease_in_cubic(t: f32) -> f32 {
        t * t * t
    }

    /// Ease out cubic - stronger deceleration (most natural feeling)
    pub fn ease_out_cubic(t: f32) -> f32 {
        let t = t - 1.0;
        t * t * t + 1.0
    }

    /// Ease in-out cubic - smooth and professional (recommended default)
    pub fn ease_in_out_cubic(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let t = 2.0 * t - 2.0;
            1.0 + t * t * t / 2.0
        }
    }

    /// Ease in quart - very strong acceleration
    pub fn ease_in_quart(t: f32) -> f32 {
        t * t * t * t
    }

    /// Ease out quart - very smooth deceleration
    pub fn ease_out_quart(t: f32) -> f32 {
        let t = t - 1.0;
        1.0 - t * t * t * t
    }

    /// Ease in-out quart - very smooth both ways
    pub fn ease_in_out_quart(t: f32) -> f32 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            let t = t - 1.0;
            1.0 - 8.0 * t * t * t * t
        }
    }

    /// Ease out expo - dramatic deceleration
    pub fn ease_out_expo(t: f32) -> f32 {
        if t >= 1.0 {
            1.0
        } else {
            1.0 - 2_f32.powf(-10.0 * t)
        }
    }

    /// Ease in-out expo - dramatic both ways
    pub fn ease_in_out_expo(t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t >= 1.0 {
            1.0
        } else if t < 0.5 {
            2_f32.powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - 2_f32.powf(-20.0 * t + 10.0)) / 2.0
        }
    }

    /// Spring easing - natural bouncy effect
    pub fn spring(t: f32) -> f32 {
        if t >= 1.0 {
            return 1.0;
        }
        let damping = 0.7;
        let frequency = 1.5;
        let decay = (-damping * t * 10.0).exp();
        let oscillation = (frequency * t * std::f32::consts::PI * 2.0).sin();
        let result = 1.0 - decay * oscillation * 0.5; // Reduced amplitude
        result.clamp(0.0, 1.0)
    }

    /// Elastic easing - more pronounced spring effect
    pub fn elastic(t: f32) -> f32 {
        if t == 0.0 {
            return 0.0;
        }
        if t >= 1.0 {
            return 1.0;
        }
        let p = 0.3;
        let s = p / 4.0;
        let t_adj = t - 1.0;
        let result = 1.0 + (2_f32.powf(10.0 * t_adj)) * ((t_adj - s) * (2.0 * std::f32::consts::PI) / p).sin();
        result.clamp(0.0, 1.0)
    }

    /// Smooth spring - subtle spring effect (recommended for UI)
    pub fn smooth_spring(t: f32) -> f32 {
        if t >= 1.0 {
            return 1.0;
        }
        let damping = 0.9; // Increased damping for smoother effect
        let frequency = 1.0;
        let decay = (-damping * t * 10.0).exp();
        let oscillation = (frequency * t * std::f32::consts::PI * 2.0).sin();
        let result = t + decay * oscillation * 0.1; // Very subtle spring
        result.clamp(0.0, 1.0)
    }

    /// Back easing - slight overshoot for emphasis
    /// Note: Clamped to prevent values outside 0-1 range
    pub fn ease_out_back(t: f32) -> f32 {
        if t >= 1.0 {
            return 1.0;
        }
        // Use a smaller constant to reduce overshoot
        let c1 = 1.2; // Reduced from 1.70158 to stay within bounds
        let c3 = c1 + 1.0;
        let t_adj = t - 1.0;
        let result = 1.0 + c3 * t_adj * t_adj * t_adj + c1 * t_adj * t_adj;
        result.clamp(0.0, 1.0)
    }

    /// Smooth ease (similar to ease-in-out-cubic) - best for most UI
    pub fn smooth() -> impl Fn(f32) -> f32 {
        ease_in_out_cubic
    }

    /// Snappy ease - quick with slight overshoot
    pub fn snappy() -> impl Fn(f32) -> f32 {
        ease_out_back
    }
}

/// Creates a smooth fade-in animation
///
/// Uses cubic easing for the most natural fade effect
pub fn fade_in(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_cubic)
}

/// Creates a smooth fade-out animation
pub fn fade_out(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_in_cubic)
}

/// Creates a smooth slide animation
///
/// Best for sliding panels, drawers, and menus
pub fn slide_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_cubic)
}

/// Creates a spring-based slide animation
///
/// Natural feeling slide with subtle bounce
pub fn spring_slide(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::smooth_spring)
}

/// Creates a scale animation with back easing
///
/// Scales with a slight overshoot for emphasis
pub fn scale_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_back)
}

/// Creates a smooth scale animation without overshoot
pub fn scale_smooth(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_cubic)
}

/// Creates a rotation animation
pub fn rotate_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::linear)
}

/// Creates a smooth, professional pulse animation
///
/// Uses sine wave for natural breathing effect
pub fn pulse_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::linear)
}

/// Creates a shake animation (horizontal movement)
///
/// Uses elastic easing for realistic shake
pub fn shake_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_quad)
}

/// Creates a bounce animation with spring physics
pub fn bounce_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::spring)
}

/// Creates a smooth bounce without overshoot
pub fn bounce_smooth(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_quart)
}

/// Creates an elastic spring animation
pub fn spring_animation(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::smooth_spring)
}

/// Pre-configured animation presets with optimal settings
pub mod presets {
    use super::*;

    // Fade animations
    /// Ultra-quick fade in (100ms) - for tooltips
    pub fn fade_in_ultra_quick() -> Animation {
        fade_in(durations::ULTRA_FAST)
    }

    /// Quick fade in (200ms) - for fast transitions
    pub fn fade_in_quick() -> Animation {
        fade_in(durations::FAST)
    }

    /// Normal fade in (300ms) - standard UI transition
    pub fn fade_in_normal() -> Animation {
        fade_in(durations::NORMAL)
    }

    /// Slow fade in (400ms) - for emphasis
    pub fn fade_in_slow() -> Animation {
        fade_in(durations::SLOW)
    }

    /// Quick fade out (200ms)
    pub fn fade_out_quick() -> Animation {
        fade_out(durations::FAST)
    }

    /// Normal fade out (300ms)
    pub fn fade_out_normal() -> Animation {
        fade_out(durations::NORMAL)
    }

    // Slide animations with improved easing
    /// Slide in from top with smooth easing
    pub fn slide_in_top() -> Animation {
        slide_animation(durations::NORMAL)
    }

    /// Slide in from bottom with smooth easing
    pub fn slide_in_bottom() -> Animation {
        slide_animation(durations::NORMAL)
    }

    /// Slide in from left with smooth easing
    pub fn slide_in_left() -> Animation {
        slide_animation(durations::NORMAL)
    }

    /// Slide in from right with smooth easing
    pub fn slide_in_right() -> Animation {
        slide_animation(durations::NORMAL)
    }

    /// Spring slide from left - natural feeling
    pub fn spring_slide_left() -> Animation {
        spring_slide(durations::SLOW)
    }

    /// Spring slide from right - natural feeling
    pub fn spring_slide_right() -> Animation {
        spring_slide(durations::SLOW)
    }

    // Scale animations
    /// Scale up with back easing (slight overshoot)
    pub fn scale_up() -> Animation {
        scale_animation(durations::FAST)
    }

    /// Scale down with back easing
    pub fn scale_down() -> Animation {
        scale_animation(durations::FAST)
    }

    /// Smooth scale up (no overshoot)
    pub fn scale_up_smooth() -> Animation {
        scale_smooth(durations::FAST)
    }

    /// Smooth scale down (no overshoot)
    pub fn scale_down_smooth() -> Animation {
        scale_smooth(durations::FAST)
    }

    // Rotation animations
    /// Continuous spin (2 seconds per rotation)
    pub fn spin() -> Animation {
        rotate_animation(Duration::from_secs(2)).repeat()
    }

    /// Fast spin (1 second per rotation)
    pub fn spin_fast() -> Animation {
        rotate_animation(Duration::from_secs(1)).repeat()
    }

    /// Slow spin (3 seconds per rotation) - for loading indicators
    pub fn spin_slow() -> Animation {
        rotate_animation(Duration::from_secs(3)).repeat()
    }

    // Pulse animations - improved smoothness
    /// Smooth pulse effect (1 second cycle)
    pub fn pulse() -> Animation {
        pulse_animation(Duration::from_secs(1)).repeat()
    }

    /// Fast pulse (600ms cycle) - for urgent notifications
    pub fn pulse_fast() -> Animation {
        pulse_animation(durations::EXTRA_SLOW).repeat()
    }

    /// Slow pulse (1.5 second cycle) - for subtle breathing effect
    pub fn pulse_slow() -> Animation {
        pulse_animation(Duration::from_millis(1500)).repeat()
    }

    // Interactive animations
    /// Shake effect (error indication)
    pub fn shake() -> Animation {
        shake_animation(durations::FAST)
    }

    /// Strong shake (critical error)
    pub fn shake_strong() -> Animation {
        shake_animation(durations::NORMAL)
    }

    // Bounce animations
    /// Bounce in effect with spring physics
    pub fn bounce_in() -> Animation {
        bounce_animation(durations::SLOW)
    }

    /// Smooth bounce (no overshoot)
    pub fn bounce_smooth_preset() -> Animation {
        bounce_smooth(durations::NORMAL)
    }

    // Spring animations
    /// Spring effect (natural feeling)
    pub fn spring() -> Animation {
        spring_animation(durations::SLOW)
    }

    /// Quick spring
    pub fn spring_quick() -> Animation {
        spring_animation(durations::NORMAL)
    }
}

/// Animation state management helper
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationState {
    /// Animation hasn't started
    Idle,
    /// Animation is running
    Running,
    /// Animation completed
    Complete,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::Idle
    }
}

impl AnimationState {
    /// Check if animation is idle
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Helper functions for common animation patterns

/// Calculate smooth pulse scale (sine wave based)
///
/// Returns a scale factor that oscillates smoothly
pub fn pulse_scale(delta: f32, min_scale: f32, max_scale: f32) -> f32 {
    let oscillation = (delta * std::f32::consts::PI * 2.0).sin();
    let normalized = (oscillation + 1.0) / 2.0; // Convert -1..1 to 0..1
    min_scale + (max_scale - min_scale) * normalized
}

/// Calculate smooth pulse opacity
///
/// Returns an opacity value that oscillates smoothly
pub fn pulse_opacity(delta: f32, min_opacity: f32, max_opacity: f32) -> f32 {
    let oscillation = (delta * std::f32::consts::PI * 2.0).sin();
    let normalized = (oscillation + 1.0) / 2.0;
    min_opacity + (max_opacity - min_opacity) * normalized
}

/// Calculate shake offset with natural decay
pub fn shake_offset(delta: f32, max_offset: f32) -> f32 {
    let frequency = 4.0;
    let decay = 1.0 - delta;
    (delta * std::f32::consts::PI * frequency).sin() * max_offset * decay
}

/// Calculate spring bounce with natural physics
pub fn spring_bounce(delta: f32, amplitude: f32) -> f32 {
    let damping = 0.7;
    let frequency = 1.5;
    let decay = (-damping * delta * 10.0).exp();
    let oscillation = (frequency * delta * std::f32::consts::PI * 2.0).sin();
    amplitude * decay * oscillation
}
