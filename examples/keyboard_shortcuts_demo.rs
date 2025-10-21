use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    components::keyboard_shortcuts::{KeyboardShortcuts, ShortcutItem, ShortcutCategory},
    layout::VStack,
    theme::use_theme,
};

actions!(keyboard_shortcuts_demo, [Quit]);

struct KeyboardShortcutsDemo;

impl KeyboardShortcutsDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for KeyboardShortcutsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .on_action(cx.listener(|_this, _: &Quit, _window, cx| {
                cx.quit();
            }))
            .child(
                VStack::new()
                    .p(px(32.0))
                    .gap(px(32.0))
                    .size_full()
                    // Header
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Keyboard Shortcuts Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Display and organize keyboard shortcuts by category")
                            )
                    )
                    // Instructions
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.muted.opacity(0.3))
                            .rounded(theme.tokens.radius_md)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Features")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Organized by category (File, Edit, View, etc.)")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Platform-specific key display (⌘ on Mac, Ctrl on others)")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Clean, readable layout with hover effects")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Monospace font for key bindings")
                            )
                    )
                    // Keyboard shortcuts display
                    .child(
                        div()
                            .flex_1()
                            .p(px(24.0))
                            .bg(theme.tokens.card)
                            .rounded(theme.tokens.radius_md)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .child(
                                cx.new(|_cx| {
                                    KeyboardShortcuts::new()
                                        .category("File", vec![
                                            ShortcutItem::new("New File", "cmd-n"),
                                            ShortcutItem::new("Open File", "cmd-o"),
                                            ShortcutItem::new("Save", "cmd-s"),
                                            ShortcutItem::new("Save As", "cmd-shift-s"),
                                            ShortcutItem::new("Close File", "cmd-w"),
                                            ShortcutItem::new("Quit", "cmd-q"),
                                        ])
                                        .category("Edit", vec![
                                            ShortcutItem::new("Undo", "cmd-z"),
                                            ShortcutItem::new("Redo", "cmd-shift-z"),
                                            ShortcutItem::new("Cut", "cmd-x"),
                                            ShortcutItem::new("Copy", "cmd-c"),
                                            ShortcutItem::new("Paste", "cmd-v"),
                                            ShortcutItem::new("Select All", "cmd-a"),
                                            ShortcutItem::new("Find", "cmd-f"),
                                            ShortcutItem::new("Replace", "cmd-shift-f"),
                                        ])
                                        .category("View", vec![
                                            ShortcutItem::new("Toggle Sidebar", "cmd-b"),
                                            ShortcutItem::new("Toggle Status Bar", "cmd-shift-b"),
                                            ShortcutItem::new("Zoom In", "cmd-="),
                                            ShortcutItem::new("Zoom Out", "cmd--"),
                                            ShortcutItem::new("Reset Zoom", "cmd-0"),
                                            ShortcutItem::new("Full Screen", "cmd-ctrl-f"),
                                        ])
                                        .category("Window", vec![
                                            ShortcutItem::new("Minimize", "cmd-m"),
                                            ShortcutItem::new("New Window", "cmd-shift-n"),
                                            ShortcutItem::new("Close Window", "cmd-shift-w"),
                                            ShortcutItem::new("Next Tab", "cmd-shift-]"),
                                            ShortcutItem::new("Previous Tab", "cmd-shift-["),
                                        ])
                                        .category("Help", vec![
                                            ShortcutItem::new("Documentation", "f1"),
                                            ShortcutItem::new("Show All Commands", "cmd-shift-p"),
                                            ShortcutItem::new("Keyboard Shortcuts", "cmd-k cmd-s"),
                                        ])
                                })
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(move |cx: &mut App| {
        // Install dark theme
        adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());

        // Initialize UI system
        adabraka_ui::init(cx);

        // Set up actions
        cx.on_action(|_: &Quit, cx| cx.quit());

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
        ]);
        cx.activate(true);

        // Create window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Keyboard Shortcuts Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| KeyboardShortcutsDemo::new(cx)),
        )
        .unwrap();
    });
}
