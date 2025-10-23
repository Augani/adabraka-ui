use adabraka_ui::{
    prelude::*,
    overlays::sheet::{Sheet, SheetSide, SheetSize},
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

            // Example 1: Default Sheet (Right Side)
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Sheet #1: Default (Right)".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(50.0), px(50.0)),
                        size: size(px(800.0), px(600.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        Sheet::new(cx)
                            .side(SheetSide::Right)
                            .size(SheetSize::Md)
                            .title("Default Sheet")
                            .description("This sheet uses default styling.")
                            .content(
                                div()
                                    .p(px(24.0))
                                    .child("Sheet content with default styling")
                                    .child(div().mt(px(12.0)).child("Appears from the right side"))
                            )
                            .footer(
                                Button::new("ok1", "OK")
                                    .variant(ButtonVariant::Default)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 2: Custom Background (Left Side)
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Sheet #2: Custom Background (Left)".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(900.0), px(50.0)),
                        size: size(px(800.0), px(600.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        Sheet::new(cx)
                            .side(SheetSide::Left)
                            .title("Custom Background")
                            .description("Sheet with blue background using Styled trait.")
                            .content(
                                div()
                                    .p(px(24.0))
                                    .child("Custom blue background applied via .bg()!")
                                    .child(div().mt(px(12.0)).child("Appears from the left side"))
                            )
                            .bg(rgb(0x3b82f6))  // Styled trait
                            .footer(
                                Button::new("ok2", "OK")
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 3: Custom Border (Bottom)
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Sheet #3: Custom Border (Bottom)".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(50.0), px(700.0)),
                        size: size(px(800.0), px(600.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        Sheet::new(cx)
                            .side(SheetSide::Bottom)
                            .title("Custom Border")
                            .description("Sheet with thick purple border using Styled trait.")
                            .content(
                                div()
                                    .p(px(24.0))
                                    .child("4px purple border applied via .border_4() and .border_color()!")
                                    .child(div().mt(px(12.0)).child("Appears from the bottom"))
                            )
                            .border_4()  // Styled trait
                            .border_color(rgb(0x8b5cf6))  // Styled trait
                            .footer(
                                Button::new("ok3", "OK")
                                    .variant(ButtonVariant::Default)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 4: Custom Border Radius (Top)
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Sheet #4: Rounded Corners (Top)".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(900.0), px(700.0)),
                        size: size(px(800.0), px(600.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        Sheet::new(cx)
                            .side(SheetSide::Top)
                            .title("Rounded Corners")
                            .description("Sheet with large border radius using Styled trait.")
                            .content(
                                div()
                                    .p(px(24.0))
                                    .child("Large border radius applied via .rounded(px(24.0))!")
                                    .child(div().mt(px(12.0)).child("Appears from the top"))
                            )
                            .rounded(px(24.0))  // Styled trait
                            .footer(
                                Button::new("ok4", "OK")
                                    .variant(ButtonVariant::Default)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();

            // Example 5: Combined Styling (Right)
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Sheet #5: Combined Styling".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(1750.0), px(50.0)),
                        size: size(px(900.0), px(700.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        Sheet::new(cx)
                            .side(SheetSide::Right)
                            .size(SheetSize::Lg)
                            .title("Ultra Custom Sheet")
                            .description("Sheet combining multiple Styled trait methods.")
                            .content(
                                div()
                                    .p(px(24.0))
                                    .child("This sheet combines multiple Styled trait methods:")
                                    .child(div().mt(px(12.0)).child("- Green background via .bg()"))
                                    .child(div().child("- 3px orange border via .border_3() and .border_color()"))
                                    .child(div().child("- Large border radius via .rounded()"))
                                    .child(div().child("- Large shadow via .shadow_lg()"))
                            )
                            .bg(rgb(0x10b981))  // Styled trait - green
                            .border_3()  // Styled trait
                            .border_color(rgb(0xf59e0b))  // Styled trait - orange
                            .rounded(px(20.0))  // Styled trait
                            .shadow_lg()  // Styled trait
                            .footer(
                                Button::new("ok5", "Amazing!")
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(|_, _, cx| cx.quit())
                            )
                    })
                },
            )
            .unwrap();
        });
}
