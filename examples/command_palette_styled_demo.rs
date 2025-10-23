use adabraka_ui::{
    prelude::*,
    overlays::command_palette::{Command, CommandPalette},
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
                        title: Some("Command Palette Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| CommandPaletteStyledDemo::new()),
            )
            .unwrap();
        });
}

struct CommandPaletteStyledDemo {
    show_default: bool,
    show_custom_width: bool,
    show_custom_border: bool,
    show_custom_bg: bool,
    show_custom_shadow: bool,
    show_combined: bool,
    message: SharedString,
}

impl CommandPaletteStyledDemo {
    fn new() -> Self {
        Self {
            show_default: false,
            show_custom_width: false,
            show_custom_border: false,
            show_custom_bg: false,
            show_custom_shadow: false,
            show_combined: false,
            message: "Click a button to open a styled command palette".into(),
        }
    }

    fn create_sample_commands(&self, _cx: &Context<Self>) -> Vec<Command> {
        vec![
            Command::new("file-new", "New File")
                .description("Create a new file")
                .icon("file-plus")
                .shortcut("Cmd+N")
                .on_select(|_window, _cx| {}),
            Command::new("file-open", "Open File")
                .description("Open an existing file")
                .icon("folder-open")
                .shortcut("Cmd+O")
                .on_select(|_window, _cx| {}),
            Command::new("file-save", "Save File")
                .description("Save the current file")
                .icon("save")
                .shortcut("Cmd+S")
                .on_select(|_window, _cx| {}),
            Command::new("edit-copy", "Copy")
                .description("Copy selected text")
                .icon("copy")
                .shortcut("Cmd+C")
                .on_select(|_window, _cx| {}),
            Command::new("edit-paste", "Paste")
                .description("Paste from clipboard")
                .icon("clipboard")
                .shortcut("Cmd+V")
                .on_select(|_window, _cx| {}),
            Command::new("edit-find", "Find")
                .description("Search in current file")
                .icon("search")
                .shortcut("Cmd+F")
                .on_select(|_window, _cx| {}),
            Command::new("view-toggle", "Toggle Sidebar")
                .description("Show or hide the sidebar")
                .icon("sidebar")
                .shortcut("Cmd+B")
                .on_select(|_window, _cx| {}),
            Command::new("help-docs", "Open Documentation")
                .description("View the documentation")
                .icon("book-open")
                .shortcut("F1")
                .on_select(|_window, _cx| {}),
        ]
    }
}

impl Render for CommandPaletteStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .child(
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
                                    .child("Command Palette Styled Trait Customization Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Demonstrating full GPUI styling control via Styled trait")
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.accent)
                                    .child(self.message.clone())
                            )
                    )
                    // 1. Default Command Palette
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("1. Default Command Palette")
                            )
                            .child(
                                Button::new("show-default", "Open Default Command Palette")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_default = true;
                                        view.message = "Default command palette opened".into();
                                        cx.notify();
                                    }))
                            )
                    )
                    // 2. Custom Width
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("2. Custom Width (via Styled trait)")
                            )
                            .child(
                                Button::new("show-custom-width", "Open Wide Command Palette (800px)")
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_width = true;
                                        view.message = "Wide command palette opened (800px)".into();
                                        cx.notify();
                                    }))
                            )
                    )
                    // 3. Custom Border
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("3. Custom Border")
                            )
                            .child(
                                Button::new("show-custom-border", "Open with Custom Border")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_border = true;
                                        view.message = "Command palette with custom border opened".into();
                                        cx.notify();
                                    }))
                            )
                    )
                    // 4. Custom Background
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("4. Custom Background Color")
                            )
                            .child(
                                Button::new("show-custom-bg", "Open with Custom Background")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_bg = true;
                                        view.message = "Command palette with custom background opened".into();
                                        cx.notify();
                                    }))
                            )
                    )
                    // 5. Custom Shadow
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("5. Custom Shadow")
                            )
                            .child(
                                Button::new("show-custom-shadow", "Open with Extra Large Shadow")
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_shadow = true;
                                        view.message = "Command palette with extra large shadow opened".into();
                                        cx.notify();
                                    }))
                            )
                    )
                    // 6. Combined Styling
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("6. Combined Styling (Multiple Styled Trait Methods)")
                            )
                            .child(
                                Button::new("show-combined", "Open Ultra Custom Command Palette")
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_combined = true;
                                        view.message = "Ultra custom command palette opened".into();
                                        cx.notify();
                                    }))
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
                                    .child("Methods used: .w(), .border_2/3(), .border_color(), .bg(), .shadow_xl(), .rounded(), and more!")
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.accent_foreground)
                                    .child("Press Esc or click outside to close any command palette.")
                            )
                    )
            )
            // Render command palettes
            .children(if self.show_default {
                let commands = self.create_sample_commands(cx);
                let entity = cx.entity().clone();
                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_default = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
            .children(if self.show_custom_width {
                let commands = self.create_sample_commands(cx);
                let entity = cx.entity().clone();
                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .w(px(800.0))  // Custom width
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_custom_width = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
            .children(if self.show_custom_border {
                let commands = self.create_sample_commands(cx);
                let entity = cx.entity().clone();
                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .border_3()  // Thicker border
                        .border_color(rgb(0x3b82f6))  // Blue border
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_custom_border = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
            .children(if self.show_custom_bg {
                let commands = self.create_sample_commands(cx);
                let entity = cx.entity().clone();
                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .bg(rgb(0x1e293b))  // Dark slate background
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_custom_bg = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
            .children(if self.show_custom_shadow {
                let commands = self.create_sample_commands(cx);
                let entity = cx.entity().clone();
                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .shadow_xl()  // Extra large shadow
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_custom_shadow = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
            .children(if self.show_combined {
                let commands = self.create_sample_commands(cx);
                let entity = cx.entity().clone();
                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .w(px(750.0))  // Custom width
                        .border_3()  // Thick border
                        .border_color(rgb(0x8b5cf6))  // Purple border
                        .bg(rgb(0x1e1b4b))  // Deep indigo background
                        .rounded(px(24.0))  // Large border radius
                        .shadow_xl()  // Extra large shadow
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_combined = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
    }
}
