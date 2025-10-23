use adabraka_ui::{
    prelude::*,
    overlays::dialog::{Dialog, DialogSize},
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

            // Example 1: Default Dialog
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Dialog #1: Default Styling".into()),
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
                        Dialog::new(cx)
                            .title("Default Dialog")
                            .description("This dialog uses default styling.")
                            .child(div().p(px(8.0)).child("This is the dialog content with default styling."))
                            .footer(
                                Button::new("ok1", "OK")
                                    .variant(ButtonVariant::Default)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 2: Custom Background Color
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Dialog #2: Custom Background".into()),
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
                        Dialog::new(cx)
                            .title("Custom Background")
                            .description("Dialog with purple background using Styled trait.")
                            .child(div().p(px(8.0)).child("Custom purple background applied via .bg()!"))
                            .bg(rgb(0x8b5cf6))  // Styled trait
                            .footer(
                                Button::new("ok2", "OK")
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 3: Custom Border
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Dialog #3: Custom Border".into()),
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
                        Dialog::new(cx)
                            .title("Custom Border")
                            .description("Dialog with thick blue border using Styled trait.")
                            .child(div().p(px(8.0)).child("3px blue border applied via .border_3() and .border_color()!"))
                            .border_3()  // Styled trait
                            .border_color(rgb(0x3b82f6))  // Styled trait
                            .footer(
                                Button::new("ok3", "OK")
                                    .variant(ButtonVariant::Default)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 4: Custom Border Radius
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Dialog #4: No Border Radius".into()),
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
                        Dialog::new(cx)
                            .title("Square Corners")
                            .description("Dialog with no border radius using Styled trait.")
                            .child(div().p(px(8.0)).child("Square corners applied via .rounded(px(0.0))!"))
                            .rounded(px(0.0))  // Styled trait
                            .footer(
                                Button::new("ok4", "OK")
                                    .variant(ButtonVariant::Default)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 5: Combined Styling
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Dialog #5: Combined Styling".into()),
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
                        Dialog::new(cx)
                            .title("Ultra Custom Dialog")
                            .description("Dialog combining multiple Styled trait methods.")
                            .size(DialogSize::Lg)
                            .child(
                                div()
                                    .p(px(8.0))
                                    .child("This dialog combines multiple Styled trait methods:")
                                    .child(div().mt(px(12.0)).child("- Orange background via .bg()"))
                                    .child(div().child("- 4px green border via .border_4() and .border_color()"))
                                    .child(div().child("- Large border radius via .rounded()"))
                                    .child(div().child("- Custom padding via .p()"))
                            )
                            .bg(rgb(0xf59e0b))  // Styled trait - orange
                            .border_4()  // Styled trait
                            .border_color(rgb(0x10b981))  // Styled trait - green
                            .rounded(px(24.0))  // Styled trait
                            .p(px(32.0))  // Styled trait
                            .footer(
                                Button::new("ok5", "Awesome!")
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();
        });
}
