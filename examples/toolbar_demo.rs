use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    navigation::toolbar::{Toolbar, ToolbarButton, ToolbarGroup, ToolbarItem, ToolbarButtonVariant, ToolbarSize},
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

actions!(toolbar_demo, [Quit]);

struct ToolbarDemo {
    bold_active: bool,
    italic_active: bool,
    underline_active: bool,
    align_left: bool,
    align_center: bool,
    align_right: bool,
    toolbar_size: ToolbarSize,
    last_action: String,
}

impl ToolbarDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            bold_active: false,
            italic_active: false,
            underline_active: false,
            align_left: true,
            align_center: false,
            align_right: false,
            toolbar_size: ToolbarSize::Md,
            last_action: "None".to_string(),
        }
    }

    fn toggle_bold(&mut self, cx: &mut Context<Self>) {
        self.bold_active = !self.bold_active;
        self.last_action = format!("Bold {}", if self.bold_active { "ON" } else { "OFF" });
        cx.notify();
    }

    fn toggle_italic(&mut self, cx: &mut Context<Self>) {
        self.italic_active = !self.italic_active;
        self.last_action = format!("Italic {}", if self.italic_active { "ON" } else { "OFF" });
        cx.notify();
    }

    fn toggle_underline(&mut self, cx: &mut Context<Self>) {
        self.underline_active = !self.underline_active;
        self.last_action = format!("Underline {}", if self.underline_active { "ON" } else { "OFF" });
        cx.notify();
    }

    fn set_alignment(&mut self, alignment: &str, cx: &mut Context<Self>) {
        self.align_left = alignment == "left";
        self.align_center = alignment == "center";
        self.align_right = alignment == "right";
        self.last_action = format!("Align {}", alignment);
        cx.notify();
    }

    fn handle_action(&mut self, action: &str, cx: &mut Context<Self>) {
        self.last_action = action.to_string();
        cx.notify();
    }
}

