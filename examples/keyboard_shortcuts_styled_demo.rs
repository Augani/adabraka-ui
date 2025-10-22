use adabraka_ui::{
    prelude::*,
    components::{
        keyboard_shortcuts::{KeyboardShortcuts, ShortcutItem},
        scrollable::scrollable_vertical,
    },
};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("KeyboardShortcuts Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| KeyboardShortcutsStyledDemo::new()),
            )
            .unwrap();
        });
}

struct KeyboardShortcutsStyledDemo {}

impl KeyboardShortcutsStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for KeyboardShortcutsStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(24.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("KeyboardShortcuts Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait")
                                )
                        )
                        // 1. Default KeyboardShortcuts
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default KeyboardShortcuts (No Custom Styling)")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("File", vec![
                                                ShortcutItem::new("New File", "cmd-n"),
                                                ShortcutItem::new("Open File", "cmd-o"),
                                                ShortcutItem::new("Save", "cmd-s"),
                                            ])
                                            .category("Edit", vec![
                                                ShortcutItem::new("Copy", "cmd-c"),
                                                ShortcutItem::new("Paste", "cmd-v"),
                                                ShortcutItem::new("Undo", "cmd-z"),
                                            ])
                                    })
                                )
                        )
                        // 2. Custom Padding
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Padding (via Styled trait)")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("File", vec![
                                                ShortcutItem::new("New File", "cmd-n"),
                                                ShortcutItem::new("Open File", "cmd-o"),
                                            ])
                                            .p(px(24.0))  // Custom padding
                                            .bg(theme.tokens.muted.opacity(0.3))
                                            .rounded(px(12.0))
                                    })
                                )
                        )
                        // 3. Custom Background & Border
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Custom Background & Border")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("Navigation", vec![
                                                ShortcutItem::new("Go to Line", "cmd-l"),
                                                ShortcutItem::new("Go to File", "cmd-p"),
                                                ShortcutItem::new("Go to Symbol", "cmd-shift-o"),
                                            ])
                                            .p(px(20.0))
                                            .bg(hsla(217.0 / 360.0, 0.91, 0.17, 1.0))  // Custom dark blue background
                                            .border_2()
                                            .border_color(rgb(0x3b82f6))  // Blue border
                                            .rounded(px(16.0))
                                    })
                                )
                        )
                        // 4. Custom Width
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Width Control")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("View", vec![
                                                ShortcutItem::new("Zoom In", "cmd-plus"),
                                                ShortcutItem::new("Zoom Out", "cmd-minus"),
                                            ])
                                            .w(px(500.0))  // Fixed width
                                            .p(px(16.0))
                                            .bg(theme.tokens.accent.opacity(0.1))
                                            .rounded(px(8.0))
                                    })
                                )
                        )
                        // 5. Shadow Effects
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Shadow Effects")
                                )
                                .child(
                                    HStack::new()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("Small Shadow")
                                                )
                                                .child(
                                                    cx.new(|_| {
                                                        KeyboardShortcuts::new()
                                                            .category("Terminal", vec![
                                                                ShortcutItem::new("New Terminal", "ctrl-backtick"),
                                                            ])
                                                            .p(px(16.0))
                                                            .bg(theme.tokens.card)
                                                            .rounded(px(8.0))
                                                            .shadow_sm()  // Small shadow
                                                    })
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("Large Shadow")
                                                )
                                                .child(
                                                    cx.new(|_| {
                                                        KeyboardShortcuts::new()
                                                            .category("Search", vec![
                                                                ShortcutItem::new("Find", "cmd-f"),
                                                            ])
                                                            .p(px(16.0))
                                                            .bg(theme.tokens.card)
                                                            .rounded(px(8.0))
                                                            .shadow_lg()  // Large shadow
                                                    })
                                                )
                                        )
                                )
                        )
                        // 6. Complex Combined Styling
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Complex Combined Styling")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("Debug", vec![
                                                ShortcutItem::new("Start Debugging", "f5"),
                                                ShortcutItem::new("Step Over", "f10"),
                                                ShortcutItem::new("Step Into", "f11"),
                                                ShortcutItem::new("Continue", "f5"),
                                            ])
                                            .category("Refactor", vec![
                                                ShortcutItem::new("Rename Symbol", "f2"),
                                                ShortcutItem::new("Extract Function", "cmd-shift-r"),
                                            ])
                                            .p(px(32.0))  // Large padding
                                            .bg(hsla(271.0 / 360.0, 0.76, 0.53, 0.1))  // Purple background with opacity
                                            .border_2()
                                            .border_color(rgb(0x7c3aed))  // Purple border
                                            .rounded(px(20.0))  // Large border radius
                                            .shadow_lg()  // Large shadow
                                            .w_full()  // Full width
                                    })
                                )
                        )
                        // 7. Margin Control
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("7. Margin Control")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("Window", vec![
                                                ShortcutItem::new("Close Window", "cmd-w"),
                                                ShortcutItem::new("Minimize", "cmd-m"),
                                            ])
                                            .p(px(16.0))
                                            .my(px(32.0))  // Vertical margin
                                            .bg(hsla(158.0 / 360.0, 0.64, 0.52, 0.1))  // Green background with opacity
                                            .border_1()
                                            .border_color(rgb(0x10b981))
                                            .rounded(px(12.0))
                                    })
                                )
                        )
                        // 8. Ultra Custom Card
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("8. Ultra Custom Card Style")
                                )
                                .child(
                                    cx.new(|_| {
                                        KeyboardShortcuts::new()
                                            .category("Git", vec![
                                                ShortcutItem::new("Commit", "cmd-enter"),
                                                ShortcutItem::new("Push", "cmd-shift-p"),
                                                ShortcutItem::new("Pull", "cmd-shift-l"),
                                                ShortcutItem::new("Fetch", "cmd-shift-f"),
                                            ])
                                            .category("GitHub", vec![
                                                ShortcutItem::new("View PR", "cmd-shift-g"),
                                                ShortcutItem::new("Create PR", "cmd-shift-c"),
                                            ])
                                            .px(px(40.0))
                                            .py(px(30.0))
                                            .bg(hsla(38.0 / 360.0, 0.92, 0.50, 0.1))  // Orange background with opacity
                                            .border_2()
                                            .border_color(rgb(0xf59e0b))
                                            .rounded(px(16.0))
                                            .shadow_md()
                                            .w(px(700.0))
                                    })
                                )
                        )
                        // Info Box
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(16.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("All customizations above use the Styled trait for full GPUI styling control!")
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("Methods used: .p(), .px(), .py(), .my(), .bg(), .border_1/2(), .border_color(), .rounded(), .w(), .w_full(), .shadow_sm/md/lg()")
                                )
                        )
                )
            )
    }
}
