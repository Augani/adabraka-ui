use gpui::single_instance::{send_activate_to_existing, SingleInstance};
use gpui::{
    div, prelude::*, px, rgb, rgba, size, App, Application, Bounds, Context, Entity, Keystroke,
    Toast, ToastPosition, ToastStack, TrayIconEvent, TrayMenuItem, Window,
    WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
};

const APP_ID: &str = "com.example.daemon-app";

struct OverlayView {
    toast_stack: Entity<ToastStack>,
}

impl Render for OverlayView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_6()
                    .rounded(px(12.0))
                    .bg(rgba(0x000000dd))
                    .text_color(rgb(0xffffff))
                    .shadow_lg()
                    .max_w(px(400.0))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Daemon App Overlay"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgba(0xffffffaa))
                            .child("This overlay window is always on top."),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgba(0xffffffaa))
                            .child("Uses WindowKind::Overlay + transparent background."),
                    ),
            )
            .child(self.toast_stack.clone())
    }
}

struct SettingsView;

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .size_full()
            .bg(rgb(0xfafafa))
            .text_color(rgb(0x333333))
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("Settings"),
            )
            .child(
                div()
                    .text_sm()
                    .child("This is a normal settings window opened from the tray menu."),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x888888))
                    .child(format!("Application ID: {}", APP_ID)),
            )
    }
}

fn main() {
    let _instance = match SingleInstance::acquire(APP_ID) {
        Ok(instance) => instance,
        Err(_) => {
            eprintln!("Another instance is already running. Sending activation signal.");
            let _ = send_activate_to_existing(APP_ID);
            std::process::exit(0);
        }
    };

    Application::new().run(|cx: &mut App| {
        cx.set_keep_alive_without_windows(true);

        setup_tray(cx);
        setup_global_hotkey(cx);

        let _ = cx.show_notification("Daemon App", "Application started in background");

        cx.activate(true);
    });
}

fn setup_tray(cx: &mut App) {
    cx.set_tray_tooltip("Daemon App");

    cx.set_tray_menu(vec![
        TrayMenuItem::Action {
            label: "Show Overlay".into(),
            id: "show_overlay".into(),
        },
        TrayMenuItem::Action {
            label: "Settings".into(),
            id: "settings".into(),
        },
        TrayMenuItem::Separator,
        TrayMenuItem::Action {
            label: "Quit".into(),
            id: "quit".into(),
        },
    ]);

    cx.on_tray_icon_event(|event, _cx| match event {
        TrayIconEvent::LeftClick => {
            eprintln!("Tray icon left-clicked");
        }
        TrayIconEvent::RightClick => {
            eprintln!("Tray icon right-clicked");
        }
        TrayIconEvent::DoubleClick => {
            eprintln!("Tray icon double-clicked");
        }
    });

    cx.on_tray_menu_action(|id, cx| match id.as_ref() {
        "show_overlay" => {
            open_overlay(cx);
            cx.activate(true);
        }
        "settings" => {
            open_settings(cx);
            cx.activate(true);
        }
        "quit" => {
            cx.quit();
        }
        _ => {}
    });
}

fn setup_global_hotkey(cx: &mut App) {
    let keystroke = Keystroke::parse("cmd-shift-k").expect("valid keystroke");
    if let Err(err) = cx.register_global_hotkey(1, &keystroke) {
        eprintln!("Failed to register global hotkey: {}", err);
    }

    cx.on_global_hotkey(move |id| {
        if id == 1 {
            eprintln!("Global hotkey triggered (Cmd+Shift+K)");
        }
    });
}

fn open_overlay(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(500.), px(300.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            kind: WindowKind::Overlay,
            titlebar: None,
            focus: true,
            show: true,
            window_background: WindowBackgroundAppearance::Transparent,
            ..Default::default()
        },
        |window, cx| {
            let toast_stack = cx.new(|_| ToastStack::new().with_position(ToastPosition::TopRight));
            let toast_stack_handle = toast_stack.clone();

            cx.new(|cx| {
                toast_stack_handle.update(cx, |stack, cx| {
                    stack.push(
                        Toast::new("Overlay Opened").body("This toast auto-dismisses in 3 seconds"),
                        window,
                        cx,
                    );
                });

                OverlayView { toast_stack }
            })
        },
    )
    .ok();
}

fn open_settings(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            kind: WindowKind::Normal,
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Daemon App Settings".into()),
                ..Default::default()
            }),
            focus: true,
            show: true,
            ..Default::default()
        },
        |_, cx| cx.new(|_| SettingsView),
    )
    .ok();
}
