use adabraka_ui::{
    components::notification_center::{
        NotificationBell, NotificationCenter, NotificationCenterState, NotificationItem,
        NotificationVariant,
    },
    components::scrollable::scrollable_vertical,
    prelude::*,
};
use gpui::prelude::FluentBuilder as _;
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
                        title: Some("Notification Center Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| NotificationCenterDemo::new(cx)),
            )
            .unwrap();
        });
}

struct NotificationCenterDemo {
    notification_state: Entity<NotificationCenterState>,
    show_panel: bool,
    next_id: u64,
}

impl NotificationCenterDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let notification_state = cx.new(|cx| NotificationCenterState::new(cx));

        notification_state.update(cx, |state, cx| {
            state.add(
                NotificationItem::new("notif-1", "Welcome to the App")
                    .message("Thanks for trying out the notification center!")
                    .timestamp("Just now")
                    .variant(NotificationVariant::Info),
                cx,
            );

            state.add(
                NotificationItem::new("notif-2", "Build Completed")
                    .message("Your project has been built successfully.")
                    .timestamp("2 min ago")
                    .variant(NotificationVariant::Success),
                cx,
            );

            state.add(
                NotificationItem::new("notif-3", "Storage Warning")
                    .message("You're running low on storage space.")
                    .timestamp("5 min ago")
                    .variant(NotificationVariant::Warning)
                    .action("Manage Storage", |_, _| {}),
                cx,
            );

            state.add(
                NotificationItem::new("notif-4", "Connection Failed")
                    .message("Unable to connect to the server. Please check your network.")
                    .timestamp("10 min ago")
                    .variant(NotificationVariant::Error)
                    .action("Retry", |_, _| {}),
                cx,
            );

            state.add(
                NotificationItem::new("notif-5", "New Message")
                    .message("You have received a new message from John.")
                    .timestamp("15 min ago")
                    .variant(NotificationVariant::Info)
                    .read(true),
                cx,
            );
        });

        Self {
            notification_state,
            show_panel: false,
            next_id: 6,
        }
    }

    fn get_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn toggle_panel(&mut self, _cx: &mut Context<Self>) {
        self.show_panel = !self.show_panel;
    }

    fn add_info_notification(&mut self, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        self.notification_state.update(cx, |state, cx| {
            state.add(
                NotificationItem::new(("notif", id), "New Information")
                    .message("This is an informational notification.")
                    .timestamp("Just now")
                    .variant(NotificationVariant::Info),
                cx,
            );
        });
    }

    fn add_success_notification(&mut self, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        self.notification_state.update(cx, |state, cx| {
            state.add(
                NotificationItem::new(("notif", id), "Operation Successful")
                    .message("Your changes have been saved.")
                    .timestamp("Just now")
                    .variant(NotificationVariant::Success),
                cx,
            );
        });
    }

    fn add_warning_notification(&mut self, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        self.notification_state.update(cx, |state, cx| {
            state.add(
                NotificationItem::new(("notif", id), "Warning")
                    .message("Please review your settings before proceeding.")
                    .timestamp("Just now")
                    .variant(NotificationVariant::Warning)
                    .action("Review", |_, _| {}),
                cx,
            );
        });
    }

    fn add_error_notification(&mut self, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        self.notification_state.update(cx, |state, cx| {
            state.add(
                NotificationItem::new(("notif", id), "Error Occurred")
                    .message("An unexpected error occurred. Please try again.")
                    .timestamp("Just now")
                    .variant(NotificationVariant::Error)
                    .action("Report Issue", |_, _| {}),
                cx,
            );
        });
    }

    fn add_custom_icon_notification(&mut self, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        self.notification_state.update(cx, |state, cx| {
            state.add(
                NotificationItem::new(("notif", id), "Download Complete")
                    .message("Your file has been downloaded successfully.")
                    .timestamp("Just now")
                    .variant(NotificationVariant::Success)
                    .icon("download"),
                cx,
            );
        });
    }

    fn add_many_notifications(&mut self, cx: &mut Context<Self>) {
        for i in 0..15 {
            let id = self.get_next_id();
            let variant = match i % 4 {
                0 => NotificationVariant::Info,
                1 => NotificationVariant::Success,
                2 => NotificationVariant::Warning,
                _ => NotificationVariant::Error,
            };
            self.notification_state.update(cx, |state, cx| {
                state.add(
                    NotificationItem::new(("notif", id), format!("Notification #{}", i + 1))
                        .message(format!("This is bulk notification number {}.", i + 1))
                        .timestamp("Just now")
                        .variant(variant),
                    cx,
                );
            });
        }
    }

    fn clear_notifications(&mut self, cx: &mut Context<Self>) {
        self.notification_state.update(cx, |state, cx| {
            state.clear_all(cx);
        });
    }
}

