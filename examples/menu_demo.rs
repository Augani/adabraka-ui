use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    navigation::menu::{Menu, MenuItem, MenuItemKind, MenuBar, MenuBarItem, ContextMenu},
    components::button::{Button, ButtonVariant},
    components::icon::IconSource,
    layout::VStack,
    theme::use_theme,
};

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
                                                MenuItem::new("layout_1", "Option 1")
                                                    .kind(MenuItemKind::Radio { checked: self.radio_option == "Option 1" })
                                                    .on_click({
                                                        let entity = cx.entity().clone();
                                                        move |_, cx| {
                                                            cx.update_entity(&entity, |this, cx| {
                                                                this.set_radio_option("Option 1", cx);
                                                            });
                                                        }
                                                    }),
                                                MenuItem::new("layout_2", "Option 2")
                                                    .kind(MenuItemKind::Radio { checked: self.radio_option == "Option 2" })
                                                    .on_click({
                                                        let entity = cx.entity().clone();
                                                        move |_, cx| {
                                                            cx.update_entity(&entity, |this, cx| {
                                                                this.set_radio_option("Option 2", cx);
                                                            });
                                                        }
                                                    }),
                                                MenuItem::new("layout_3", "Option 3")
                                                    .kind(MenuItemKind::Radio { checked: self.radio_option == "Option 3" })
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

                                        cx.new(|cx| {
                                            MenuBar::new(cx, vec![
                                                MenuBarItem::new("File", file_menu),
                                                MenuBarItem::new("Edit", edit_menu),
                                                MenuBarItem::new("View", view_menu),
                                            ])
                                        })
                                    })
                            )
                            // Standalone Menu Section
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
                                                    .child("Standalone Menu")
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Menu component with various item types")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(12.0))
                                            .child({
                                                let items = vec![
                                                    MenuItem::new("action1", "Regular Action")
                                                        .with_icon(IconSource::Named("check".into()))
                                                        .on_click({
                                                            let entity = cx.entity().clone();
                                                            move |_, cx| {
                                                                cx.update_entity(&entity, |this, cx| {
                                                                    this.handle_menu_action("Regular Action", cx);
                                                                });
                                                            }
                                                        }),
                                                    MenuItem::new("action2", "With Shortcut")
                                                        .with_shortcut("Ctrl+K")
                                                        .on_click({
                                                            let entity = cx.entity().clone();
                                                            move |_, cx| {
                                                                cx.update_entity(&entity, |this, cx| {
                                                                    this.handle_menu_action("With Shortcut", cx);
                                                                });
                                                            }
                                                        }),
                                                    MenuItem::separator(),
                                                    MenuItem::new("disabled", "Disabled Item")
                                                        .disabled(true),
                                                ];

                                                cx.new(|cx| Menu::new(cx, items))
                                            })
                                    )
                            )
                            // Context Menu Section
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
                                                    .child("Context Menu")
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Right-click the box below to open a context menu")
                                            )
                                    )
                                    .child(
                                        div()
                                            .relative()
                                            .w(px(400.0))
                                            .h(px(200.0))
                                            .bg(theme.tokens.muted.opacity(0.3))
                                            .border_2()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_md)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                                this.show_context_menu = true;
                                                this.context_menu_position = event.position;
                                                cx.notify();
                                            }))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Right-click here")
                                            )
                                            .children(if self.show_context_menu {
                                                let items = vec![
                                                    MenuItem::new("copy", "Copy")
                                                        .with_icon(IconSource::Named("copy".into()))
                                                        .with_shortcut("Cmd+C")
                                                        .on_click({
                                                            let entity = cx.entity().clone();
                                                            move |_, cx| {
                                                                cx.update_entity(&entity, |this, cx| {
                                                                    this.handle_menu_action("Copy", cx);
                                                                    this.show_context_menu = false;
                                                                    cx.notify();
                                                                });
                                                            }
                                                        }),
                                                    MenuItem::new("paste", "Paste")
                                                        .with_icon(IconSource::Named("clipboard".into()))
                                                        .with_shortcut("Cmd+V")
                                                        .on_click({
                                                            let entity = cx.entity().clone();
                                                            move |_, cx| {
                                                                cx.update_entity(&entity, |this, cx| {
                                                                    this.handle_menu_action("Paste", cx);
                                                                    this.show_context_menu = false;
                                                                    cx.notify();
                                                                });
                                                            }
                                                        }),
                                                    MenuItem::separator(),
                                                    MenuItem::new("delete", "Delete")
                                                        .with_icon(IconSource::Named("trash".into()))
                                                        .on_click({
                                                            let entity = cx.entity().clone();
                                                            move |_, cx| {
                                                                cx.update_entity(&entity, |this, cx| {
                                                                    this.handle_menu_action("Delete", cx);
                                                                    this.show_context_menu = false;
                                                                    cx.notify();
                                                                });
                                                            }
                                                        }),
                                                ];

                                                vec![cx.new(|cx| {
                                                    ContextMenu::new(cx, items, self.context_menu_position)
                                                        .on_close({
                                                            let entity = cx.entity().clone();
                                                            move |_, cx| {
                                                                cx.update_entity(&entity, |this, cx| {
                                                                    this.show_context_menu = false;
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                }).into_any_element()]
                                            } else {
                                                vec![]
                                            })
                                    )
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
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.activate(true);

        // Create window
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
