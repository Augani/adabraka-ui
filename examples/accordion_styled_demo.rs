use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    display::accordion::Accordion,
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
                        title: Some("Accordion Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| AccordionStyledDemo::new()),
            )
            .unwrap();
        });
}

struct AccordionStyledDemo {}

impl AccordionStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for AccordionStyledDemo {
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
                                        .child("Accordion Styled Trait Customization Demo")
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
                                    Accordion::new("default-padding")
                                        .item(|item| {
                                            item.title("Default Padding")
                                                .content(div().child("This accordion has default padding"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Second Item")
                                                .content(div().child("Another item with default settings"))
                                        })
                                )
                                .child(
                                    Accordion::new("custom-p4-padding")
                                        .item(|item| {
                                            item.title("Custom p_4() Padding")
                                                .content(div().child("This accordion has p_4() padding applied"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Second Item")
                                                .content(div().child("Consistent padding across all items"))
                                        })
                                        .p_4()  // <- Styled trait method
                                )
                                .child(
                                    Accordion::new("custom-p8-padding")
                                        .item(|item| {
                                            item.title("Custom p_8() Padding")
                                                .content(div().child("This accordion has p_8() padding applied"))
                                        })
                                        .item(|item| {
                                            item.title("Second Item")
                                                .content(div().child("Extra large padding for spacious feel"))
                                        })
                                        .p_8()  // <- Styled trait method
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
                                    Accordion::new("blue-bg")
                                        .item(|item| {
                                            item.title("Blue Background Accordion")
                                                .icon("star")
                                                .content(div().child("Content with blue background parent"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Second Blue Item")
                                                .content(div().child("All items share the blue background"))
                                        })
                                        .bg(hsla(217.0 / 360.0, 0.91, 0.60, 0.2))  // <- Styled trait (blue)
                                        .rounded(px(8.0))
                                        .p(px(12.0))
                                )
                                .child(
                                    Accordion::new("purple-bg")
                                        .item(|item| {
                                            item.title("Purple Background Accordion")
                                                .icon("heart")
                                                .content(div().child("Content with purple background parent"))
                                        })
                                        .item(|item| {
                                            item.title("Second Purple Item")
                                                .content(div().child("Beautiful purple themed accordion"))
                                        })
                                        .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.2))  // <- Styled trait (purple)
                                        .rounded(px(8.0))
                                        .p(px(12.0))
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
                                    Accordion::new("border-blue")
                                        .item(|item| {
                                            item.title("Border 2px Blue")
                                                .content(div().child("Content with 2px blue border"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Second Bordered Item")
                                                .content(div().child("Sharp blue border around accordion"))
                                        })
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0x3b82f6))
                                        .rounded(px(8.0))
                                        .p(px(8.0))
                                )
                                .child(
                                    Accordion::new("border-red")
                                        .item(|item| {
                                            item.title("Border 2px Red")
                                                .content(div().child("Content with 2px red border"))
                                        })
                                        .item(|item| {
                                            item.title("Second Red Bordered Item")
                                                .content(div().child("Bold red border for emphasis"))
                                        })
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0xef4444))
                                        .rounded(px(8.0))
                                        .p(px(8.0))
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
                                    Accordion::new("no-radius")
                                        .item(|item| {
                                            item.title("No Radius (Square)")
                                                .content(div().child("Content with no border radius"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Sharp Corners")
                                                .content(div().child("Perfect for modern, geometric designs"))
                                        })
                                        .rounded(px(0.0))  // <- Styled trait
                                        .bg(theme.tokens.muted.opacity(0.3))
                                        .p(px(8.0))
                                )
                                .child(
                                    Accordion::new("large-radius")
                                        .item(|item| {
                                            item.title("Large Radius")
                                                .content(div().child("Content with large border radius"))
                                        })
                                        .item(|item| {
                                            item.title("Soft Rounded")
                                                .content(div().child("Smooth and welcoming appearance"))
                                        })
                                        .rounded(px(20.0))  // <- Styled trait
                                        .bg(theme.tokens.muted.opacity(0.3))
                                        .p(px(8.0))
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
                                    Accordion::new("full-width")
                                        .item(|item| {
                                            item.title("Full Width Accordion (w_full)")
                                                .content(div().child("Content spans full width"))
                                        })
                                        .item(|item| {
                                            item.title("Expands Completely")
                                                .content(div().child("Uses all available horizontal space"))
                                        })
                                        .w_full()  // <- Styled trait (default, but explicit)
                                        .bg(theme.tokens.accent.opacity(0.2))
                                        .rounded(px(8.0))
                                        .p(px(8.0))
                                )
                                .child(
                                    Accordion::new("custom-width")
                                        .item(|item| {
                                            item.title("Custom Width (600px)")
                                                .content(div().child("Content with custom width"))
                                        })
                                        .item(|item| {
                                            item.title("Fixed Size")
                                                .content(div().child("Controlled width for specific layouts"))
                                        })
                                        .w(px(600.0))  // <- Styled trait
                                        .bg(theme.tokens.accent.opacity(0.2))
                                        .rounded(px(8.0))
                                        .p(px(8.0))
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
                                    Accordion::new("shadow-small")
                                        .item(|item| {
                                            item.title("Shadow Small")
                                                .content(div().child("Content with small shadow"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Subtle Depth")
                                                .content(div().child("Gentle elevation effect"))
                                        })
                                        .shadow_sm()  // <- Styled trait
                                        .bg(theme.tokens.card)
                                        .rounded(px(8.0))
                                        .p(px(8.0))
                                )
                                .child(
                                    Accordion::new("shadow-medium")
                                        .item(|item| {
                                            item.title("Shadow Medium")
                                                .content(div().child("Content with medium shadow"))
                                        })
                                        .item(|item| {
                                            item.title("Moderate Elevation")
                                                .content(div().child("Balanced depth perception"))
                                        })
                                        .shadow_md()  // <- Styled trait
                                        .bg(theme.tokens.card)
                                        .rounded(px(8.0))
                                        .p(px(8.0))
                                )
                                .child(
                                    Accordion::new("shadow-large")
                                        .item(|item| {
                                            item.title("Shadow Large")
                                                .content(div().child("Content with large shadow"))
                                        })
                                        .item(|item| {
                                            item.title("Prominent Depth")
                                                .content(div().child("Strong floating appearance"))
                                        })
                                        .shadow_lg()  // <- Styled trait
                                        .bg(theme.tokens.card)
                                        .rounded(px(8.0))
                                        .p(px(8.0))
                                )
                        )
                        // 7. Multiple Selection with Styling
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("7. Multiple Selection with Custom Styling")
                                )
                                .child(
                                    Accordion::new("multi-select-styled")
                                        .item(|item| {
                                            item.title("Feature 1: Multiple Open Items")
                                                .icon("check-circle")
                                                .content(div().child("This accordion allows multiple items to be open simultaneously"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Feature 2: Custom Styling")
                                                .icon("palette")
                                                .content(div().child("Combined with beautiful custom styling"))
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Feature 3: Enhanced Experience")
                                                .icon("zap")
                                                .content(div().child("Multiple features can be viewed at once"))
                                        })
                                        .multiple(true)  // <- Allow multiple open
                                        .bg(hsla(142.0 / 360.0, 0.76, 0.45, 0.15))  // <- Styled trait (green)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0x10b981))
                                        .rounded(px(12.0))  // <- Styled trait
                                        .shadow_md()  // <- Styled trait
                                        .p(px(12.0))
                                        .on_change(|indices, _, _| {
                                            println!("Open items: {:?}", indices);
                                        })
                                )
                        )
                        // 8. Combined Ultra Styling
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("8. Combined Ultra Styling (All Styled Trait Methods)")
                                )
                                .child(
                                    Accordion::new("ultra-styled")
                                        .item(|item| {
                                            item.title("Premium Accordion Design")
                                                .icon("award")
                                                .content(
                                                    div()
                                                        .p(px(20.0))
                                                        .child("This demonstrates maximum customization with all Styled trait capabilities combined: padding, background, border, radius, shadow, and width control")
                                                )
                                                .open(true)
                                        })
                                        .item(|item| {
                                            item.title("Professional Appearance")
                                                .icon("sparkles")
                                                .content(
                                                    div()
                                                        .p(px(20.0))
                                                        .child("Perfect for high-end applications requiring distinctive visual design")
                                                )
                                        })
                                        .item(|item| {
                                            item.title("Full Customization Power")
                                                .icon("settings")
                                                .content(
                                                    div()
                                                        .p(px(20.0))
                                                        .child("Every aspect is controllable via the Styled trait")
                                                )
                                        })
                                        .w(px(800.0))  // <- Styled trait
                                        .px(px(24.0))  // <- Styled trait
                                        .py(px(16.0))  // <- Styled trait
                                        .bg(hsla(43.0 / 360.0, 0.96, 0.56, 0.15))  // <- Styled trait (orange)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0xf59e0b))
                                        .rounded(px(20.0))  // <- Styled trait
                                        .shadow_lg()  // <- Styled trait
                                        .on_change(|indices, _, _| {
                                            println!("Ultra styled accordion changed: {:?}", indices);
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
                                        .child("Methods used: .p_4(), .p_8(), .px(), .py(), .p(), .bg(), .border_2(), .border_color(), .rounded(), .w_full(), .w(), .shadow_sm/md/lg()")
                                )
                        )
                )
            )
    }
}