impl Render for NotificationCenterDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let show_panel = self.show_panel;
        let unread_count = self.notification_state.read(cx).unread_count();

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
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    VStack::new()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_size(px(24.0))
                                                .font_weight(FontWeight::BOLD)
                                                .child("Notification Center Demo"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("A comprehensive notification system with bell indicator"),
                                        ),
                                )
                                .child(
                                    NotificationBell::new(self.notification_state.clone())
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.toggle_panel(cx);
                                            cx.notify();
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .p(px(16.0))
                                .bg(theme.tokens.card)
                                .rounded(theme.tokens.radius_md)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.foreground)
                                                .font_family(theme.tokens.font_family.clone())
                                                .child(format!("Unread notifications: {}", unread_count)),
                                        ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Add Notifications"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(px(12.0))
                                        .child(
                                            Button::new("btn-info", "Add Info")
                                                .variant(ButtonVariant::Outline)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.add_info_notification(cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("btn-success", "Add Success")
                                                .variant(ButtonVariant::Default)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.add_success_notification(cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("btn-warning", "Add Warning")
                                                .variant(ButtonVariant::Secondary)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.add_warning_notification(cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("btn-error", "Add Error")
                                                .variant(ButtonVariant::Destructive)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.add_error_notification(cx);
                                                })),
                                        ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Custom Notifications"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(px(12.0))
                                        .child(
                                            Button::new("btn-custom-icon", "With Custom Icon")
                                                .variant(ButtonVariant::Outline)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.add_custom_icon_notification(cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("btn-many", "Add 15 Notifications")
                                                .variant(ButtonVariant::Outline)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.add_many_notifications(cx);
                                                })),
                                        ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Actions"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(px(12.0))
                                        .child(
                                            Button::new("btn-toggle", "Toggle Panel")
                                                .variant(ButtonVariant::Default)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.toggle_panel(cx);
                                                    cx.notify();
                                                })),
                                        )
                                        .child(
                                            Button::new("btn-clear", "Clear All")
                                                .variant(ButtonVariant::Ghost)
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.clear_notifications(cx);
                                                })),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(16.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(8.0))
                                .child(
                                    VStack::new()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("Features Demonstrated"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- NotificationBell with unread count badge"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- NotificationCenter panel with scrollable list"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Multiple notification variants (Info, Success, Warning, Error)"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Mark as read on click"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Dismiss individual notifications"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Mark all read / Clear all buttons"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Empty state when no notifications"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Custom icons and action buttons"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Show more indicator for many notifications"),
                                        ),
                                ),
                        ),
                ),
            )
            .when(show_panel, |d| {
                d.child(
                    div()
                        .absolute()
                        .top(px(80.0))
                        .right(px(32.0))
                        .child(
                            NotificationCenter::new(self.notification_state.clone())
                                .max_visible(8)
                                .show_timestamps(true)
                                .on_notification_click(|notification, _, _| {
                                    println!("Clicked notification: {:?}", notification.title);
                                }),
                        ),
                )
            })
    }
}
