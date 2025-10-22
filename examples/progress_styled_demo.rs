use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    components::progress::SpinnerType,
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
                        title: Some("Progress Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ProgressStyledDemo::new()),
            )
            .unwrap();
        });
}

struct ProgressStyledDemo {}

impl ProgressStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for ProgressStyledDemo {
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
                            .child("Progress Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. ProgressBar - Custom Width
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. ProgressBar - Custom Width")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Default width (w_full)")
                            )
                            .child(
                                ProgressBar::new(0.7)
                                    .variant(ProgressVariant::Default)
                                    .size(ProgressSize::Md)
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom width (400px)")
                            )
                            .child(
                                ProgressBar::new(0.7)
                                    .variant(ProgressVariant::Default)
                                    .size(ProgressSize::Md)
                                    .w(px(400.0))  // <- Styled trait method
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom width (200px)")
                            )
                            .child(
                                ProgressBar::new(0.7)
                                    .variant(ProgressVariant::Default)
                                    .size(ProgressSize::Md)
                                    .w(px(200.0))  // <- Styled trait method
                            )
                    )
            )
            // 2. ProgressBar - Custom Margins
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. ProgressBar - Custom Margins")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom margin-left (ml)")
                            )
                            .child(
                                ProgressBar::new(0.6)
                                    .variant(ProgressVariant::Success)
                                    .ml(px(32.0))  // <- Styled trait method
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom horizontal margin (mx)")
                            )
                            .child(
                                ProgressBar::new(0.6)
                                    .variant(ProgressVariant::Warning)
                                    .mx(px(48.0))  // <- Styled trait method
                            )
                    )
            )
            // 3. ProgressBar - Custom Opacity
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. ProgressBar - Custom Opacity")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Opacity 1.0 (default)")
                            )
                            .child(
                                ProgressBar::new(0.8)
                                    .variant(ProgressVariant::Default)
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Opacity 0.5")
                            )
                            .child(
                                ProgressBar::new(0.8)
                                    .variant(ProgressVariant::Default)
                                    .opacity(0.5)  // <- Styled trait method
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Opacity 0.3")
                            )
                            .child(
                                ProgressBar::new(0.8)
                                    .variant(ProgressVariant::Default)
                                    .opacity(0.3)  // <- Styled trait method
                            )
                    )
            )
            // 4. CircularProgress - Custom Margins
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. CircularProgress - Custom Margins")
                    )
                    .child(
                        HStack::new()
                            .gap(px(24.0))
                            .items_center()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(40.0))
                                            .spinner_type(SpinnerType::Dot)
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Default")
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(40.0))
                                            .spinner_type(SpinnerType::Dot)
                                            .ml(px(32.0))  // <- Styled trait method
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("With ml(32px)")
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(40.0))
                                            .spinner_type(SpinnerType::Dot)
                                            .mx(px(16.0))  // <- Styled trait method
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("With mx(16px)")
                                    )
                            )
                    )
            )
            // 5. CircularProgress - Custom Opacity
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. CircularProgress - Custom Opacity")
                    )
                    .child(
                        HStack::new()
                            .gap(px(24.0))
                            .items_center()
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(50.0))
                                            .spinner_type(SpinnerType::Arc)
                                            .variant(ProgressVariant::Default)
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Opacity 1.0")
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(50.0))
                                            .spinner_type(SpinnerType::Arc)
                                            .variant(ProgressVariant::Default)
                                            .opacity(0.6)  // <- Styled trait method
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Opacity 0.6")
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(50.0))
                                            .spinner_type(SpinnerType::Arc)
                                            .variant(ProgressVariant::Default)
                                            .opacity(0.3)  // <- Styled trait method
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Opacity 0.3")
                                    )
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
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom width + margins + opacity")
                            )
                            .child(
                                ProgressBar::new(0.65)
                                    .variant(ProgressVariant::Success)
                                    .label("Processing...")
                                    .w(px(350.0))  // <- Styled trait
                                    .mx(px(24.0))  // <- Styled trait
                                    .opacity(0.8)  // <- Styled trait
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Margins + opacity spinner")
                            )
                            .child(
                                HStack::new()
                                    .gap(px(16.0))
                                    .items_center()
                                    .child(
                                        CircularProgress::indeterminate()
                                            .size(px(50.0))
                                            .spinner_type(SpinnerType::GrowingCircle)
                                            .variant(ProgressVariant::Warning)
                                            .ml(px(24.0))  // <- Styled trait
                                            .opacity(0.7)  // <- Styled trait
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.foreground)
                                            .child("Loading with custom styling...")
                                    )
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
                            .child("Methods used: .w(), .ml(), .mx(), .opacity()")
                    )
            )
                )
            )
    }
}
