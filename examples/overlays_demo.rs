use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    overlays::dialog::DialogSize,
    overlays::popover::{Popover, PopoverContent},
    overlays::toast::{ToastManager, ToastItem, ToastVariant, ToastPosition},
    components::button::{Button, ButtonVariant},
    layout::VStack,
    theme::use_theme,
};
use std::sync::atomic::{AtomicU64, Ordering};

actions!(overlays_demo, [Quit]);

static TOAST_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

struct OverlaysDemo {
    show_dialog: bool,
    dialog_size: DialogSize,
    toast_manager: Entity<ToastManager>,
}

impl OverlaysDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let toast_manager = cx.new(|cx| ToastManager::new(cx).position(ToastPosition::BottomRight));

        Self {
            show_dialog: false,
            dialog_size: DialogSize::Md,
            toast_manager,
        }
    }

    fn add_toast(&mut self, variant: ToastVariant, title: impl Into<SharedString>, description: Option<impl Into<SharedString>>, window: &mut Window, cx: &mut Context<Self>) {
        let id = TOAST_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut toast = ToastItem::new(id, title).variant(variant);

        if let Some(desc) = description {
            toast = toast.description(desc);
        }

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }
}

impl OverlaysDemo {
    fn close_dialog(&mut self, cx: &mut Context<Self>) {
        self.show_dialog = false;
        cx.notify();
    }

    fn confirm_dialog(&mut self, cx: &mut Context<Self>) {
        self.show_dialog = false;
        cx.notify();
    }
}

