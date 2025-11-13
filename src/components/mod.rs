//! UI components module.

pub mod button;
pub mod icon_source;
pub mod icon_button;
pub mod icon;
pub mod text;

// Re-export commonly used types
pub use icon_source::IconSource;
pub use icon::{IconSize, IconVariant};
pub mod text_field;
pub mod checkbox;
pub mod toggle;
pub mod toggle_group;
pub mod select;
pub mod tooltip;
pub mod scrollbar;
pub mod scrollable;
pub mod confirm_dialog;
pub mod input;
pub mod input_state;
pub mod textarea;
pub mod resizable;
pub mod drag_drop;
pub mod editor;
pub mod progress;
pub mod search_input;
pub mod keyboard_shortcuts;
pub mod separator;
pub mod label;
pub mod skeleton;
pub mod radio;
pub mod slider;
pub use slider::SliderAxis;
pub mod avatar;
pub mod pagination;
pub mod collapsible;
pub mod calendar;
pub mod date_picker;
pub mod navigation_menu;
pub mod combobox;
pub mod color_picker;

pub use crate::display::badge;


