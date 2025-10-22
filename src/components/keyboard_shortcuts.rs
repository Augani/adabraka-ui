//! Keyboard shortcuts manager component with categorized shortcuts and platform-specific key display.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

#[derive(Clone, Debug)]
pub struct ShortcutItem {
    pub description: SharedString,
    pub keys: SharedString,
    pub icon: Option<SharedString>,
}

impl ShortcutItem {
    pub fn new(description: impl Into<SharedString>, keys: impl Into<SharedString>) -> Self {
        Self {
            description: description.into(),
            keys: keys.into(),
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn formatted_keys(&self) -> String {
        let keys = self.keys.to_string();

        #[cfg(target_os = "macos")]
        {
            keys.replace("cmd", "⌘")
                .replace("shift", "⇧")
                .replace("alt", "⌥")
                .replace("ctrl", "⌃")
                .replace("-", " ")
        }

        #[cfg(not(target_os = "macos"))]
        {
            keys.replace("cmd", "Ctrl")
                .replace("shift", "Shift")
                .replace("alt", "Alt")
                .replace("ctrl", "Ctrl")
                .replace("-", "+")
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShortcutCategory {
    pub name: SharedString,
    pub shortcuts: Vec<ShortcutItem>,
    pub expanded: bool,
}

impl ShortcutCategory {
    pub fn new(name: impl Into<SharedString>, shortcuts: Vec<ShortcutItem>) -> Self {
        Self {
            name: name.into(),
            shortcuts,
            expanded: true,
        }
    }
}

pub struct KeyboardShortcuts {
    categories: Vec<ShortcutCategory>,
    show_icons: bool,
    style: StyleRefinement,
}

impl KeyboardShortcuts {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            show_icons: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn category(mut self, name: impl Into<SharedString>, shortcuts: Vec<ShortcutItem>) -> Self {
        self.categories.push(ShortcutCategory::new(name, shortcuts));
        self
    }

    pub fn add_category(mut self, category: ShortcutCategory) -> Self {
        self.categories.push(category);
        self
    }

    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for KeyboardShortcuts {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for KeyboardShortcuts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style.clone();

        div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .children(self.categories.iter().map(|category| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .child(category.name.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .children(category.shortcuts.iter().map(|shortcut| {
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .rounded(theme.tokens.radius_sm)
                                    .hover(|style| style.bg(theme.tokens.muted.opacity(0.5)))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.foreground)
                                                    .child(shortcut.description.clone())
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(
                                                div()
                                                    .px(px(8.0))
                                                    .py(px(4.0))
                                                    .rounded(theme.tokens.radius_sm)
                                                    .bg(theme.tokens.muted)
                                                    .border_1()
                                                    .border_color(theme.tokens.border)
                                                    .text_size(px(12.0))
                                                    .font_family("monospace")
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child(shortcut.formatted_keys())
                                            )
                                    )
                                    .into_any_element()
                            }))
                    )
                    .into_any_element()
            }))
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
