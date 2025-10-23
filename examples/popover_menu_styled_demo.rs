use adabraka_ui::{
    overlays::popover_menu::{PopoverMenu, PopoverMenuItem},
    prelude::*,
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
                        title: Some("PopoverMenu Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| PopoverMenuStyledDemoView::new()),
            )
            .unwrap();
        });
}

struct PopoverMenuStyledDemoView {
    show_default: bool,
    show_custom_bg: bool,
    show_custom_border: bool,
    show_custom_size: bool,
    show_gradient: bool,
    show_compact: bool,
    show_wide: bool,
    show_elevated: bool,
}

impl PopoverMenuStyledDemoView {
    fn new() -> Self {
        Self {
            show_default: false,
            show_custom_bg: false,
            show_custom_border: false,
            show_custom_size: false,
            show_gradient: false,
            show_compact: false,
            show_wide: false,
            show_elevated: false,
        }
    }
}

impl Render for PopoverMenuStyledDemoView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_6()
            .p_8()
            .bg(rgb(0xf5f5f5))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1a1a1a))
                            .mb_4()
                            .child("PopoverMenu Styled Trait Demo")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .mb_4()
                            .child("Click buttons to open PopoverMenus with different styling. Click outside to close.")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_6()
                            // 1. Default PopoverMenu
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Default Menu")
                                    )
                                    .child(
                                        Button::new("default-menu-btn", "Default Menu")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_default = !this.show_default;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 2. Custom Background
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Custom Background")
                                    )
                                    .child(
                                        Button::new("custom-bg-btn", "Blue Background")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_custom_bg = !this.show_custom_bg;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 3. Custom Border
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Custom Border")
                                    )
                                    .child(
                                        Button::new("custom-border-btn", "Thick Border")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_custom_border = !this.show_custom_border;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 4. Custom Size
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Custom Size")
                                    )
                                    .child(
                                        Button::new("custom-size-btn", "Wide Menu")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_wide = !this.show_wide;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 5. Gradient Background
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Gradient Menu")
                                    )
                                    .child(
                                        Button::new("gradient-btn", "Gradient")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_gradient = !this.show_gradient;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 6. Compact Menu
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Compact Menu")
                                    )
                                    .child(
                                        Button::new("compact-btn", "Compact")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_compact = !this.show_compact;
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 7. Elevated Menu
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Elevated Menu")
                                    )
                                    .child(
                                        Button::new("elevated-btn", "Elevated")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_elevated = !this.show_elevated;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
            )
            // Default Menu
            .when(self.show_default, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "Edit")
                        .icon("edit")
                        .on_click(|_, _| println!("Edit clicked")),
                    PopoverMenuItem::new("item2", "Copy")
                        .icon("copy")
                        .on_click(|_, _| println!("Copy clicked")),
                    PopoverMenuItem::new("item3", "Delete")
                        .icon("trash")
                        .on_click(|_, _| println!("Delete clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(150.0), px(250.0)), items)
                        .on_close(cx.listener(|this, _, _| {
                            this.show_default = false;
                            cx.notify();
                        }))
                )
            })
            // Custom Background Menu
            .when(self.show_custom_bg, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "Profile")
                        .icon("user")
                        .on_click(|_, _| println!("Profile clicked")),
                    PopoverMenuItem::new("item2", "Settings")
                        .icon("settings")
                        .on_click(|_, _| println!("Settings clicked")),
                    PopoverMenuItem::new("item3", "Logout")
                        .icon("log-out")
                        .on_click(|_, _| println!("Logout clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(350.0), px(250.0)), items)
                        .bg(rgb(0xe3f2fd))
                        .on_close(cx.listener(|this, _, _| {
                            this.show_custom_bg = false;
                            cx.notify();
                        }))
                )
            })
            // Custom Border Menu
            .when(self.show_custom_border, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "New File")
                        .icon("file")
                        .on_click(|_, _| println!("New File clicked")),
                    PopoverMenuItem::new("item2", "Open")
                        .icon("folder-open")
                        .on_click(|_, _| println!("Open clicked")),
                    PopoverMenuItem::new("item3", "Save")
                        .icon("save")
                        .on_click(|_, _| println!("Save clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(550.0), px(250.0)), items)
                        .border_3()
                        .border_color(rgb(0x9c27b0))
                        .rounded(px(16.0))
                        .on_close(cx.listener(|this, _, _| {
                            this.show_custom_border = false;
                            cx.notify();
                        }))
                )
            })
            // Wide Menu
            .when(self.show_wide, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "Dashboard Overview")
                        .icon("layout-dashboard")
                        .on_click(|_, _| println!("Dashboard clicked")),
                    PopoverMenuItem::new("item2", "Analytics & Reports")
                        .icon("bar-chart")
                        .on_click(|_, _| println!("Analytics clicked")),
                    PopoverMenuItem::new("item3", "User Management")
                        .icon("users")
                        .on_click(|_, _| println!("Users clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(150.0), px(400.0)), items)
                        .min_w(px(350.0))
                        .max_w(px(500.0))
                        .p_3()
                        .on_close(cx.listener(|this, _, _| {
                            this.show_wide = false;
                            cx.notify();
                        }))
                )
            })
            // Gradient Menu
            .when(self.show_gradient, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "Premium Feature")
                        .icon("star")
                        .on_click(|_, _| println!("Premium clicked")),
                    PopoverMenuItem::new("item2", "Upgrade Plan")
                        .icon("zap")
                        .on_click(|_, _| println!("Upgrade clicked")),
                    PopoverMenuItem::new("item3", "View Benefits")
                        .icon("gift")
                        .on_click(|_, _| println!("Benefits clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(350.0), px(400.0)), items)
                        .bg(rgb(0x667eea))
                        .text_color(white())
                        .border_color(rgb(0x764ba2))
                        .on_close(cx.listener(|this, _, _| {
                            this.show_gradient = false;
                            cx.notify();
                        }))
                )
            })
            // Compact Menu
            .when(self.show_compact, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "Undo")
                        .icon("undo")
                        .on_click(|_, _| println!("Undo clicked")),
                    PopoverMenuItem::new("item2", "Redo")
                        .icon("redo")
                        .on_click(|_, _| println!("Redo clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(550.0), px(400.0)), items)
                        .min_w(px(150.0))
                        .p_1()
                        .rounded(px(6.0))
                        .on_close(cx.listener(|this, _, _| {
                            this.show_compact = false;
                            cx.notify();
                        }))
                )
            })
            // Elevated Menu
            .when(self.show_elevated, |this| {
                let items = vec![
                    PopoverMenuItem::new("item1", "Export PDF")
                        .icon("file-text")
                        .on_click(|_, _| println!("Export PDF clicked")),
                    PopoverMenuItem::new("item2", "Export CSV")
                        .icon("table")
                        .on_click(|_, _| println!("Export CSV clicked")),
                    PopoverMenuItem::new("item3", "Print")
                        .icon("printer")
                        .on_click(|_, _| println!("Print clicked")),
                ];

                this.child(
                    PopoverMenu::new(point(px(150.0), px(550.0)), items)
                        .shadow(vec![
                            BoxShadow {
                                offset: point(px(0.0), px(8.0)),
                                blur_radius: px(24.0),
                                spread_radius: px(0.0),
                                color: hsla(0.0, 0.0, 0.0, 0.35),
                            }
                        ])
                        .on_close(cx.listener(|this, _, _| {
                            this.show_elevated = false;
                            cx.notify();
                        }))
                )
            })
    }
}
