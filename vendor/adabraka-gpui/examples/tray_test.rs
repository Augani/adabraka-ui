use gpui::{App, Application, TrayMenuItem};

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_keep_alive_without_windows(true);

        cx.set_tray_tooltip("Test Tray App");

        cx.set_tray_menu(vec![
            TrayMenuItem::Action {
                label: "Hello".into(),
                id: "hello".into(),
            },
            TrayMenuItem::Separator,
            TrayMenuItem::Action {
                label: "Quit".into(),
                id: "quit".into(),
            },
        ]);

        cx.on_tray_menu_action(|id, cx| {
            eprintln!("Menu action: {}", id);
            if id.as_ref() == "quit" {
                cx.quit();
            }
        });

        eprintln!("Tray should be visible now.");
    });
}
