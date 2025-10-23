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
                        title: Some("TextField Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| TextFieldStyledDemo::new()),
            )
            .unwrap();
        });
}

struct TextFieldStyledDemo;

impl TextFieldStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for TextFieldStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child("TextField Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Custom Width Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Width (via Styled trait)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Default width")
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom width 300px")
                                    .w(px(300.0))  // <- Styled trait method
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom width 500px")
                                    .w(px(500.0))  // <- Styled trait method
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Full width")
                                    .w_full()  // <- Styled trait method
                            )
                    )
            )
            // 2. Custom Padding Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Padding")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Default padding")
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom p_4()")
                                    .p_4()  // <- Styled trait method
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom p_8()")
                                    .p_8()  // <- Styled trait method
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom px(32)")
                                    .px(px(32.0))  // <- Styled trait method
                                    .w(px(400.0))
                            )
                    )
            )
            // 3. Custom Background Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Background Colors")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Blue background")
                                    .bg(rgb(0x1e3a5f))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Purple background")
                                    .bg(rgb(0x3d2d5f))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Green background")
                                    .bg(rgb(0x1a4d2e))  // <- Styled trait
                                    .w(px(400.0))
                            )
                    )
            )
            // 4. Custom Border Styles
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Borders")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Border 2px")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Red border")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0xef4444))
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Purple thick border")
                                    .border_4()  // <- Styled trait
                                    .border_color(rgb(0x8b5cf6))
                                    .w(px(400.0))
                            )
                    )
            )
            // 5. Custom Border Radius
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Border Radius")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("No radius")
                                    .rounded(px(0.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Large radius")
                                    .rounded(px(16.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Pill shape")
                                    .rounded(px(999.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                    )
            )
            // 6. Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Shadow Effects")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Shadow small")
                                    .shadow_sm()  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Shadow medium")
                                    .shadow_md()  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Shadow large")
                                    .shadow_lg()  // <- Styled trait
                                    .w(px(400.0))
                            )
                    )
            )
            // 7. Height Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Height Control")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Small height (size variant)")
                                    .size(TextFieldSize::Sm)
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom height 60px")
                                    .h(px(60.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom height 80px")
                                    .h(px(80.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                    )
            )
            // 8. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Custom blue input with shadow")
                                    .bg(rgb(0x1e3a5f))  // <- Styled trait
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .rounded(px(12.0))  // <- Styled trait
                                    .shadow_md()  // <- Styled trait
                                    .p(px(16.0))  // <- Styled trait
                                    .w_full()  // <- Styled trait
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Purple pill input with large padding")
                                    .bg(rgb(0x3d2d5f))  // <- Styled trait
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x8b5cf6))
                                    .rounded(px(999.0))  // <- Styled trait
                                    .px(px(32.0))  // <- Styled trait
                                    .py(px(16.0))  // <- Styled trait
                                    .w(px(500.0))  // <- Styled trait
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Ultra custom styled input")
                                    .bg(rgb(0x1a4d2e))  // <- Styled trait
                                    .border_4()  // <- Styled trait
                                    .border_color(rgb(0x10b981))
                                    .rounded(px(8.0))  // <- Styled trait
                                    .shadow_lg()  // <- Styled trait
                                    .p(px(20.0))  // <- Styled trait
                                    .h(px(70.0))  // <- Styled trait
                                    .w_full()  // <- Styled trait
                            )
                    )
            )
            // 9. Different Sizes with Custom Styles
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("9. Size Variants with Custom Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Small with custom border")
                                    .size(TextFieldSize::Sm)
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Medium with custom background")
                                    .size(TextFieldSize::Md)
                                    .bg(rgb(0x1e3a5f))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Large with shadow")
                                    .size(TextFieldSize::Lg)
                                    .shadow_lg()  // <- Styled trait
                                    .w(px(400.0))
                            )
                    )
            )
            // 10. State Combinations
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("10. State Variants with Custom Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                TextField::new(cx)
                                    .placeholder("Normal state with custom style")
                                    .bg(rgb(0x1e3a5f))  // <- Styled trait
                                    .rounded(px(8.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Disabled state with custom style")
                                    .disabled(true)
                                    .bg(rgb(0x1e3a5f))  // <- Styled trait
                                    .rounded(px(8.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
                            .child(
                                TextField::new(cx)
                                    .placeholder("Invalid state with custom border")
                                    .invalid(true)
                                    .border_4()  // <- Styled trait
                                    .rounded(px(12.0))  // <- Styled trait
                                    .w(px(400.0))
                            )
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
                            .child("Methods used: .w(), .w_full(), .p_4(), .p_8(), .px(), .py(), .bg(), .border_2(), .border_4(), .rounded(), .shadow_sm/md/lg(), .h()")
                    )
            )
                )
            )
    }
}