impl Render for OverlaysDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                    .child("Overlays Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Dialogs, Popovers, and Toast notifications")
                            )
                    )
                    // Content
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(48.0))
                            // Dialog Section
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
                                                    .child("Dialog")
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Modal dialogs with focus trap and backdrop")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(12.0))
                                            .flex_wrap()
                                            .child(
                                                Button::new("small-dialog-btn", "Small Dialog")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.dialog_size = DialogSize::Sm;
                                                        this.show_dialog = true;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                Button::new("medium-dialog-btn", "Medium Dialog")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.dialog_size = DialogSize::Md;
                                                        this.show_dialog = true;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                Button::new("large-dialog-btn", "Large Dialog")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.dialog_size = DialogSize::Lg;
                                                        this.show_dialog = true;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                Button::new("xl-dialog-btn", "XL Dialog")
                                                    .variant(ButtonVariant::Secondary)
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.dialog_size = DialogSize::Xl;
                                                        this.show_dialog = true;
                                                        cx.notify();
                                                    }))
                                            )
                                    )
                            )
                            // Popover Section
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
                                                    .child("Popover")
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Anchored popovers with smart positioning")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(12.0))
                                            .flex_wrap()
                                            .child(
                                                Popover::new("simple-popover")
                                                    .trigger(Button::new("simple-popover-btn", "Simple Popover").variant(ButtonVariant::Outline))
                                                    .content(|window, cx| {
                                                        cx.new(|cx| {
                                                            PopoverContent::new(window, cx, |_, _| {
                                                                div()
                                                                    .flex()
                                                                    .flex_col()
                                                                    .gap(px(8.0))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(14.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .child("Popover Title")
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(13.0))
                                                                            .child("This is popover content that anchors to the trigger element.")
                                                                    )
                                                                    .into_any_element()
                                                            })
                                                        })
                                                    })
                                            )
                                            .child(
                                                Popover::new("actions-popover")
                                                    .trigger(Button::new("with-actions-btn", "With Actions").variant(ButtonVariant::Outline))
                                                    .content(|window, cx| {
                                                        let theme = use_theme();
                                                        cx.new(|cx| {
                                                            PopoverContent::new(window, cx, move |_, _| {
                                                                div()
                                                                    .flex()
                                                                    .flex_col()
                                                                    .gap(px(12.0))
                                                                    .min_w(px(250.0))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(14.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .child("Quick Actions")
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .flex()
                                                                            .flex_col()
                                                                            .gap(px(4.0))
                                                                            .child(
                                                                                div()
                                                                                    .px(px(8.0))
                                                                                    .py(px(6.0))
                                                                                    .rounded(theme.tokens.radius_sm)
                                                                                    .cursor(CursorStyle::PointingHand)
                                                                                    .hover(|style| {
                                                                                        style.bg(theme.tokens.accent)
                                                                                    })
                                                                                    .child("Action 1")
                                                                            )
                                                                            .child(
                                                                                div()
                                                                                    .px(px(8.0))
                                                                                    .py(px(6.0))
                                                                                    .rounded(theme.tokens.radius_sm)
                                                                                    .cursor(CursorStyle::PointingHand)
                                                                                    .hover(|style| {
                                                                                        style.bg(theme.tokens.accent)
                                                                                    })
                                                                                    .child("Action 2")
                                                                            )
                                                                            .child(
                                                                                div()
                                                                                    .px(px(8.0))
                                                                                    .py(px(6.0))
                                                                                    .rounded(theme.tokens.radius_sm)
                                                                                    .cursor(CursorStyle::PointingHand)
                                                                                    .hover(|style| {
                                                                                        style.bg(theme.tokens.accent)
                                                                                    })
                                                                                    .child("Action 3")
                                                                            )
                                                                    )
                                                                    .into_any_element()
                                                            })
                                                        })
                                                    })
                                            )
                                    )
                            )
                            // Toast Section
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
                                                    .child("Toast Notifications")
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Auto-dismissing notifications with variants")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(12.0))
                                            .flex_wrap()
                                            .child(
                                                Button::new("default-toast-btn", "Default Toast")
                                                    .variant(ButtonVariant::Outline)
                                                    .on_click(cx.listener(|this, _, window, cx| {
                                                        this.add_toast(
                                                            ToastVariant::Default,
                                                            "Notification",
                                                            Some("This is a default toast notification"),
                                                            window,
                                                            cx
                                                        );
                                                    }))
                                            )
                                            .child(
                                                Button::new("success-toast-btn", "Success Toast")
                                                    .variant(ButtonVariant::Outline)
                                                    .on_click(cx.listener(|this, _, window, cx| {
                                                        this.add_toast(
                                                            ToastVariant::Success,
                                                            "Success!",
                                                            Some("Operation completed successfully"),
                                                            window,
                                                            cx
                                                        );
                                                    }))
                                            )
                                            .child(
                                                Button::new("warning-toast-btn", "Warning Toast")
                                                    .variant(ButtonVariant::Outline)
                                                    .on_click(cx.listener(|this, _, window, cx| {
                                                        this.add_toast(
                                                            ToastVariant::Warning,
                                                            "Warning",
                                                            Some("Please review this action"),
                                                            window,
                                                            cx
                                                        );
                                                    }))
                                            )
                                            .child(
                                                Button::new("error-toast-btn", "Error Toast")
                                                    .variant(ButtonVariant::Destructive)
                                                    .on_click(cx.listener(|this, _, window, cx| {
                                                        this.add_toast(
                                                            ToastVariant::Error,
                                                            "Error",
                                                            Some("Something went wrong"),
                                                            window,
                                                            cx
                                                        );
                                                    }))
                                            )
                                    )
                            )
                    )
            )
            // Dialog Overlay
            .when(self.show_dialog, |this| {
                let theme = use_theme();

                let dialog_width = match self.dialog_size {
                    DialogSize::Sm => px(400.0),
                    DialogSize::Md => px(500.0),
                    DialogSize::Lg => px(600.0),
                    DialogSize::Xl => px(800.0),
                    DialogSize::Full => px(950.0), // Use px instead of relative for consistency
                };

                // Backdrop
                this.child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(gpui::black().opacity(0.5))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.close_dialog(cx);
                        }))
                        .child(
                            div()
                                .occlude()
                                .w(dialog_width)
                                .max_h(relative(0.85))
                                .flex()
                                .flex_col()
                                .bg(theme.tokens.card)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .rounded(theme.tokens.radius_lg)
                                .shadow_xl()
                                .overflow_hidden()
                                .on_mouse_down(MouseButton::Left, |_, _, _| {
                                    // Stop propagation - clicking inside dialog shouldn't close it
                                })
                                // Header
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(8.0))
                                        .px(px(24.0))
                                        .pt(px(24.0))
                                        .pb(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .justify_between()
                                                .gap(px(16.0))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .gap(px(4.0))
                                                        .flex_1()
                                                        .child(
                                                            div()
                                                                .text_size(px(18.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .line_height(relative(1.2))
                                                                .child("Example Dialog")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .line_height(relative(1.5))
                                                                .child("This is an example dialog with various size options")
                                                        )
                                                )
                                                .child(
                                                    Button::new("close-btn", "×")
                                                        .variant(ButtonVariant::Ghost)
                                                        .on_click(cx.listener(|this, _, _, cx| {
                                                            this.close_dialog(cx);
                                                        }))
                                                )
                                        )
                                )
                                // Content
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(16.0))
                                        .px(px(24.0))
                                        .py(px(16.0))
                                        .flex_1()
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .child("Dialogs can contain any content you need. They include a backdrop, focus trap, and keyboard handling (press Escape to close).")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("The dialog will prevent interaction with content behind it until closed.")
                                        )
                                )
                                // Footer
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_end()
                                        .gap(px(8.0))
                                        .px(px(24.0))
                                        .py(px(16.0))
                                        .border_t_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            Button::new("cancel-btn", "Cancel")
                                                .variant(ButtonVariant::Outline)
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.close_dialog(cx);
                                                }))
                                        )
                                        .child(
                                            Button::new("confirm-btn", "Confirm")
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.confirm_dialog(cx);
                                                    this.add_toast(
                                                        ToastVariant::Success,
                                                        "Dialog confirmed",
                                                        Some("You clicked the confirm button"),
                                                        window,
                                                        cx
                                                    );
                                                }))
                                        )
                                )
                        )
                )
            })
            // Toast Manager
            .child(self.toast_manager.clone())
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
                    size(px(1000.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Overlays Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| OverlaysDemo::new(cx)),
        )
        .unwrap();
    });
}
