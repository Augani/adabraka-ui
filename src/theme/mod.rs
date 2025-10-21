//! # Theme System Module
//!
//! Comprehensive theming system inspired by shadcn/ui, providing consistent design tokens
//! and automatic light/dark mode support across all components.
//! ## Architecture
//!
//! ### Design Tokens (`tokens.rs`)
//! - **Colors**: Semantic color palette with light/dark variants
//! - **Typography**: Font families, sizes, and weights
//! - **Spacing**: Consistent spacing scale for layouts
//! - **Border Radius**: Standardized corner radius values
//! - **Shadows**: Elevation system for depth and hierarchy
//!
//! ### Theme Management (`theme.rs`)
//! - **Theme Variants**: Light and dark mode configurations
//! - **Global State**: Thread-safe theme storage and access
//! - **Runtime Switching**: Dynamic theme changes without restart
//!
//! ## Color System
//!
//! Colors follow semantic naming conventions:
//! - `primary` / `primary_foreground`: Main brand colors
//! - `secondary` / `secondary_foreground`: Supporting colors
//! - `muted` / `muted_foreground`: Subtle, less prominent colors
//! - `accent` / `accent_foreground`: Highlight colors for interactions
//! - `destructive`: Error and danger states
//! - `background` / `foreground`: Base canvas colors
//!
//! ## Usage Patterns
//!
//! ```rust
//! // Initialize theme at app startup
//! fn init_app(cx: &mut App) {
//!     theme::install_theme(cx, theme::Theme::dark());
//!     adabraka_ui::init(cx);
//! }
//!
//! // Access theme in components
//! fn render(cx: &mut App) -> impl IntoElement {
//!     let theme = use_theme();
//!
//!     div()
//!         .bg(theme.tokens.primary)
//!         .text_color(theme.tokens.primary_foreground)
//!         .child("Themed content")
//! }
//!
//! // Switch themes dynamically
//! fn toggle_theme(cx: &mut App) {
//!     let current = use_theme();
//!     let new_theme = match current.variant {
//!         ThemeVariant::Light => Theme::dark(),
//!         ThemeVariant::Dark => Theme::light(),
//!     };
//!     install_theme(cx, new_theme);
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **shadcn/ui Compatibility**: Token names and values match shadcn/ui specifications
//! - **Performance**: Global static storage minimizes allocation overhead
//! - **Thread Safety**: Mutex-protected global state for multi-threaded access
//! - **Extensibility**: Easy to add new theme variants or custom tokens
//! - **Consistency**: All components automatically use theme tokens
//!

mod tokens;
mod theme;

pub use tokens::ThemeTokens;
pub use theme::{install_theme, use_theme, Theme, ThemeVariant};


