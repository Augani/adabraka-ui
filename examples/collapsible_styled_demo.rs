use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        collapsible::Collapsible,
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
                        title: Some("Collapsible Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| CollapsibleStyledDemo::new()),
            )
            .unwrap();
        });
}

struct CollapsibleStyledDemo {}

impl CollapsibleStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for CollapsibleStyledDemo {
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
                                        .child("Collapsible Styled Trait Customization Demo")
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
                                    Collapsible::new()
                                        .trigger(div().child("Default Padding"))
                                        .content(div().p(px(16.0)).child("This collapsible has default padding"))
                                        .open(true)
                                        .on_toggle(|is_open, _, _| {
                                            println!("Default Padding toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Custom p_4() Padding"))
                                        .content(div().p(px(16.0)).child("This collapsible has p_4() padding applied"))
                                        .open(false)
                                        .p_4()  // <- Styled trait method
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Custom p_8() Padding"))
                                        .content(div().p(px(16.0)).child("This collapsible has p_8() padding applied"))
                                        .open(false)
                                        .p_8()  // <- Styled trait method
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
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
                                    Collapsible::new()
                                        .trigger(div().child("Blue Background Collapsible"))
                                        .content(div().p(px(16.0)).child("Content with blue background parent"))
                                        .open(true)
                                        .bg(hsla(217.0 / 360.0, 0.91, 0.60, 0.2))  // <- Styled trait (blue)
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Purple Background Collapsible"))
                                        .content(div().p(px(16.0)).child("Content with purple background parent"))
                                        .open(false)
                                        .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.2))  // <- Styled trait (purple)
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
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
                                    Collapsible::new()
                                        .trigger(div().child("Border 2px Blue"))
                                        .content(div().p(px(16.0)).child("Content with 2px blue border"))
                                        .open(true)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0x3b82f6))
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Border 2px Red"))
                                        .content(div().p(px(16.0)).child("Content with 2px red border"))
                                        .open(false)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0xef4444))
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
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
                                    Collapsible::new()
                                        .trigger(div().child("No Radius (Square)"))
                                        .content(div().p(px(16.0)).child("Content with no border radius"))
                                        .open(true)
                                        .rounded(px(0.0))  // <- Styled trait
                                        .bg(theme.tokens.muted.opacity(0.3))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Large Radius"))
                                        .content(div().p(px(16.0)).child("Content with large border radius"))
                                        .open(false)
                                        .rounded(px(20.0))  // <- Styled trait
                                        .bg(theme.tokens.muted.opacity(0.3))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
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
                                    Collapsible::new()
                                        .trigger(div().child("Full Width Collapsible (w_full)"))
                                        .content(div().p(px(16.0)).child("Content spans full width"))
                                        .open(false)
                                        .w_full()  // <- Styled trait (default, but explicit)
                                        .bg(theme.tokens.accent.opacity(0.2))
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Custom Width (600px)"))
                                        .content(div().p(px(16.0)).child("Content with custom width"))
                                        .open(false)
                                        .w(px(600.0))  // <- Styled trait
                                        .bg(theme.tokens.accent.opacity(0.2))
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
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
                                    Collapsible::new()
                                        .trigger(div().child("Shadow Small"))
                                        .content(div().p(px(16.0)).child("Content with small shadow"))
                                        .open(true)
                                        .shadow_sm()  // <- Styled trait
                                        .bg(theme.tokens.card)
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Shadow Medium"))
                                        .content(div().p(px(16.0)).child("Content with medium shadow"))
                                        .open(false)
                                        .shadow_md()  // <- Styled trait
                                        .bg(theme.tokens.card)
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Shadow Large"))
                                        .content(div().p(px(16.0)).child("Content with large shadow"))
                                        .open(false)
                                        .shadow_lg()  // <- Styled trait
                                        .bg(theme.tokens.card)
                                        .rounded(px(8.0))
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
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
                                    Collapsible::new()
                                        .trigger(div().child("Purple Card with Shadow & Padding"))
                                        .content(
                                            div()
                                                .p(px(20.0))
                                                .child("This combines multiple styling methods: padding, background, border, radius, and shadow")
                                        )
                                        .open(true)
                                        .p_4()  // <- Styled trait
                                        .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.15))  // <- Styled trait (purple)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0x8b5cf6))
                                        .rounded(px(12.0))  // <- Styled trait
                                        .shadow_lg()  // <- Styled trait
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Green Accent Card with Custom Width"))
                                        .content(
                                            div()
                                                .p(px(20.0))
                                                .child("Full customization with width control, padding, colors, and shadows")
                                        )
                                        .open(false)
                                        .w(px(700.0))  // <- Styled trait
                                        .p(px(20.0))  // <- Styled trait
                                        .bg(hsla(142.0 / 360.0, 0.76, 0.45, 0.15))  // <- Styled trait (green)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0x10b981))
                                        .rounded(px(16.0))  // <- Styled trait
                                        .shadow_md()  // <- Styled trait
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
                                        })
                                )
                                .child(
                                    Collapsible::new()
                                        .trigger(div().child("Orange Ultra Custom Card"))
                                        .content(
                                            div()
                                                .p(px(24.0))
                                                .child("Maximum customization demonstrating all Styled trait capabilities combined")
                                        )
                                        .open(false)
                                        .w(px(750.0))  // <- Styled trait
                                        .px(px(24.0))  // <- Styled trait
                                        .py(px(16.0))  // <- Styled trait
                                        .bg(hsla(43.0 / 360.0, 0.96, 0.56, 0.15))  // <- Styled trait (orange)
                                        .border_2()  // <- Styled trait
                                        .border_color(rgb(0xf59e0b))
                                        .rounded(px(20.0))  // <- Styled trait
                                        .shadow_lg()  // <- Styled trait
                                        .on_toggle(|is_open, _, _| {
                                            println!("Toggled: {}", is_open);
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
