use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    overlays::command_palette::{CommandPalette, Command, NavigateUp, NavigateDown, SelectCommand, CloseCommand},
    components::button::{Button, ButtonVariant},
    components::icon::IconSource,
    layout::VStack,
    theme::use_theme,
};
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

    fn list(&self, path: &str) -> Result<Vec<gpui::SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(gpui::SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

actions!(command_palette_demo, [Quit, TogglePalette]);

struct CommandPaletteDemo {
    show_palette: bool,
    last_command: String,
    theme_mode: String,
    sidebar_visible: bool,
}

impl CommandPaletteDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            show_palette: false,
            last_command: "None".to_string(),
            theme_mode: "Dark".to_string(),
            sidebar_visible: true,
        }
    }

    fn toggle_palette(&mut self, cx: &mut Context<Self>) {
        self.show_palette = !self.show_palette;
        cx.notify();
    }

    fn execute_command(&mut self, command: &str, cx: &mut Context<Self>) {
        self.last_command = command.to_string();

        // Handle specific commands
        match command {
            "Toggle Sidebar" => {
                self.sidebar_visible = !self.sidebar_visible;
            }
            "Switch to Light Theme" => {
                self.theme_mode = "Light".to_string();
            }
            "Switch to Dark Theme" => {
                self.theme_mode = "Dark".to_string();
            }
            _ => {}
        }

        self.show_palette = false;
        cx.notify();
    }

    fn create_commands(&self, cx: &Context<Self>) -> Vec<Command> {
        let entity = cx.entity().clone();

        vec![
            // File commands
            Command::new("file.new", "New File")
                .icon(IconSource::Named("file-plus".into()))
                .description("Create a new file")
                .category("File")
                .shortcut("Cmd+N")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("New File", cx);
                        });
                    }
                }),
            Command::new("file.open", "Open File")
                .icon(IconSource::Named("folder-open".into()))
                .description("Open an existing file")
                .category("File")
                .shortcut("Cmd+O")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Open File", cx);
                        });
                    }
                }),
            Command::new("file.save", "Save")
                .icon(IconSource::Named("save".into()))
                .description("Save the current file")
                .category("File")
                .shortcut("Cmd+S")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Save", cx);
                        });
                    }
                }),

            // Edit commands
            Command::new("edit.undo", "Undo")
                .icon(IconSource::Named("undo".into()))
                .description("Undo the last action")
                .category("Edit")
                .shortcut("Cmd+Z")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Undo", cx);
                        });
                    }
                }),
            Command::new("edit.redo", "Redo")
                .icon(IconSource::Named("redo".into()))
                .description("Redo the last undone action")
                .category("Edit")
                .shortcut("Cmd+Shift+Z")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Redo", cx);
                        });
                    }
                }),
            Command::new("edit.find", "Find")
                .icon(IconSource::Named("search".into()))
                .description("Find text in the current file")
                .category("Edit")
                .shortcut("Cmd+F")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Find", cx);
                        });
                    }
                }),

            // View commands
            Command::new("view.sidebar", "Toggle Sidebar")
                .icon(IconSource::Named("sidebar".into()))
                .description("Show or hide the sidebar")
                .category("View")
                .shortcut("Cmd+B")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Toggle Sidebar", cx);
                        });
                    }
                }),
            Command::new("view.terminal", "Toggle Terminal")
                .icon(IconSource::Named("terminal".into()))
                .description("Show or hide the terminal")
                .category("View")
                .shortcut("Cmd+`")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Toggle Terminal", cx);
                        });
                    }
                }),

            // Theme commands
            Command::new("theme.light", "Switch to Light Theme")
                .icon(IconSource::Named("sun".into()))
                .description("Use light color theme")
                .category("Theme")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Switch to Light Theme", cx);
                        });
                    }
                }),
            Command::new("theme.dark", "Switch to Dark Theme")
                .icon(IconSource::Named("moon".into()))
                .description("Use dark color theme")
                .category("Theme")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Switch to Dark Theme", cx);
                        });
                    }
                }),

            // Settings commands
            Command::new("settings.open", "Open Settings")
                .icon(IconSource::Named("settings".into()))
                .description("Open application settings")
                .category("Settings")
                .shortcut("Cmd+,")
                .on_select({
                    let entity = entity.clone();
                    move |_, cx| {
                        cx.update_entity(&entity, |this, cx| {
                            this.execute_command("Open Settings", cx);
                        });
                    }
                }),
        ]
    }
}

impl Render for CommandPaletteDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .relative()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .on_action(cx.listener(|view: &mut Self, _action: &TogglePalette, _window, cx| {
                view.toggle_palette(cx);
            }))
            .child(
                VStack::new()
                    .p(px(32.0))
                    .gap(px(32.0))
                    .size_full()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Command Palette Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Searchable command palette (Cmd+K style) for quick access to application commands")
                            )
                    )
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
                                    .child("How to Use")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Click the button below or press Cmd+K to open the command palette")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Type to search commands by name or description")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Use ↑↓ arrow keys to navigate, Enter to select")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Press Escape or click outside to close the palette")
                            )
                    )
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.card)
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
                                    .child("Current State")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Last Command: {}", self.last_command))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Theme: {}", self.theme_mode))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Sidebar: {}", if self.sidebar_visible { "Visible" } else { "Hidden" }))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .child(
                                Button::new("open-palette-btn", "Open Command Palette")
                                    .variant(ButtonVariant::Default)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.toggle_palette(cx);
                                    }))
                            )
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .rounded(theme.tokens.radius_sm)
                                    .bg(theme.tokens.muted)
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Cmd+K")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Features")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Fuzzy search with relevance scoring")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Command categories (File, Edit, View, Theme, Settings)")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Icons and keyboard shortcuts display")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Full keyboard navigation")
                                    )
                            )
                    )
            )
            .children(if self.show_palette {
                let commands = self.create_commands(cx);
                let entity = cx.entity().clone();

                vec![cx.new(|palette_cx| {
                    CommandPalette::new(_window, palette_cx, commands)
                        .on_close(move |_, close_cx| {
                            close_cx.update_entity(&entity, |this, inner_cx| {
                                this.show_palette = false;
                                inner_cx.notify();
                            });
                        })
                }).into_any_element()]
            } else {
                vec![]
            })
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(move |cx: &mut App| {
        adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());
        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");

        cx.on_action(|_: &Quit, cx| cx.quit());

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-k", TogglePalette, None),
            KeyBinding::new("up", NavigateUp, None),
            KeyBinding::new("down", NavigateDown, None),
            KeyBinding::new("enter", SelectCommand, None),
            KeyBinding::new("escape", CloseCommand, None),
        ]);
        cx.activate(true);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Command Palette Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| CommandPaletteDemo::new(cx)),
        )
        .unwrap();
    });
}
