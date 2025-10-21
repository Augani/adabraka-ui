
//! # Utility Extensions and Helper Functions
//!
//! Common extension traits and utility functions that enhance GPUI's standard types
//! with convenient methods used throughout the component library.
//! ## Extension Traits
//!
//! - **`AxisExt`**: Convenient methods for checking axis orientation
//! - **`PixelsExt`**: Conversion utilities for Pixels type
//! - **`ScrollHandleOffsetable`**: Trait for scroll handle offset operations
//! - **Color utilities**: Color manipulation and conversion helpers
//! - **Layout helpers**: Common layout calculations and measurements
//!
//! ## Design Decisions
//!
//! - **Minimal Surface Area**: Only essential utilities that are reused across components
//! - **Type Safety**: Extension traits maintain strong typing and prevent errors
//! - **Performance**: Zero-cost abstractions that compile to efficient machine code
//! - **Consistency**: Standardized patterns for common operations
//! - **Discoverability**: Clear naming that makes functionality obvious
//!

use gpui::{Pixels, Point, Size, ScrollHandle};

/// Extension trait for Axis
pub trait AxisExt {
    /// Returns true if the axis is horizontal
    fn is_horizontal(self) -> bool;
    /// Returns true if the axis is vertical
    fn is_vertical(self) -> bool;
}

impl AxisExt for gpui::Axis {
    fn is_horizontal(self) -> bool {
        self == gpui::Axis::Horizontal
    }

    fn is_vertical(self) -> bool {
        self == gpui::Axis::Vertical
    }
}

/// Extension trait for converting Pixels to f32 and f64
pub trait PixelsExt {
    /// Convert to f32
    fn as_f32(&self) -> f32;
    /// Convert to f64
    fn as_f64(self) -> f64;
}

impl PixelsExt for Pixels {
    fn as_f32(&self) -> f32 {
        f32::from(*self)
    }

    fn as_f64(self) -> f64 {
        f64::from(self)
    }
}

/// Trait for types that can be used as scroll handles with offset tracking
pub trait ScrollHandleOffsetable {
    /// Get the current scroll offset
    fn offset(&self) -> Point<Pixels>;
    /// Set the scroll offset
    fn set_offset(&self, offset: Point<Pixels>);
    /// Get the full content size
    fn content_size(&self) -> Size<Pixels>;
}

impl ScrollHandleOffsetable for ScrollHandle {
    fn offset(&self) -> Point<Pixels> {
        self.offset()
    }

    fn set_offset(&self, offset: Point<Pixels>) {
        self.set_offset(offset);
    }

    fn content_size(&self) -> Size<Pixels> {
        self.max_offset() + self.bounds().size
    }
}
