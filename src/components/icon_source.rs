//! Shared IconSource type for Icon, IconButton, and Button components.

use gpui::SharedString;

/// Enum representing the source of an icon - either a named icon or a file path.
#[derive(Debug, Clone, PartialEq)]
pub enum IconSource {
    /// A named icon that will be resolved using the configured icon base path.
    /// Example: "search" → "assets/icons/search.svg"
    Named(String),

    /// A direct file path to an icon.
    /// Example: "custom/icons/logo.svg"
    FilePath(SharedString),
}

impl From<&str> for IconSource {
    fn from(s: &str) -> Self {
        // Check for path separators FIRST (more reliable than .svg check)
        if s.contains('/') || s.contains('\\') {
            IconSource::FilePath(SharedString::from(s.to_string()))
        } else if s.trim_end().to_lowercase().ends_with(".svg") {
            // If it ends with .svg but has no path separator, treat as named icon
            // This handles cases like "my-icon.svg" which should be resolved
            IconSource::Named(s.trim_end_matches(".svg").to_string())
        } else {
            IconSource::Named(s.to_string())
        }
    }
}

impl From<String> for IconSource {
    fn from(s: String) -> Self {
        if s.contains('/') || s.contains('\\') {
            IconSource::FilePath(s.into())
        } else if s.trim_end().to_lowercase().ends_with(".svg") {
            IconSource::Named(s.trim_end_matches(".svg").to_string())
        } else {
            IconSource::Named(s)
        }
    }
}

impl From<SharedString> for IconSource {
    fn from(s: SharedString) -> Self {
        let s_str = s.to_string();
        if s_str.contains('/') || s_str.contains('\\') {
            IconSource::FilePath(s)
        } else if s_str.trim_end().to_lowercase().ends_with(".svg") {
            IconSource::Named(s_str.trim_end_matches(".svg").to_string())
        } else {
            IconSource::Named(s_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_source_detection() {
        // Named icons
        match IconSource::from("search") {
            IconSource::Named(name) => assert_eq!(name, "search"),
            _ => panic!("Should be Named"),
        }

        // File paths
        match IconSource::from("custom/icon.svg") {
            IconSource::FilePath(_) => {},
            _ => panic!("Should be FilePath"),
        }

        // Edge case: .svg without path separator (treat as named)
        match IconSource::from("my-icon.svg") {
            IconSource::Named(name) => assert_eq!(name, "my-icon"),
            _ => panic!("Should be Named"),
        }

        // Windows paths
        match IconSource::from("custom\\icon.svg") {
            IconSource::FilePath(_) => {},
            _ => panic!("Should be FilePath"),
        }
    }
}
