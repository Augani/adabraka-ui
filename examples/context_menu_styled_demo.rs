use adabraka_ui::{
    prelude::*,
    overlays::context_menu::{ContextMenu, ContextMenuItem},
    components::scrollable::scrollable_vertical,
};
use gpui::*;
use std::path::PathBuf;
use std::rc::Rc;

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
                        title: Some("ContextMenu Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ContextMenuStyledDemo::new()),
            )
            .unwrap();
        });
}

struct ContextMenuStyledDemo {
    show_default: bool,
    show_custom_bg: bool,
    show_custom_border: bool,
    show_custom_radius: bool,
    show_custom_shadow: bool,
    show_combined: bool,
    click_position: Point<Pixels>,
    selected_item: Rc<std::cell::RefCell<String>>,
}

impl ContextMenuStyledDemo {
    fn new() -> Self {
        Self {
            show_default: false,
            show_custom_bg: false,
            show_custom_border: false,
            show_custom_radius: false,
            show_custom_shadow: false,
            show_combined: false,
            click_position: point(px(0.0), px(0.0)),
            selected_item: Rc::new(std::cell::RefCell::new("None".to_string())),
        }
    }

    fn close_all_menus(&mut self) {
        self.show_default = false;
        self.show_custom_bg = false;
        self.show_custom_border = false;
        self.show_custom_radius = false;
        self.show_custom_shadow = false;
        self.show_combined = false;
    }
}

