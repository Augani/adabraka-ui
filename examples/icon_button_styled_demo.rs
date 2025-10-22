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
                        title: Some("IconButton Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| IconButtonStyledDemo::new()),
            )
            .unwrap();
        });
}

struct IconButtonStyledDemo {
    click_count: usize,
}

impl IconButtonStyledDemo {
    fn new() -> Self {
        Self {
            click_count: 0,
        }
    }
}

impl Render for IconButtonStyledDemo {
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
                            .child("IconButton Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent)
                            .child(format!("Total clicks: {}", self.click_count))
                    )
            )
            // 1. Custom Sizes
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Sizes (via Styled trait)")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .size(px(56.0))  // Larger default size
                                    .w(px(80.0))  // Custom width via Styled trait
                                    .h(px(60.0))  // Custom height via Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .size(px(64.0))  // Even larger
                                    .w(px(100.0))  // Custom width via Styled trait
                                    .h(px(80.0))  // Custom height via Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                IconButton::new("heart")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0xe11d48))  // Rose red
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("star")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0xf59e0b))  // Amber
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("check")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0x10b981))  // Emerald
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("info")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0x3b82f6))  // Blue
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("x")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0x8b5cf6))  // Violet
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Outline)
                                    .border_2()  // Thicker border
                                    .border_color(rgb(0x3b82f6))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("bell")
                                    .variant(ButtonVariant::Outline)
                                    .border_2()  // Thicker border
                                    .border_color(rgb(0xef4444))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("user")
                                    .variant(ButtonVariant::Outline)
                                    .border_2()  // Thicker border
                                    .border_color(rgb(0x8b5cf6))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(0.0))  // Square
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(8.0))  // Slight rounding
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(16.0))  // More rounded
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(999.0))  // Perfect circle
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 5. Custom Padding
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Padding")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .p_2()  // Small padding
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .p_4()  // Medium padding
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .p_8()  // Large padding
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                IconButton::new("heart")
                                    .variant(ButtonVariant::Default)
                                    .shadow_sm()  // Small shadow
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("star")
                                    .variant(ButtonVariant::Default)
                                    .shadow_md()  // Medium shadow
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("zap")
                                    .variant(ButtonVariant::Default)
                                    .shadow_lg()  // Large shadow
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
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
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                IconButton::new("heart")
                                    .variant(ButtonVariant::Ghost)
                                    .size(px(48.0))
                                    .w(px(64.0))  // Custom width
                                    .h(px(64.0))  // Custom height
                                    .bg(rgb(0xe11d48))  // Rose red
                                    .rounded(px(999.0))  // Perfect circle
                                    .shadow_lg()  // Large shadow
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("star")
                                    .variant(ButtonVariant::Outline)
                                    .size(px(48.0))
                                    .w(px(80.0))  // Custom width
                                    .h(px(56.0))  // Custom height
                                    .border_2()  // Thicker border
                                    .border_color(rgb(0xf59e0b))  // Amber
                                    .rounded(px(16.0))  // Rounded corners
                                    .shadow_md()  // Medium shadow
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("zap")
                                    .variant(ButtonVariant::Ghost)
                                    .size(px(48.0))
                                    .w(px(72.0))  // Custom width
                                    .h(px(72.0))  // Custom height
                                    .p_4()  // Custom padding
                                    .bg(rgb(0x8b5cf6))  // Violet
                                    .rounded(px(20.0))  // Rounded corners
                                    .shadow_lg()  // Large shadow
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 8. Different Variants with Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Different Variants with Custom Styling")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Default)
                                    .size(px(48.0))
                                    .rounded(px(12.0))
                                    .shadow_md()
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Secondary)
                                    .size(px(48.0))
                                    .rounded(px(12.0))
                                    .shadow_md()
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Outline)
                                    .size(px(48.0))
                                    .rounded(px(12.0))
                                    .border_2()
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Ghost)
                                    .size(px(48.0))
                                    .rounded(px(12.0))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                IconButton::new("settings")
                                    .variant(ButtonVariant::Destructive)
                                    .size(px(48.0))
                                    .rounded(px(12.0))
                                    .shadow_md()
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
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
                            .child("Methods used: .w(), .h(), .p_2/4/8(), .bg(), .border_2(), .border_color(), .rounded(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
