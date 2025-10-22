use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    navigation::menu::{MenuBar, MenuBarItem},
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

actions!(menu_demo, [Quit]);

struct MenuDemo {
    selected_option: String,
    checkbox_state: bool,
    radio_option: String,
    show_context_menu: bool,
    context_menu_position: Point<Pixels>,
}

impl MenuDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_option: "None".to_string(),
            checkbox_state: false,
            radio_option: "Option 1".to_string(),
            show_context_menu: false,
            context_menu_position: point(px(0.0), px(0.0)),
        }
    }

    fn handle_menu_action(&mut self, action: &str, cx: &mut Context<Self>) {
        self.selected_option = action.to_string();
        cx.notify();
    }

    fn toggle_checkbox(&mut self, cx: &mut Context<Self>) {
        self.checkbox_state = !self.checkbox_state;
        cx.notify();
    }

    fn set_radio_option(&mut self, option: &str, cx: &mut Context<Self>) {
        self.radio_option = option.to_string();
        cx.notify();
    }
}

impl Render for MenuDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .relative()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
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
                                    .child("Menu System Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("MenuBar, Menu, MenuItem, and ContextMenu components")
                            )
                    )
                    // Status Display
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
                                    .text_size(px(14.0))
                                    .child(format!("Selected Action: {}", self.selected_option))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Checkbox State: {}", if self.checkbox_state { "Checked" } else { "Unchecked" }))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Radio Selection: {}", self.radio_option))
                            )
                    )
                    // Content
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(48.0))
                            // MenuBar Section
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .child(
                                                div()
                                                    .text_size(px(20.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("MenuBar")
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Application menu bar with dropdowns")
                                            )
                                    )
                                    .child({
                                        let file_menu = vec![
                                            MenuItem::new("new", "New File")
                                                .with_icon(IconSource::Named("file-plus".into()))
                                                .with_shortcut("Cmd+N")
                                                .on_click({
                                                    let entity = cx.entity().clone();
                                                    move |_, cx| {
                                                        cx.update_entity(&entity, |this, cx| {
                                                            this.handle_menu_action("New File", cx);
                                                        });
                                                    }
                                                }),
                                            MenuItem::new("open", "Open...")
                                                .with_icon(IconSource::Named("folder-open".into()))
                                                .with_shortcut("Cmd+O")
                                                .on_click({
                                                    let entity = cx.entity().clone();
                                                    move |_, cx| {
                                                        cx.update_entity(&entity, |this, cx| {
                                                            this.handle_menu_action("Open", cx);
                                                        });
                                                    }
                                                }),
                                            MenuItem::separator(),
                                            MenuItem::new("save", "Save")
                                                .with_shortcut("Cmd+S")
                                                .on_click({
                                                    let entity = cx.entity().clone();
                                                    move |_, cx| {
                                                        cx.update_entity(&entity, |this, cx| {
                                                            this.handle_menu_action("Save", cx);
                                                        });
                                                    }
                                                }),
                                            MenuItem::new("save_as", "Save As...")
                                                .with_shortcut("Cmd+Shift+S")
                                                .disabled(true),
                                        ];

                                        let edit_menu = vec![
                                            MenuItem::new("undo", "Undo")
                                                .with_shortcut("Cmd+Z")
                                                .on_click({
                                                    let entity = cx.entity().clone();
                                                    move |_, cx| {
                                                        cx.update_entity(&entity, |this, cx| {
                                                            this.handle_menu_action("Undo", cx);
                                                        });
                                                    }
                                                }),
                                            MenuItem::new("redo", "Redo")
                                                .with_shortcut("Cmd+Shift+Z")
                                                .on_click({
                                                    let entity = cx.entity().clone();
                                                    move |_, cx| {
                                                        cx.update_entity(&entity, |this, cx| {
                                                            this.handle_menu_action("Redo", cx);
                                                        });
                                                    }
                                                }),
                                            MenuItem::separator(),
                                            MenuItem::checkbox("spell_check", "Spell Check", self.checkbox_state)
                                                .on_click({
                                                    let entity = cx.entity().clone();
                                                    move |_, cx| {
                                                        cx.update_entity(&entity, |this, cx| {
                                                            this.toggle_checkbox(cx);
                                                        });
                                                    }
                                                }),
                                        ];

                                        let view_menu = vec![
                                            MenuItem::new("radio_group", "Layout").with_children(vec![
                                                MenuItem::checkbox("layout_1", "Option 1", self.radio_option == "Option 1")
                                                    .on_click({
                                                        let entity = cx.entity().clone();
                                                        move |_, cx| {
                                                            cx.update_entity(&entity, |this, cx| {
                                                                this.set_radio_option("Option 1", cx);
                                                            });
                                                        }
                                                    }),
                                                MenuItem::checkbox("layout_2", "Option 2", self.radio_option == "Option 2")
                                                    .on_click({
                                                        let entity = cx.entity().clone();
                                                        move |_, cx| {
                                                            cx.update_entity(&entity, |this, cx| {
                                                                this.set_radio_option("Option 2", cx);
                                                            });
                                                        }
                                                    }),
                                                MenuItem::checkbox("layout_3", "Option 3", self.radio_option == "Option 3")
                                                    .on_click({
                                                        let entity = cx.entity().clone();
                                                        move |_, cx| {
                                                            cx.update_entity(&entity, |this, cx| {
                                                                this.set_radio_option("Option 3", cx);
                                                            });
                                                        }
                                                    }),
                                            ]),
                                        ];

                                        cx.new(|_cx| {
                                            MenuBar::new(vec![
                                                MenuBarItem::new("file", "File").with_items(file_menu),
                                                MenuBarItem::new("edit", "Edit").with_items(edit_menu),
                                                MenuBarItem::new("view", "View").with_items(view_menu),
                                            ])
                                        })
                                    })
                            )
                    )
            )
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
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.activate(true);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(900.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Menu System Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| MenuDemo::new(cx)),
        )
        .unwrap();
    });
}