impl Render for ContextMenuStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let selected_text = self.selected_item.borrow().clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .relative()
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
                                        .child("ContextMenu Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.accent)
                                        .child(format!("Selected: {}", selected_text))
                                )
                        )
                        // 1. Default ContextMenu
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default ContextMenu (No Custom Styling)")
                                )
                                .child(
                                    div()
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.1))
                                        .rounded(px(8.0))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(MouseButton::Right, cx.listener(|view, event: &MouseDownEvent, _, cx| {
                                            view.close_all_menus();
                                            view.show_default = true;
                                            view.click_position = event.position;
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Right-click here for default context menu")
                                        )
                                )
                        )
                        // 2. Custom Background Color
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Background Color")
                                )
                                .child(
                                    div()
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.1))
                                        .rounded(px(8.0))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(MouseButton::Right, cx.listener(|view, event: &MouseDownEvent, _, cx| {
                                            view.close_all_menus();
                                            view.show_custom_bg = true;
                                            view.click_position = event.position;
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Right-click here for purple background menu")
                                        )
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
                                        .child("3. Custom Border (Styled trait)")
                                )
                                .child(
                                    div()
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.1))
                                        .rounded(px(8.0))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(MouseButton::Right, cx.listener(|view, event: &MouseDownEvent, _, cx| {
                                            view.close_all_menus();
                                            view.show_custom_border = true;
                                            view.click_position = event.position;
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Right-click here for custom border menu")
                                        )
                                )
                        )
                        // 4. Custom Border Radius
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Border Radius")
                                )
                                .child(
                                    div()
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.1))
                                        .rounded(px(8.0))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(MouseButton::Right, cx.listener(|view, event: &MouseDownEvent, _, cx| {
                                            view.close_all_menus();
                                            view.show_custom_radius = true;
                                            view.click_position = event.position;
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Right-click here for no radius menu")
                                        )
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
                                    div()
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.1))
                                        .rounded(px(8.0))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(MouseButton::Right, cx.listener(|view, event: &MouseDownEvent, _, cx| {
                                            view.close_all_menus();
                                            view.show_custom_shadow = true;
                                            view.click_position = event.position;
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Right-click here for large shadow menu")
                                        )
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
                                    div()
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.1))
                                        .rounded(px(8.0))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(MouseButton::Right, cx.listener(|view, event: &MouseDownEvent, _, cx| {
                                            view.close_all_menus();
                                            view.show_combined = true;
                                            view.click_position = event.position;
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Right-click here for ultra-custom menu")
                                        )
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
                                        .child("Methods used: .bg(), .border_3(), .border_color(), .rounded(), .shadow_lg(), .p(), .min_w()")
                                )
                        )
                )
            )
            // Render context menus
            .when(self.show_default, {
                let selected = self.selected_item.clone();
                let entity = cx.entity().clone();
                move |this| {
                    let selected = selected.clone();
                    let entity = entity.clone();
                    this.child(
                        ContextMenu::new(self.click_position)
                            .items(vec![
                                ContextMenuItem::new("copy", "Copy")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Copy".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::new("paste", "Paste")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Paste".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::separator(),
                                ContextMenuItem::new("delete", "Delete")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Delete".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                            ])
                            .on_close({
                                let entity = entity.clone();
                                move |_, cx| {
                                    cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                        view.close_all_menus();
                                        cx.notify();
                                    });
                                }
                            })
                    )
                }
            })
            .when(self.show_custom_bg, {
                let selected = self.selected_item.clone();
                let entity = cx.entity().clone();
                move |this| {
                    let selected = selected.clone();
                    let entity = entity.clone();
                    this.child(
                        ContextMenu::new(self.click_position)
                            .bg(rgb(0x8b5cf6))  // Purple background
                            .items(vec![
                                ContextMenuItem::new("action1", "Action 1")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Action 1 (Purple BG)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::new("action2", "Action 2")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Action 2 (Purple BG)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                            ])
                            .on_close({
                                let entity = entity.clone();
                                move |_, cx| {
                                    cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                        view.close_all_menus();
                                        cx.notify();
                                    });
                                }
                            })
                    )
                }
            })
            .when(self.show_custom_border, {
                let selected = self.selected_item.clone();
                let entity = cx.entity().clone();
                move |this| {
                    let selected = selected.clone();
                    let entity = entity.clone();
                    this.child(
                        ContextMenu::new(self.click_position)
                            .border_3()  // Custom border
                            .border_color(rgb(0x3b82f6))  // Blue border
                            .items(vec![
                                ContextMenuItem::new("option1", "Option 1")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Option 1 (Custom Border)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::new("option2", "Option 2")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Option 2 (Custom Border)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                            ])
                            .on_close({
                                let entity = entity.clone();
                                move |_, cx| {
                                    cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                        view.close_all_menus();
                                        cx.notify();
                                    });
                                }
                            })
                    )
                }
            })
            .when(self.show_custom_radius, {
                let selected = self.selected_item.clone();
                let entity = cx.entity().clone();
                move |this| {
                    let selected = selected.clone();
                    let entity = entity.clone();
                    this.child(
                        ContextMenu::new(self.click_position)
                            .rounded(px(0.0))  // No border radius
                            .items(vec![
                                ContextMenuItem::new("item1", "Item 1")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Item 1 (No Radius)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::new("item2", "Item 2")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Item 2 (No Radius)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                            ])
                            .on_close({
                                let entity = entity.clone();
                                move |_, cx| {
                                    cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                        view.close_all_menus();
                                        cx.notify();
                                    });
                                }
                            })
                    )
                }
            })
            .when(self.show_custom_shadow, {
                let selected = self.selected_item.clone();
                let entity = cx.entity().clone();
                move |this| {
                    let selected = selected.clone();
                    let entity = entity.clone();
                    this.child(
                        ContextMenu::new(self.click_position)
                            .shadow_lg()  // Large shadow
                            .items(vec![
                                ContextMenuItem::new("choice1", "Choice 1")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Choice 1 (Large Shadow)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::new("choice2", "Choice 2")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Choice 2 (Large Shadow)".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                            ])
                            .on_close({
                                let entity = entity.clone();
                                move |_, cx| {
                                    cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                        view.close_all_menus();
                                        cx.notify();
                                    });
                                }
                            })
                    )
                }
            })
            .when(self.show_combined, {
                let selected = self.selected_item.clone();
                let entity = cx.entity().clone();
                move |this| {
                    let selected = selected.clone();
                    let entity = entity.clone();
                    this.child(
                        ContextMenu::new(self.click_position)
                            .bg(rgb(0xf59e0b))  // Orange background
                            .border_3()
                            .border_color(rgb(0xdc2626))  // Red border
                            .rounded(px(16.0))  // Large radius
                            .shadow_lg()  // Large shadow
                            .p(px(12.0))  // Extra padding
                            .min_w(px(300.0))  // Wider menu
                            .items(vec![
                                ContextMenuItem::new("ultra1", "Ultra Custom 1")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Ultra Custom 1".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::new("ultra2", "Ultra Custom 2")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Ultra Custom 2".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ContextMenuItem::separator(),
                                ContextMenuItem::new("ultra3", "Ultra Custom 3")
                                    .on_click({
                                        let selected = selected.clone();
                                        let entity = entity.clone();
                                        move |_, cx| {
                                            *selected.borrow_mut() = "Ultra Custom 3".to_string();
                                            cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                                view.close_all_menus();
                                                cx.notify();
                                            });
                                        }
                                    }),
                            ])
                            .on_close({
                                let entity = entity.clone();
                                move |_, cx| {
                                    cx.update_entity(&entity, |view: &mut ContextMenuStyledDemo, cx| {
                                        view.close_all_menus();
                                        cx.notify();
                                    });
                                }
                            })
                    )
                }
            })
    }
}
