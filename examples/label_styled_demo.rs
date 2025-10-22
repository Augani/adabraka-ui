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
                        title: Some("Label Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| LabelStyledDemo::new()),
            )
            .unwrap();
        });
}

struct LabelStyledDemo;

impl LabelStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for LabelStyledDemo {
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
                            .child("Label Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Custom Padding Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Padding (via Styled trait)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Label::new("Default Label")
                                    .helper_text("No custom styling applied")
                            )
                            .child(
                                Label::new("Custom p_4() Label")
                                    .helper_text("With p_4() padding")
                                    .p_4()  // Styled trait method
                            )
                            .child(
                                Label::new("Custom px(32) Label")
                                    .helper_text("With px(32.0) horizontal padding")
                                    .px(px(32.0))  // Styled trait method
                            )
                    )
            )
            // 2. Custom Background Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Background Colors")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Label::new("Blue Background Label")
                                    .helper_text("With custom blue background")
                                    .bg(rgb(0x3b82f6))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                            .child(
                                Label::new("Purple Background Label")
                                    .helper_text("With custom purple background")
                                    .bg(rgb(0x8b5cf6))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                            .child(
                                Label::new("Green Background Label")
                                    .required(true)
                                    .helper_text("Required field with green background")
                                    .bg(rgb(0x10b981))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                    )
            )
            // 3. Custom Borders
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Borders")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Label::new("Blue Border Label")
                                    .helper_text("With 2px blue border")
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                            .child(
                                Label::new("Red Border Label")
                                    .required(true)
                                    .helper_text("Required field with red border")
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0xef4444))
                                    .p_4()
                                    .rounded(px(8.0))
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
                                Label::new("No Radius Label")
                                    .helper_text("Rounded(0) - sharp corners")
                                    .bg(theme.tokens.accent)
                                    .p_4()
                                    .rounded(px(0.0))  // Styled trait
                            )
                            .child(
                                Label::new("Large Radius Label")
                                    .helper_text("Rounded(16) - large corners")
                                    .bg(theme.tokens.accent)
                                    .p_4()
                                    .rounded(px(16.0))  // Styled trait
                            )
                            .child(
                                Label::new("Pill Shape Label")
                                    .helper_text("Rounded(999) - pill shape")
                                    .bg(theme.tokens.accent)
                                    .px(px(20.0))
                                    .py(px(8.0))
                                    .rounded(px(999.0))  // Styled trait
                            )
                    )
            )
            // 5. Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Width Control")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Label::new("Full Width Label")
                                    .helper_text("With w_full() applied")
                                    .w_full()  // Styled trait
                                    .bg(theme.tokens.accent)
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                            .child(
                                Label::new("Custom Width Label")
                                    .required(true)
                                    .helper_text("With w(px(400.0)) applied")
                                    .w(px(400.0))  // Styled trait
                                    .bg(theme.tokens.secondary)
                                    .p_4()
                                    .rounded(px(8.0))
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
                            .gap(px(16.0))
                            .child(
                                Label::new("Shadow Small")
                                    .helper_text("With shadow_sm() applied")
                                    .bg(theme.tokens.background)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_sm()  // Styled trait
                            )
                            .child(
                                Label::new("Shadow Medium")
                                    .helper_text("With shadow_md() applied")
                                    .bg(theme.tokens.background)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_md()  // Styled trait
                            )
                            .child(
                                Label::new("Shadow Large")
                                    .required(true)
                                    .helper_text("With shadow_lg() applied")
                                    .bg(theme.tokens.background)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_lg()  // Styled trait
                            )
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
                            .gap(px(16.0))
                            .child(
                                Label::new("Purple Card Label")
                                    .helper_text("Combined: p_8, rounded, bg, shadow_lg")
                                    .p_8()  // Styled trait
                                    .rounded(px(12.0))  // Styled trait
                                    .bg(rgb(0x8b5cf6))  // Styled trait
                                    .text_color(gpui::white())
                                    .shadow_lg()  // Styled trait
                            )
                            .child(
                                Label::new("Full Width Custom Card")
                                    .required(true)
                                    .helper_text("Combined: w_full, p, border_2, rounded")
                                    .w_full()  // Styled trait
                                    .p(px(20.0))  // Styled trait
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0x10b981))
                                    .rounded(px(12.0))  // Styled trait
                                    .bg(theme.tokens.background)
                            )
                            .child(
                                Label::new("Ultra Custom Label")
                                    .helper_text("Combined: px, py, bg, rounded, shadow_md, w")
                                    .px(px(40.0))  // Styled trait
                                    .py(px(16.0))  // Styled trait
                                    .bg(rgb(0xf59e0b))  // Styled trait
                                    .text_color(gpui::white())
                                    .rounded(px(8.0))  // Styled trait
                                    .shadow_md()  // Styled trait
                                    .w(px(500.0))  // Styled trait
                            )
                    )
            )
            // 8. Disabled State with Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Disabled State with Custom Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Label::new("Disabled Label")
                                    .disabled(true)
                                    .helper_text("Disabled with custom background")
                                    .bg(theme.tokens.muted)
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                            .child(
                                Label::new("Disabled Required Field")
                                    .disabled(true)
                                    .required(true)
                                    .helper_text("Disabled, required, with border")
                                    .border_2()
                                    .border_color(theme.tokens.border)
                                    .p_4()
                                    .rounded(px(8.0))
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
                            .child("Methods used: .p_4(), .p_8(), .px(), .py(), .bg(), .border_2(), .rounded(), .w_full(), .w(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
