use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    components::slider::Slider,
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
                        title: Some("Slider Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| SliderStyledDemo::new()),
            )
            .unwrap();
        });
}

struct SliderStyledDemo {
    value1: f32,
    value2: f32,
    value3: f32,
    value4: f32,
    value5: f32,
    value6: f32,
    value7: f32,
}

impl SliderStyledDemo {
    fn new() -> Self {
        Self {
            value1: 50.0,
            value2: 30.0,
            value3: 70.0,
            value4: 45.0,
            value5: 25.0,
            value6: 60.0,
            value7: 80.0,
        }
    }
}

impl Render for SliderStyledDemo {
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
                            .child("Slider Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Default Slider
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default Slider")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value1))
                    )
                    .child(
                        Slider::new("default-slider")
                            .value(self.value1)
                    )
            )
            // 2. Custom Width
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Width")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value2))
                    )
                    .child(
                        Slider::new("width-slider")
                            .value(self.value2)
                            .w(px(400.0))  // Custom width via Styled trait
                    )
            )
            // 3. Custom Background & Padding
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Background & Padding")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value3))
                    )
                    .child(
                        Slider::new("bg-slider")
                            .value(self.value3)
                            .bg(rgb(0x1e293b))  // Dark slate background via Styled trait
                            .p(px(16.0))  // Custom padding via Styled trait
                    )
            )
            // 4. Custom Border Radius
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Border Radius")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value4))
                    )
                    .child(
                        Slider::new("rounded-slider")
                            .value(self.value4)
                            .rounded(px(20.0))  // Large border radius via Styled trait
                    )
            )
            // 5. Custom Shadow
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Shadow")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value5))
                    )
                    .child(
                        Slider::new("shadow-slider")
                            .value(self.value5)
                            .shadow_lg()  // Large shadow via Styled trait
                    )
            )
            // 6. Custom Border
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Custom Border")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value6))
                    )
                    .child(
                        Slider::new("border-slider")
                            .value(self.value6)
                            .border_2()  // Border via Styled trait
                            .border_color(rgb(0x3b82f6))  // Blue border
                            .p(px(8.0))  // Padding to show border better
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
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Value: {:.0}", self.value7))
                    )
                    .child(
                        Slider::new("combined-slider")
                            .value(self.value7)
                            .w(px(600.0))  // Custom width
                            .h(px(40.0))  // Custom height
                            .bg(rgb(0x0f172a))  // Dark background
                            .p(px(12.0))  // Custom padding
                            .rounded(px(24.0))  // Large border radius
                            .border_2()  // Border
                            .border_color(rgb(0x8b5cf6))  // Purple border
                            .shadow_md()  // Medium shadow
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
                            .child("✅ All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .w(), .h(), .bg(), .p(), .rounded(), .border_2(), .border_color(), .shadow_lg(), .shadow_md()")
                    )
            )
                )
            )
    }
}
