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
                        title: Some("Badge Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| BadgeStyledDemo::new()),
            )
            .unwrap();
        });
}

struct BadgeStyledDemo;

impl BadgeStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for BadgeStyledDemo {
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
                            .child("Badge Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait for Badge component")
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
                            .child(Badge::new("Default"))
                            .child(
                                Badge::new("p_4()")
                                    .p_4()  // Styled trait method
                            )
                            .child(
                                Badge::new("px(20)")
                                    .px(px(20.0))  // Styled trait method
                            )
                            .child(
                                Badge::new("py(10)")
                                    .py(px(10.0))  // Styled trait method
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
                            .child("2. Custom Background Colors & Text Colors")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Badge::new("Blue")
                                    .bg(rgb(0x3b82f6))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
                            )
                            .child(
                                Badge::new("Purple")
                                    .bg(rgb(0x8b5cf6))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
                            )
                            .child(
                                Badge::new("Green")
                                    .bg(rgb(0x10b981))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
                            )
                            .child(
                                Badge::new("Orange")
                                    .bg(rgb(0xf59e0b))  // Styled trait
                                    .text_color(gpui::white())
                                    .p_4()
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
                            .items_center()
                            .child(
                                Badge::new("Border 1px")
                                    .border_1()  // Styled trait
                                    .border_color(rgb(0x3b82f6))
                            )
                            .child(
                                Badge::new("Border 2px")
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0xef4444))
                                    .bg(gpui::transparent_black())
                            )
                            .child(
                                Badge::new("Border 4px")
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0x10b981))
                                    .bg(gpui::transparent_black())
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
                            .items_center()
                            .child(
                                Badge::new("No Radius")
                                    .variant(BadgeVariant::Secondary)
                                    .rounded(px(0.0))  // Styled trait - sharp corners
                            )
                            .child(
                                Badge::new("Small Radius")
                                    .variant(BadgeVariant::Secondary)
                                    .rounded(px(4.0))  // Styled trait
                            )
                            .child(
                                Badge::new("Large Radius")
                                    .variant(BadgeVariant::Destructive)
                                    .rounded(px(16.0))  // Styled trait
                            )
                            .child(
                                Badge::new("Default is Full")
                                    .variant(BadgeVariant::Warning)
                                    // Uses default rounded_full() from Badge
                            )
                    )
            )
            // 5. Size Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Size Control (Width & Height)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Badge::new("Custom Width Badge")
                                    .w(px(200.0))  // Styled trait
                                    .text_size(px(14.0))
                                    .justify_center()
                            )
                            .child(
                                Badge::new("Custom Height Badge")
                                    .h(px(50.0))  // Styled trait
                                    .text_size(px(16.0))
                                    .items_center()
                            )
                            .child(
                                Badge::new("Wide Badge")
                                    .min_w(px(300.0))  // Styled trait
                                    .justify_center()
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
                            .gap(px(16.0))
                            .items_center()
                            .child(
                                Badge::new("Shadow SM")
                                    .variant(BadgeVariant::Secondary)
                                    .shadow_sm()  // Styled trait
                            )
                            .child(
                                Badge::new("Shadow MD")
                                    .variant(BadgeVariant::Default)
                                    .shadow_md()  // Styled trait
                            )
                            .child(
                                Badge::new("Shadow LG")
                                    .variant(BadgeVariant::Destructive)
                                    .shadow_lg()  // Styled trait
                            )
                    )
            )
            // 7. Variant Combinations with Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. All Variants with Custom Styling")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .flex_wrap()
                            .child(
                                Badge::new("Default + Shadow")
                                    .variant(BadgeVariant::Default)
                                    .shadow_md()
                                    .p_4()
                            )
                            .child(
                                Badge::new("Secondary + Border")
                                    .variant(BadgeVariant::Secondary)
                                    .border_2()
                                    .border_color(theme.tokens.primary)
                                    .p_4()
                            )
                            .child(
                                Badge::new("Destructive + Large")
                                    .variant(BadgeVariant::Destructive)
                                    .text_size(px(16.0))
                                    .px(px(16.0))
                                    .py(px(6.0))
                            )
                            .child(
                                Badge::new("Outline + Colored")
                                    .variant(BadgeVariant::Outline)
                                    .border_color(rgb(0x3b82f6))
                                    .text_color(rgb(0x3b82f6))
                                    .p_4()
                            )
                            .child(
                                Badge::new("Warning + Shadow")
                                    .variant(BadgeVariant::Warning)
                                    .shadow_lg()
                                    .p_4()
                            )
                    )
            )
            // 8. Combined Advanced Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Combined Advanced Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Badge::new("Premium Badge")
                                    .px(px(24.0))  // Styled trait
                                    .py(px(8.0))  // Styled trait
                                    .bg(rgb(0xfbbf24))  // Styled trait - gold color
                                    .text_color(rgb(0x000000))
                                    .rounded(px(12.0))  // Styled trait
                                    .shadow_lg()  // Styled trait
                                    .border_2()
                                    .border_color(rgb(0xf59e0b))
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                            )
                            .child(
                                Badge::new("Featured")
                                    .w(px(250.0))  // Styled trait
                                    .h(px(40.0))  // Styled trait
                                    .bg(gpui::hsla(260.0 / 360.0, 0.6, 0.5, 1.0))  // Styled trait
                                    .text_color(gpui::white())
                                    .rounded(px(8.0))  // Styled trait
                                    .shadow_md()  // Styled trait
                                    .text_size(px(16.0))
                                    .justify_center()
                                    .items_center()
                            )
                            .child(
                                Badge::new("Exclusive Offer")
                                    .px(px(32.0))  // Styled trait
                                    .py(px(12.0))  // Styled trait
                                    .bg(gpui::hsla(340.0 / 360.0, 0.82, 0.52, 1.0))  // Styled trait - pink/red
                                    .text_color(gpui::white())
                                    .rounded(px(20.0))  // Styled trait
                                    .shadow_lg()  // Styled trait
                                    .border_2()
                                    .border_color(gpui::hsla(340.0 / 360.0, 0.82, 0.62, 1.0))
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::SEMIBOLD)
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
                            .child("Methods used: .p_4(), .px(), .py(), .bg(), .text_color(), .border_1/2(), .border_color(), .rounded(), .w(), .h(), .min_w(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
