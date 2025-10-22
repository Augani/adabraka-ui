use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    navigation::status_bar::{StatusBar, StatusItem},
    components::badge::BadgeVariant,
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

actions!(status_bar_demo, [Quit, ShowNotifications, ShowWarnings, ShowErrors, ChangeEncoding, ToggleGit]);

struct StatusBarDemo {
    notifications: usize,
    warnings: usize,
    errors: usize,
    encoding: String,
    git_enabled: bool,
    current_file: String,
    line: usize,
    column: usize,
    last_action: String,
}

impl StatusBarDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            notifications: 3,
            warnings: 5,
            errors: 2,
            encoding: "UTF-8".to_string(),
            git_enabled: true,
            current_file: "main.rs".to_string(),
            line: 42,
            column: 15,
            last_action: "None".to_string(),
        }
    }

    fn handle_action(&mut self, action: &str, cx: &mut Context<Self>) {
        self.last_action = action.to_string();

        match action {
            "Show Notifications" => {
                println!("Opening notifications panel ({} items)", self.notifications);
            }
            "Show Warnings" => {
                println!("Opening warnings panel ({} items)", self.warnings);
            }
            "Show Errors" => {
                println!("Opening errors panel ({} items)", self.errors);
            }
            "Change Encoding" => {
                self.encoding = if self.encoding == "UTF-8" { "UTF-16" } else { "UTF-8" }.to_string();
            }
            "Toggle Git" => {
                self.git_enabled = !self.git_enabled;
            }
            _ => {}
        }

        cx.notify();
    }
}

impl Render for StatusBarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .on_action(cx.listener(|this, _: &ShowNotifications, _window, cx| {
                this.handle_action("Show Notifications", cx);
            }))
            .on_action(cx.listener(|this, _: &ShowWarnings, _window, cx| {
                this.handle_action("Show Warnings", cx);
            }))
            .on_action(cx.listener(|this, _: &ShowErrors, _window, cx| {
                this.handle_action("Show Errors", cx);
            }))
            .on_action(cx.listener(|this, _: &ChangeEncoding, _window, cx| {
                this.handle_action("Change Encoding", cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleGit, _window, cx| {
                this.handle_action("Toggle Git", cx);
            }))
            .on_action(cx.listener(|_this, _: &Quit, _window, cx| {
                cx.quit();
            }))
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
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
                                            .child("Status Bar Demo")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Bottom application status bar with sections and interactive items")
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
                                            .child("• Look at the status bar at the bottom of the window")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child("• Click on the badges (notifications, warnings, errors) to see actions")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child("• Click on encoding (UTF-8) to toggle between UTF-8 and UTF-16")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child("• Click on Git status to toggle Git integration")
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
                                            .child(format!("Last Action: {}", self.last_action))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Notifications: {}", self.notifications))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Warnings: {}", self.warnings))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Errors: {}", self.errors))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Encoding: {}", self.encoding))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .child(format!("Git: {}", if self.git_enabled { "Enabled" } else { "Disabled" }))
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
                                            .child("Features Demonstrated")
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
                                                    .child("Three sections: Left, Center, Right")
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
                                                    .child("Icons with text")
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
                                                    .child("Badges for counts (notifications, warnings, errors)")
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
                                                    .child("Interactive items with click handlers")
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
                                                    .child("Hover effects for clickable items")
                                            )
                                    )
                            )
                    )
            )
            .child({
                let entity = cx.entity().clone();
                cx.new(|_| {
                    StatusBar::new()
                        .left(vec![
                            StatusItem::icon_text(IconSource::Named("file".into()), &self.current_file),
                            StatusItem::text(format!("Ln {}, Col {}", self.line, self.column)),
                        ])
                        .center(vec![
                            StatusItem::text("Ready"),
                        ])
                        .right(vec![
                            if self.notifications > 0 {
                                StatusItem::icon_badge(
                                    IconSource::Named("bell".into()),
                                    format!("{}", self.notifications)
                                )
                                .badge_variant(BadgeVariant::Default)
                                .tooltip("Notifications")
                                .on_click({
                                    let entity = entity.clone();
                                    move |_window, app_cx| {
                                        app_cx.update_entity(&entity, |this: &mut StatusBarDemo, cx| {
                                            this.handle_action("Show Notifications", cx);
                                        });
                                    }
                                })
                            } else {
                                StatusItem::icon(IconSource::Named("bell".into()))
                                    .disabled(true)
                            },
                            if self.warnings > 0 {
                                StatusItem::icon_badge(
                                    IconSource::Named("alert-triangle".into()),
                                    format!("{}", self.warnings)
                                )
                                .badge_variant(BadgeVariant::Warning)
                                .tooltip("Warnings")
                                .on_click({
                                    let entity = entity.clone();
                                    move |_window, app_cx| {
                                        app_cx.update_entity(&entity, |this: &mut StatusBarDemo, cx| {
                                            this.handle_action("Show Warnings", cx);
                                        });
                                    }
                                })
                            } else {
                                StatusItem::icon(IconSource::Named("alert-triangle".into()))
                                    .disabled(true)
                            },
                            if self.errors > 0 {
                                StatusItem::icon_badge(
                                    IconSource::Named("alert-circle".into()),
                                    format!("{}", self.errors)
                                )
                                .badge_variant(BadgeVariant::Destructive)
                                .tooltip("Errors")
                                .on_click({
                                    let entity = entity.clone();
                                    move |_window, app_cx| {
                                        app_cx.update_entity(&entity, |this: &mut StatusBarDemo, cx| {
                                            this.handle_action("Show Errors", cx);
                                        });
                                    }
                                })
                            } else {
                                StatusItem::icon(IconSource::Named("alert-circle".into()))
                                    .disabled(true)
                            },
                            StatusItem::text(&self.encoding)
                                .tooltip("File Encoding")
                                .on_click({
                                    let entity = entity.clone();
                                    move |_window, app_cx| {
                                        app_cx.update_entity(&entity, |this: &mut StatusBarDemo, cx| {
                                            this.handle_action("Change Encoding", cx);
                                        });
                                    }
                                }),
                            if self.git_enabled {
                                StatusItem::icon_text(IconSource::Named("git-branch".into()), "main")
                                    .tooltip("Git Branch")
                                    .on_click({
                                        let entity = entity.clone();
                                        move |_window, app_cx| {
                                            app_cx.update_entity(&entity, |this: &mut StatusBarDemo, cx| {
                                                this.handle_action("Toggle Git", cx);
                                            });
                                        }
                                    })
                            } else {
                                StatusItem::text("No Git")
                                    .disabled(true)
                            },
                        ])
                })
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
                    title: Some("Status Bar Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| StatusBarDemo::new(cx)),
        )
        .unwrap();
    });
}
