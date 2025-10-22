use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
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
                        title: Some("Separator Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| SeparatorStyledDemo::new()),
            )
            .unwrap();
        });
}

struct SeparatorStyledDemo;

impl SeparatorStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for SeparatorStyledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Separator Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Default Separators
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default Separators")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(Separator::new())
                            .child(div().child("Content below"))
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .h(px(100.0))
                            .child(div().child("Left content"))
                            .child(Separator::vertical())
                            .child(div().child("Right content"))
                    )
            )
            // 2. Custom Margins
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Margins (via Styled trait)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .my(px(32.0))  // Styled trait method
                            )
                            .child(div().child("Content below with larger margin"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .mx(px(48.0))  // Styled trait method
                            )
                            .child(div().child("Content below with horizontal inset"))
                    )
            )
            // 3. Custom Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Width Control")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .w(px(400.0))  // Styled trait
                            )
                            .child(div().child("Separator with fixed width"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .w_1_2()  // Styled trait - 50% width
                            )
                            .child(div().child("Separator with 50% width"))
                    )
            )
            // 4. With Labels and Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Labels with Custom Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .label("OR")
                                    .my(px(16.0))  // Styled trait
                            )
                            .child(div().child("Content below"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .label("SECTION")
                                    .my(px(24.0))  // Styled trait
                                    .px(px(32.0))  // Styled trait
                            )
                            .child(div().child("Content below with padding"))
                    )
            )
            // 5. Custom Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Colors")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .color(rgb(0x3b82f6))  // Blue
                                    .my(px(16.0))
                            )
                            .child(div().child("Blue separator"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .color(rgb(0x10b981))  // Green
                                    .my(px(16.0))
                            )
                            .child(div().child("Green separator"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .color(rgb(0xef4444))  // Red
                                    .label("WARNING")
                                    .my(px(16.0))
                            )
                            .child(div().child("Red separator with label"))
                    )
            )
            // 6. Opacity Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Opacity Control (via Styled trait)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .opacity(0.3)  // Styled trait
                                    .my(px(16.0))
                            )
                            .child(div().child("30% opacity"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .opacity(0.6)  // Styled trait
                                    .my(px(16.0))
                            )
                            .child(div().child("60% opacity"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .opacity(1.0)  // Styled trait
                                    .my(px(16.0))
                            )
                            .child(div().child("100% opacity"))
                    )
            )
            // 7. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .label("STYLED SEPARATOR")
                                    .color(rgb(0x8b5cf6))  // Purple
                                    .my(px(32.0))  // Styled trait
                                    .px(px(48.0))  // Styled trait
                                    .w_full()  // Styled trait
                            )
                            .child(div().child("Fully customized separator"))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(div().child("Content above"))
                            .child(
                                Separator::new()
                                    .label("PREMIUM SECTION")
                                    .color(rgb(0xf59e0b))  // Amber
                                    .my(px(24.0))  // Styled trait
                                    .w(px(600.0))  // Styled trait
                                    .opacity(0.8)  // Styled trait
                            )
                            .child(div().child("Combined: custom width, color, spacing, and opacity"))
                    )
            )
            // 8. Vertical Separators with Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Vertical Separators with Custom Styling")
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .h(px(120.0))
                            .items_center()
                            .child(div().child("Left section"))
                            .child(
                                Separator::vertical()
                                    .mx(px(24.0))  // Styled trait
                            )
                            .child(div().child("Middle section"))
                            .child(
                                Separator::vertical()
                                    .color(rgb(0x3b82f6))
                                    .mx(px(24.0))  // Styled trait
                                    .h(px(80.0))  // Styled trait - custom height
                            )
                            .child(div().child("Right section"))
                    )
            )
            // Info Box
            .child(
                div()
                    .mt(px(16.0))
                    .p(px(16.0))
                    .bg(theme.tokens.accent)
                    .rounded(px(8.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .my(), .mx(), .w(), .w_full(), .w_1_2(), .h(), .opacity(), plus native methods like .color() and .label()")
                    )
            )
                )
            )
    }
}
