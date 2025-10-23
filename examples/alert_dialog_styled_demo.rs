use adabraka_ui::{
    prelude::*,
    overlays::alert_dialog::AlertDialog,
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

            // Example 1: Default Alert Dialog
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Alert Dialog #1: Default Styling".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(50.0), px(50.0)),
                        size: size(px(600.0), px(400.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        AlertDialog::new(cx)
                            .title("Delete Account")
                            .description("This action cannot be undone. This will permanently delete your account.")
                            .cancel_text("Cancel")
                            .action_text("Delete")
                            .destructive(true)
                            .on_cancel(|_, cx| cx.quit())
                            .on_action(|_, cx| cx.quit())
                    })
                },
            )
            .unwrap();

            // Example 2: Custom Background Color
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Alert Dialog #2: Custom Background".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(700.0), px(50.0)),
                        size: size(px(600.0), px(400.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        AlertDialog::new(cx)
                            .title("Custom Styled Dialog")
                            .description("This alert dialog has a custom purple background using Styled trait.")
                            .cancel_text("No")
                            .action_text("Yes")
                            .bg(rgb(0x8b5cf6))  // Styled trait - purple
                            .on_cancel(|_, cx| cx.quit())
                            .on_action(|_, cx| cx.quit())
                    })
                },
            )
            .unwrap();

            // Example 3: Custom Border
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Alert Dialog #3: Custom Border".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(1350.0), px(50.0)),
                        size: size(px(600.0), px(400.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        AlertDialog::new(cx)
                            .title("Save Changes?")
                            .description("You have unsaved changes. Do you want to save them before closing?")
                            .cancel_text("Don't Save")
                            .action_text("Save")
                            .border_3()  // Styled trait
                            .border_color(rgb(0x3b82f6))  // Styled trait - blue
                            .on_cancel(|_, cx| cx.quit())
                            .on_action(|_, cx| cx.quit())
                    })
                },
            )
            .unwrap();

            // Example 4: Custom Border Radius
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Alert Dialog #4: Square Corners".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(50.0), px(500.0)),
                        size: size(px(600.0), px(400.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        AlertDialog::new(cx)
                            .title("Confirm Action")
                            .description("Are you sure you want to proceed with this action?")
                            .cancel_text("Cancel")
                            .action_text("Proceed")
                            .rounded(px(0.0))  // Styled trait - no border radius
                            .on_cancel(|_, cx| cx.quit())
                            .on_action(|_, cx| cx.quit())
                    })
                },
            )
            .unwrap();

            // Example 5: Combined Styling
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Alert Dialog #5: Combined Styling".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(700.0), px(500.0)),
                        size: size(px(700.0), px(500.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        AlertDialog::new(cx)
                            .title("Ultra Custom Alert")
                            .description("This alert combines multiple Styled trait methods for a unique look.")
                            .cancel_text("Cancel")
                            .action_text("Confirm")
                            .destructive(false)
                            .bg(rgb(0xf59e0b))  // Styled trait - orange
                            .border_4()  // Styled trait
                            .border_color(rgb(0x10b981))  // Styled trait - green
                            .rounded(px(24.0))  // Styled trait - large radius
                            .p(px(32.0))  // Styled trait - extra padding
                            .on_cancel(|_, cx| cx.quit())
                            .on_action(|_, cx| cx.quit())
                    })
                },
            )
            .unwrap();
        });
}
