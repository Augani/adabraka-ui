#![allow(missing_docs)]

//! # adabraka-ui: Professional UI Component Library for GPUI
//!
//! A comprehensive, themeable component library inspired by shadcn/ui, designed specifically
//! for building polished desktop applications using GPUI. Provides a complete set of
//! reusable components with consistent styling, smooth animations, and accessibility support.
//! ## Architecture Overview
//!
//! The library is organized into several key modules:
//! - `theme`: Design tokens and theming system with light/dark variants
//! - `components`: Core interactive elements (buttons, inputs, selects, etc.)
//! - `display`: Presentation components (tables, cards, badges, etc.)
//! - `navigation`: Navigation components (sidebars, menus, tabs, etc.)
//! - `overlays`: Modal dialogs, popovers, tooltips, and command palettes
//! - `animations`: Professional animation presets and easing functions
//!
//! ## Key Features
//!
//! - **Theme System**: Comprehensive design tokens with automatic light/dark mode support
//! - **Accessibility**: Full keyboard navigation, ARIA labels, and screen reader support
//! - **Performance**: Optimized rendering with virtual scrolling for large datasets
//! - **Animation**: Smooth, professional animations using spring physics and easing curves
//! - **Type Safety**: Strong typing throughout with compile-time guarantees
//!
//! ## Design Philosophy
//!
//! Components follow shadcn/ui principles with GPUI-specific optimizations:
//! - Composition over inheritance for flexible component APIs
//! - Builder pattern for ergonomic component construction
//! - Entity-based state management for complex interactive components
//! - Consistent naming and styling patterns across all components
//!
//! ## Usage Example
//!
//! ```rust
//! use adabraka_ui::{prelude::*, theme};
//!
//! // Initialize theme and components
//! fn init_app(cx: &mut gpui::App) {
//!     theme::install_theme(cx, theme::Theme::dark());
//!     adabraka_ui::init(cx);
//! }
//!
//! // Use components in your views
//! fn render(cx: &mut gpui::App) -> impl gpui::IntoElement {
//!     div()
//!         .child(Button::new("Click me").on_click(|_, _, _| println!("Clicked!")))
//!         .child(Input::new(&input_state).placeholder("Enter text..."))
//! }
//! ```
//!

extern crate gpui;

pub mod prelude;
pub mod theme;
pub mod layout;
pub mod components;
pub mod navigation;
pub mod display;
pub mod overlays;
pub mod animations;
pub mod transitions;
pub mod virtual_list;

/// Extension traits for common types
pub mod util;

/// Font loading and registration
pub mod fonts;

/// Icon configuration for custom asset paths
pub mod icon_config;

// Re-export commonly used icon configuration functions
pub use icon_config::set_icon_base_path;

/// Initialize the UI library
///
/// This registers all necessary keybindings and initializes component systems.
/// Registers custom fonts for the component library.
pub fn init(cx: &mut gpui::App) {
    fonts::register_fonts(cx);

    components::input::init(cx);
    components::select::init_select(cx);
    components::combobox::init_combobox(cx);
    components::editor::init(cx);
    navigation::sidebar::init_sidebar(cx);
    overlays::popover::init(cx);
}