impl Render for ToolbarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .child(
                VStack::new()
                    .gap(px(0.0))
                    .size_full()
                    // Header with title and size controls
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px(px(32.0))
                            .py(px(24.0))
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(32.0))
                                            .font_weight(FontWeight::BOLD)
                                            .child("Toolbar Demo")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Action bars with icon buttons, groups, and toggle states")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Toolbar Size:")
                                    )
                                    .child(
                                        div()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(theme.tokens.radius_sm)
                                            .bg(if matches!(self.toolbar_size, ToolbarSize::Sm) {
                                                theme.tokens.accent
                                            } else {
                                                theme.tokens.muted
                                            })
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.toolbar_size = ToolbarSize::Sm;
                                                cx.notify();
                                            }))
                                            .child("Small")
                                    )
                                    .child(
                                        div()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(theme.tokens.radius_sm)
                                            .bg(if matches!(self.toolbar_size, ToolbarSize::Md) {
                                                theme.tokens.accent
                                            } else {
                                                theme.tokens.muted
                                            })
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.toolbar_size = ToolbarSize::Md;
                                                cx.notify();
                                            }))
                                            .child("Medium")
                                    )
                                    .child(
                                        div()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(theme.tokens.radius_sm)
                                            .bg(if matches!(self.toolbar_size, ToolbarSize::Lg) {
                                                theme.tokens.accent
                                            } else {
                                                theme.tokens.muted
                                            })
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.toolbar_size = ToolbarSize::Lg;
                                                cx.notify();
                                            }))
                                            .child("Large")
                                    )
                            )
                    )
                    .child({
                        let formatting_group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("bold", IconSource::Named("bold".into()))
                                    .tooltip("Bold")
                                    .variant(ToolbarButtonVariant::Toggle)
                                    .pressed(self.bold_active)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.toggle_bold(cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("italic", IconSource::Named("italic".into()))
                                    .tooltip("Italic")
                                    .variant(ToolbarButtonVariant::Toggle)
                                    .pressed(self.italic_active)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.toggle_italic(cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("underline", IconSource::Named("underline".into()))
                                    .tooltip("Underline")
                                    .variant(ToolbarButtonVariant::Toggle)
                                    .pressed(self.underline_active)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.toggle_underline(cx);
                                            });
                                        }
                                    })
                            );

                        let alignment_group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("align_left", IconSource::Named("align-left".into()))
                                    .tooltip("Align Left")
                                    .variant(ToolbarButtonVariant::Toggle)
                                    .pressed(self.align_left)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.set_alignment("left", cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("align_center", IconSource::Named("align-center".into()))
                                    .tooltip("Align Center")
                                    .variant(ToolbarButtonVariant::Toggle)
                                    .pressed(self.align_center)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.set_alignment("center", cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("align_right", IconSource::Named("align-right".into()))
                                    .tooltip("Align Right")
                                    .variant(ToolbarButtonVariant::Toggle)
                                    .pressed(self.align_right)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.set_alignment("right", cx);
                                            });
                                        }
                                    })
                            );

                        let list_group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("bullet_list", IconSource::Named("list".into()))
                                    .tooltip("Bullet List")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Bullet List", cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("numbered_list", IconSource::Named("list-ordered".into()))
                                    .tooltip("Numbered List")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Numbered List", cx);
                                            });
                                        }
                                    })
                            );

                        cx.new(|_cx| {
                            Toolbar::new()
                                .size(self.toolbar_size)
                                .group(formatting_group)
                                .group(alignment_group)
                                .group(list_group)
                        })
                    })
                    .child({
                        let file_group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("new", IconSource::Named("file-plus".into()))
                                    .tooltip("New File")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("New File", cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("open", IconSource::Named("folder-open".into()))
                                    .tooltip("Open File")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Open File", cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("save", IconSource::Named("save".into()))
                                    .tooltip("Save")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Save", cx);
                                            });
                                        }
                                    })
                            );

                        let edit_group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("undo", IconSource::Named("undo".into()))
                                    .tooltip("Undo")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Undo", cx);
                                            });
                                        }
                                    })
                            )
                            .button(
                                ToolbarButton::new("redo", IconSource::Named("redo".into()))
                                    .tooltip("Redo")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Redo", cx);
                                            });
                                        }
                                    })
                            );

                        let spacer_group = ToolbarGroup::new()
                            .spacer();

                        let settings_group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("settings", IconSource::Named("settings".into()))
                                    .tooltip("Settings")
                                    .variant(ToolbarButtonVariant::Dropdown)
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Settings", cx);
                                            });
                                        }
                                    })
                            );

                        cx.new(|_cx| {
                            Toolbar::new()
                                .size(self.toolbar_size)
                                .group(file_group)
                                .group(edit_group)
                                .group(spacer_group)
                                .group(settings_group)
                        })
                    })
                    .child({
                        let group = ToolbarGroup::new()
                            .button(
                                ToolbarButton::new("active", IconSource::Named("check".into()))
                                    .tooltip("Active Button")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Active Button", cx);
                                            });
                                        }
                                    })
                            )
                            .separator()
                            .button(
                                ToolbarButton::new("disabled", IconSource::Named("x".into()))
                                    .tooltip("Disabled Button")
                                    .disabled(true)
                            )
                            .separator()
                            .button(
                                ToolbarButton::new("another", IconSource::Named("star".into()))
                                    .tooltip("Another Button")
                                    .on_click({
                                        let entity = cx.entity().clone();
                                        move |_, cx| {
                                            cx.update_entity(&entity, |this, cx| {
                                                this.handle_action("Another Button", cx);
                                            });
                                        }
                                    })
                            );

                        cx.new(|_cx| {
                            Toolbar::new()
                                .size(self.toolbar_size)
                                .group(group)
                        })
                    })
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .rounded(theme.tokens.radius_md)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .min_w(px(400.0))
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Current State")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Last Action: {}", self.last_action))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Bold: {}", if self.bold_active { "ON" } else { "OFF" }))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Italic: {}", if self.italic_active { "ON" } else { "OFF" }))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Underline: {}", if self.underline_active { "ON" } else { "OFF" }))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!(
                                                "Alignment: {}",
                                                if self.align_left { "Left" }
                                                else if self.align_center { "Center" }
                                                else { "Right" }
                                            ))
                                    )
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
                    size(px(1000.0), px(700.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Toolbar Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| ToolbarDemo::new(cx)),
        )
        .unwrap();
    });
}
