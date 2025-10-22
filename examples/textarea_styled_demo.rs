use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    components::textarea::Textarea,
    components::input::InputVariant,
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
                        title: Some("Textarea Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_cx| TextareaStyledDemo::new()),
            )
            .unwrap();
        });
}

struct TextareaStyledDemo {}

impl TextareaStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for TextareaStyledDemo {
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
                            .child("Textarea Styled Trait Customization Demo")
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
                                Textarea::new("textarea-1")
                                    .placeholder("Default width textarea")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                            )
                            .child(
                                Textarea::new("textarea-2")
                                    .placeholder("Full width textarea")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                                    .w_full()  // Styled trait method
                            )
                            .child(
                                Textarea::new("textarea-3")
                                    .placeholder("Custom width (600px)")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                                    .w(px(600.0))  // Styled trait method
                            )
                    )
            )
            // 2. Custom Padding
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
                                Textarea::new("textarea-4")
                                    .placeholder("Default padding")
                                    .variant(InputVariant::Outline)
                                    .rows(3)
                            )
                            .child(
                                Textarea::new("textarea-5")
                                    .placeholder("Extra padding - p_4()")
                                    .variant(InputVariant::Outline)
                                    .rows(3)
                                    .p_4()  // Styled trait method
                            )
                            .child(
                                Textarea::new("textarea-6")
                                    .placeholder("More padding - p_8()")
                                    .variant(InputVariant::Outline)
                                    .rows(3)
                                    .p_8()  // Styled trait method
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
                                Textarea::new("textarea-7")
                                    .placeholder("Blue background textarea")
                                    .variant(InputVariant::Ghost)
                                    .rows(4)
                                    .bg(rgb(0x1e3a8a))  // Styled trait
                                    .text_color(gpui::white())
                            )
                            .child(
                                Textarea::new("textarea-8")
                                    .placeholder("Purple background textarea")
                                    .variant(InputVariant::Ghost)
                                    .rows(4)
                                    .bg(rgb(0x581c87))  // Styled trait
                                    .text_color(gpui::white())
                            )
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Textarea::new("textarea-9")
                                    .placeholder("No radius (sharp corners)")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                                    .rounded(px(0.0))  // Styled trait
                            )
                            .child(
                                Textarea::new("textarea-10")
                                    .placeholder("Large radius (20px)")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                                    .rounded(px(20.0))  // Styled trait
                            )
                    )
            )
            // 5. Custom Height and Rows
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Height and Rows")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Textarea::new("textarea-11")
                                    .placeholder("Small textarea (2 rows)")
                                    .variant(InputVariant::Default)
                                    .rows(2)
                            )
                            .child(
                                Textarea::new("textarea-12")
                                    .placeholder("Large textarea (6 rows)")
                                    .variant(InputVariant::Default)
                                    .rows(6)
                            )
                            .child(
                                Textarea::new("textarea-13")
                                    .placeholder("Custom min height via Styled trait")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                                    .min_h(px(150.0))  // Styled trait method
                            )
                    )
            )
            // 6. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Textarea::new("textarea-14")
                                    .placeholder("Fully customized textarea with shadow and gradient")
                                    .variant(InputVariant::Ghost)
                                    .rows(4)
                                    .w_full()  // Styled trait
                                    .p(px(20.0))  // Styled trait
                                    .bg(rgb(0x047857))  // Styled trait
                                    .text_color(gpui::white())
                                    .rounded(px(12.0))  // Styled trait
                                    .shadow_lg()  // Styled trait
                            )
                            .child(
                                Textarea::new("textarea-15")
                                    .placeholder("Ultra custom with border and special styling")
                                    .variant(InputVariant::Outline)
                                    .rows(5)
                                    .w_full()  // Styled trait
                                    .px(px(24.0))  // Styled trait
                                    .py(px(16.0))  // Styled trait
                                    .bg(hsla(43.0 / 360.0, 0.96, 0.56, 0.2))  // Styled trait
                                    .rounded(px(16.0))  // Styled trait
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0xfbbf24))
                            )
                    )
            )
            // 7. Different Variants with Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Different Variants with Custom Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Textarea::new("textarea-16")
                                    .placeholder("Default variant with custom width")
                                    .variant(InputVariant::Default)
                                    .rows(3)
                                    .w(px(500.0))
                            )
                            .child(
                                Textarea::new("textarea-17")
                                    .placeholder("Outline variant with custom background")
                                    .variant(InputVariant::Outline)
                                    .rows(3)
                                    .bg(hsla(0.0, 0.0, 0.95, 1.0))
                            )
                            .child(
                                Textarea::new("textarea-18")
                                    .placeholder("Ghost variant with custom border")
                                    .variant(InputVariant::Ghost)
                                    .rows(3)
                                    .border_1()
                                    .border_color(theme.tokens.primary)
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
                            .child("Methods used: .w_full(), .w(), .p_4(), .p_8(), .p(), .px(), .py(), .bg(), .rounded(), .border_1(), .border_2(), .shadow_lg(), .min_h()")
                    )
            )
                )
            )
    }
}
