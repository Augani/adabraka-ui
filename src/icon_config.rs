//! Icon configuration for customizing icon asset paths.
//!
//! This module provides global configuration for icon asset paths, allowing
//! users to provide their own icon assets instead of bundling them with the library.

use once_cell::sync::OnceCell;
use std::sync::RwLock;

static ICON_BASE_PATH: OnceCell<RwLock<String>> = OnceCell::new();

/// Sets the base path for icon assets.
///
/// This should be called once at application startup, before any icons are loaded.
/// The path will be used as a prefix when loading named icons.
///
/// # Example
///
/// ```rust
/// use adabraka_ui::set_icon_base_path;
///
/// // Set icons to be loaded from your application's assets directory
/// set_icon_base_path("assets/icons");
///
/// // Now icons will be loaded from assets/icons/{icon-name}.svg
/// // instead of crates/adabraka-ui/assets/icons/{icon-name}.svg
/// ```
///
/// # Arguments
///
/// * `path` - The base path where icon SVG files are located (without trailing slash)
pub fn set_icon_base_path(path: impl Into<String>) {
    let path_string = path.into();
    ICON_BASE_PATH
        .get_or_init(|| RwLock::new(String::new()))
        .write()
        .unwrap()
        .clone_from(&path_string);
}

/// Gets the current icon base path.
///
/// Returns the configured icon base path, or a default path if none has been set.
///
/// # Returns
///
/// The base path for loading icon assets.
pub(crate) fn get_icon_base_path() -> String {
    ICON_BASE_PATH
        .get_or_init(|| RwLock::new("assets/icons".to_string()))
        .read()
        .unwrap()
        .clone()
}

/// Resolves a named icon to its full path.
///
/// This function combines the configured base path with the icon name.
///
/// # Arguments
///
/// * `name` - The icon name (e.g., "arrow-up", "search")
///
/// # Returns
///
/// The full path to the icon SVG file.
///
/// # Example
///
/// ```rust
/// use adabraka_ui::icon_config::resolve_icon_path;
///
/// let path = resolve_icon_path("arrow-up");
/// // Returns "assets/icons/arrow-up.svg" (or your configured path)
/// ```
pub fn resolve_icon_path(name: &str) -> String {
    format!("{}/{}.svg", get_icon_base_path(), name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_icon_path() {
        let path = resolve_icon_path("test-icon");
        assert!(path.contains("test-icon.svg"));
    }

    #[test]
    fn test_custom_icon_path() {
        set_icon_base_path("custom/path/icons");
        let path = resolve_icon_path("custom-icon");
        assert_eq!(path, "custom/path/icons/custom-icon.svg");
    }
}
