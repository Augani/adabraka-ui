use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    navigation::status_bar::{StatusBar, StatusItem},
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
                        title: Some("StatusBar Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| StatusBarStyledDemo::new()),
            )
            .unwrap();
        });
}

struct StatusBarStyledDemo {
    click_count: usize,
}

impl StatusBarStyledDemo {
    fn new() -> Self {
        Self {
            click_count: 0,
        }
    }
}

impl Render for StatusBarStyledDemo {
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
                            .child("StatusBar Styled Trait Customization Demo")
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
            // 1. Default StatusBar
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default StatusBar")
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon_text("file", "main.rs"),
                                    StatusItem::text("Modified"),
                                ])
                                .center(vec![
                                    StatusItem::text("Ready"),
                                ])
                                .right(vec![
                                    StatusItem::text("Line 42, Col 12"),
                                    StatusItem::icon_text("git-branch", "main"),
                                ])
                        })
                    )
            )
            // 2. Custom Background Color
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
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon_text("alert-circle", "Build Success"),
                                ])
                                .center(vec![
                                    StatusItem::text("Green Theme"),
                                ])
                                .right(vec![
                                    StatusItem::text("100%"),
                                ])
                                .bg(rgb(0x10b981))  // Green background
                                .text_color(gpui::white())
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon_text("info", "Information"),
                                ])
                                .center(vec![
                                    StatusItem::text("Blue Theme"),
                                ])
                                .right(vec![
                                    StatusItem::icon("settings"),
                                ])
                                .bg(rgb(0x3b82f6))  // Blue background
                                .text_color(gpui::white())
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon_text("alert-triangle", "Warning"),
                                ])
                                .center(vec![
                                    StatusItem::text("Orange Theme"),
                                ])
                                .right(vec![
                                    StatusItem::text("Check logs"),
                                ])
                                .bg(rgb(0xf59e0b))  // Orange background
                                .text_color(gpui::white())
                        })
                    )
            )
            // 3. Custom Height and Padding
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Height and Padding")
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Compact"),
                                ])
                                .center(vec![
                                    StatusItem::text("Small height"),
                                ])
                                .right(vec![
                                    StatusItem::text("20px"),
                                ])
                                .height(px(20.0))
                                .py(px(2.0))  // Custom padding
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Extra Large"),
                                ])
                                .center(vec![
                                    StatusItem::text("Large height with padding"),
                                ])
                                .right(vec![
                                    StatusItem::text("60px"),
                                ])
                                .height(px(60.0))
                                .p(px(16.0))  // Custom padding all around
                        })
                    )
            )
            // 4. Custom Borders
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
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Blue Border Top"),
                                ])
                                .center(vec![
                                    StatusItem::icon("star"),
                                ])
                                .right(vec![
                                    StatusItem::text("3px border"),
                                ])
                                .border_t(px(3.0))  // Custom top border
                                .border_color(rgb(0x3b82f6))
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("All borders"),
                                ])
                                .center(vec![
                                    StatusItem::icon_text("box", "Boxed"),
                                ])
                                .right(vec![
                                    StatusItem::text("2px all around"),
                                ])
                                .border_2()  // Border all around
                                .border_color(rgb(0x8b5cf6))
                        })
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
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("No Radius"),
                                ])
                                .center(vec![
                                    StatusItem::icon("square"),
                                ])
                                .right(vec![
                                    StatusItem::text("0px"),
                                ])
                                .rounded(px(0.0))  // No rounded corners
                                .border_1()
                                .border_color(theme.tokens.border)
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Large Radius"),
                                ])
                                .center(vec![
                                    StatusItem::icon("circle"),
                                ])
                                .right(vec![
                                    StatusItem::text("16px"),
                                ])
                                .rounded(px(16.0))  // Large rounded corners
                                .border_1()
                                .border_color(theme.tokens.border)
                        })
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
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Shadow Small"),
                                ])
                                .center(vec![
                                    StatusItem::icon("sun"),
                                ])
                                .right(vec![
                                    StatusItem::text("sm"),
                                ])
                                .shadow_sm()  // Small shadow
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Shadow Medium"),
                                ])
                                .center(vec![
                                    StatusItem::icon("cloud"),
                                ])
                                .right(vec![
                                    StatusItem::text("md"),
                                ])
                                .shadow_md()  // Medium shadow
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::text("Shadow Large"),
                                ])
                                .center(vec![
                                    StatusItem::icon("moon"),
                                ])
                                .right(vec![
                                    StatusItem::text("lg"),
                                ])
                                .shadow_lg()  // Large shadow
                        })
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
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon_text("check-circle", "Success"),
                                ])
                                .center(vec![
                                    StatusItem::text("All tests passed"),
                                ])
                                .right(vec![
                                    StatusItem::text("100% Coverage"),
                                ])
                                .height(px(48.0))
                                .p(px(12.0))  // Custom padding
                                .bg(rgb(0x10b981))  // Green background
                                .text_color(gpui::white())
                                .rounded(px(12.0))  // Rounded corners
                                .shadow_lg()  // Large shadow
                                .border_2()
                                .border_color(rgb(0x059669))  // Darker green border
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon_text("alert-triangle", "Error"),
                                ])
                                .center(vec![
                                    StatusItem::text("Build failed - check console"),
                                ])
                                .right(vec![
                                    StatusItem::icon_text("x-circle", "3 errors"),
                                ])
                                .height(px(50.0))
                                .px(px(20.0))  // Horizontal padding
                                .py(px(10.0))  // Vertical padding
                                .bg(rgb(0xef4444))  // Red background
                                .text_color(gpui::white())
                                .rounded(px(8.0))
                                .shadow_md()
                                .border_l(px(4.0))  // Left accent border
                                .border_color(rgb(0x991b1b))  // Darker red
                        })
                    )
                    .child(
                        cx.new(|_| {
                            StatusBar::new()
                                .left(vec![
                                    StatusItem::icon("star"),
                                    StatusItem::text("Premium"),
                                ])
                                .center(vec![
                                    StatusItem::icon_text("zap", "All features unlocked"),
                                ])
                                .right(vec![
                                    StatusItem::icon("settings"),
                                    StatusItem::text("Configure"),
                                ])
                                .height(px(44.0))
                                .p_8()  // Padding level 8
                                .bg(rgb(0x8b5cf6))  // Purple background
                                .text_color(gpui::white())
                                .rounded(px(999.0))  // Pill shape
                                .shadow_lg()
                                .border_2()
                                .border_color(rgb(0x6d28d9))
                        })
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
                            .child("Methods used: .bg(), .text_color(), .p(), .px(), .py(), .p_8(), .border_2(), .border_t(), .border_l(), .border_color(), .rounded(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
