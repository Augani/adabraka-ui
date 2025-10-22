use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        tooltip::{Tooltip, TooltipPlacement},
    },
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
                        title: Some("Tooltip Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| TooltipStyledDemo::new()),
            )
            .unwrap();
        });
}

struct TooltipStyledDemo {}

impl TooltipStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for TooltipStyledDemo {
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
                            .child("Tooltip Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait on Tooltips")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.accent)
                            .child("Hover over the elements to see the custom styled tooltips")
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
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Tooltip::new("Default padding tooltip")
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Default")
                                    )
                            )
                            .child(
                                Tooltip::new("Custom p_4() padding")
                                    .p_4()  // <- Styled trait method
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: p_4()")
                                    )
                            )
                            .child(
                                Tooltip::new("Custom p_8() padding")
                                    .p_8()  // <- Styled trait method
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: p_8()")
                                    )
                            )
                            .child(
                                Tooltip::new("Extra large padding")
                                    .px(px(32.0))  // <- Styled trait method
                                    .py(px(16.0))  // <- Styled trait method
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: px(32) py(16)")
                                    )
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
                                Tooltip::new("Blue themed tooltip")
                                    .bg(rgb(0x3b82f6))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Blue")
                                    )
                            )
                            .child(
                                Tooltip::new("Purple themed tooltip")
                                    .bg(rgb(0x8b5cf6))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Purple")
                                    )
                            )
                            .child(
                                Tooltip::new("Green themed tooltip")
                                    .bg(rgb(0x10b981))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Green")
                                    )
                            )
                            .child(
                                Tooltip::new("Orange themed tooltip")
                                    .bg(rgb(0xf59e0b))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Orange")
                                    )
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
                                Tooltip::new("Thick blue border")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Border 2px Blue")
                                    )
                            )
                            .child(
                                Tooltip::new("Thick red border")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0xef4444))
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Border 2px Red")
                                    )
                            )
                            .child(
                                Tooltip::new("Ultra thick purple border")
                                    .border(px(4.0))  // <- Styled trait
                                    .border_color(rgb(0x8b5cf6))
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Border 4px Purple")
                                    )
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
                                Tooltip::new("No border radius - sharp corners")
                                    .rounded(px(0.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: No Radius")
                                    )
                            )
                            .child(
                                Tooltip::new("Medium border radius")
                                    .rounded(px(12.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Radius 12px")
                                    )
                            )
                            .child(
                                Tooltip::new("Large border radius")
                                    .rounded(px(20.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Radius 20px")
                                    )
                            )
                            .child(
                                Tooltip::new("Pill shaped tooltip")
                                    .rounded(px(999.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Pill")
                                    )
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Tooltip::new("This is a tooltip with full width styling applied to it")
                                    .w_full()  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: w_full()")
                                    )
                            )
                            .child(
                                Tooltip::new("Custom width 200px tooltip")
                                    .w(px(200.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: w(200px)")
                                    )
                            )
                            .child(
                                Tooltip::new("Custom width 400px tooltip with extra text")
                                    .w(px(400.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: w(400px)")
                                    )
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
                                Tooltip::new("Small shadow tooltip")
                                    .shadow_sm()  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: shadow_sm()")
                                    )
                            )
                            .child(
                                Tooltip::new("Medium shadow tooltip")
                                    .shadow_md()  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: shadow_md()")
                                    )
                            )
                            .child(
                                Tooltip::new("Large shadow tooltip")
                                    .shadow_lg()  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: shadow_lg()")
                                    )
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
                            .gap(px(12.0))
                            .child(
                                Tooltip::new("Purple pill tooltip with large shadow")
                                    .p_8()  // <- Styled trait
                                    .rounded(px(999.0))  // <- Styled trait
                                    .bg(rgb(0x8b5cf6))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .shadow_lg()  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Purple Pill + Shadow")
                                    )
                            )
                            .child(
                                Tooltip::new("Custom bordered gradient-like tooltip")
                                    .p(px(20.0))  // <- Styled trait
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x10b981))
                                    .rounded(px(12.0))  // <- Styled trait
                                    .bg(rgb(0x059669))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .shadow_md()  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Green Border + BG")
                                    )
                            )
                            .child(
                                Tooltip::new("Ultra custom orange tooltip with all the bells and whistles")
                                    .px(px(40.0))  // <- Styled trait
                                    .py(px(16.0))  // <- Styled trait
                                    .bg(rgb(0xf59e0b))  // <- Styled trait
                                    .text_color(gpui::white())
                                    .rounded(px(8.0))  // <- Styled trait
                                    .shadow_lg()  // <- Styled trait
                                    .w(px(400.0))  // <- Styled trait
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Hover: Ultra Custom Orange")
                                    )
                            )
                    )
            )
            // 8. Different Placements with Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Different Placements with Custom Styling")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Each tooltip has custom styling applied via Styled trait")
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(80.0))
                            .p(px(60.0))
                            .child(
                                // Top
                                Tooltip::new("Top tooltip with blue styling")
                                    .placement(TooltipPlacement::Top)
                                    .bg(rgb(0x3b82f6))
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_lg()
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Top")
                                    )
                            )
                            .child(
                                // Bottom
                                Tooltip::new("Bottom tooltip with green styling")
                                    .placement(TooltipPlacement::Bottom)
                                    .bg(rgb(0x10b981))
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_lg()
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Bottom")
                                    )
                            )
                            .child(
                                // Left
                                Tooltip::new("Left tooltip with purple styling")
                                    .placement(TooltipPlacement::Left)
                                    .bg(rgb(0x8b5cf6))
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_lg()
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Left")
                                    )
                            )
                            .child(
                                // Right
                                Tooltip::new("Right tooltip with orange styling")
                                    .placement(TooltipPlacement::Right)
                                    .bg(rgb(0xf59e0b))
                                    .text_color(gpui::white())
                                    .p_4()
                                    .rounded(px(8.0))
                                    .shadow_lg()
                                    .child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.secondary)
                                            .rounded(px(8.0))
                                            .child("Right")
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
                            .child("All tooltip customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .p_4(), .p_8(), .px(), .py(), .bg(), .border_2(), .border(), .border_color(), .rounded(), .w_full(), .w(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
