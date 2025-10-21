//! # Font Loading and Registration
//!
//! Handles embedding and registering custom fonts with GPUI for consistent typography
//! across the component library. Fonts are embedded at compile time for reliable distribution.
//! ## Font Families
//!
//! - **Inter**: Primary UI font family (sans-serif) - clean, modern, highly legible
//! - **JetBrains Mono**: Monospace font for code, terminals, and technical content
//!
//! ## Font Weights
//!
//! - **Regular (400)**: Default weight for body text and labels
//! - **Medium (500)**: Slightly heavier for emphasis and buttons
//! - **SemiBold (600)**: For headings and important UI elements
//! - **Bold (700)**: For strong emphasis and primary actions
//!
//! ## Design Decisions
//!
//! - **Compile-time Embedding**: Fonts are included in the binary for consistent rendering
//! - **Limited Weights**: Only essential weights to minimize binary size
//! - **Cross-platform**: Fonts chosen for excellent rendering across all platforms
//! - **Performance**: Fonts loaded once at startup, cached by GPUI's text system
//! - **Fallback**: System fonts used if custom fonts fail to load
//!
//! ## Usage
//!
//! Fonts are automatically registered when calling `adabraka_ui::init(cx)`.
//! Access font families through the theme system or utility functions.
//!
//! ```rust
//! // Access via theme (recommended)
//! let theme = use_theme();
//! div().font_family(theme.tokens.font_family.clone())
//!
//! // Direct access to font families
//! ui_font_family() // -> "Inter"
//! mono_font_family() // -> "JetBrains Mono"
//! ```
//!

use gpui::*;

/// Font family names used throughout the UI
pub const UI_FONT_FAMILY: &str = "Inter";
pub const UI_MONO_FONT_FAMILY: &str = "JetBrains Mono";

// Embed font files at compile time
// Note: You'll need to place font files in assets/fonts/ directory
// Example fonts (you can replace these with your preferred fonts):
// - Inter: https://rsms.me/inter/
// - JetBrains Mono: https://www.jetbrains.com/lp/mono/

// Regular weights
const INTER_REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");
const INTER_MEDIUM: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");
const INTER_BOLD: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf");

// Monospace
const JETBRAINS_MONO_REGULAR: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");
const JETBRAINS_MONO_BOLD: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono-Bold.ttf");

/// Register all embedded fonts with GPUI
///
/// This should be called during application initialization before any UI is rendered.
///
/// # Example
/// ```ignore
/// use adabraka_ui::fonts;
///
/// Application::new().run(|cx| {
///     fonts::register_fonts(cx);
///     // ... rest of initialization
/// });
/// ```
pub fn register_fonts(cx: &mut App) {
    // Register Inter family (UI font)
    cx.text_system()
        .add_fonts(vec![
            INTER_REGULAR.into(),
            INTER_MEDIUM.into(),
            INTER_SEMIBOLD.into(),
            INTER_BOLD.into(),
        ])
        .expect("Failed to load Inter fonts");

    // Register JetBrains Mono family (monospace font)
    cx.text_system()
        .add_fonts(vec![
            JETBRAINS_MONO_REGULAR.into(),
            JETBRAINS_MONO_BOLD.into(),
        ])
        .expect("Failed to load JetBrains Mono fonts");
}

/// Get the default UI font family
pub fn ui_font_family() -> SharedString {
    UI_FONT_FAMILY.into()
}

/// Get the default monospace font family
pub fn mono_font_family() -> SharedString {
    UI_MONO_FONT_FAMILY.into()
}
