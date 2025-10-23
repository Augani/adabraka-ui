use adabraka_ui::{
    prelude::*,
    overlays::bottom_sheet::{BottomSheet, BottomSheetSize},
    components::button::{Button, ButtonVariant},
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
                        title: Some("BottomSheet Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| BottomSheetStyledDemo::new()),
            )
            .unwrap();
        });
}

struct BottomSheetStyledDemo {
    show_default: bool,
    show_custom_bg: bool,
    show_custom_border: bool,
    show_custom_radius: bool,
    show_custom_shadow: bool,
    show_combined: bool,
}

impl BottomSheetStyledDemo {
    fn new() -> Self {
        Self {
            show_default: false,
            show_custom_bg: false,
            show_custom_border: false,
            show_custom_radius: false,
            show_custom_shadow: false,
            show_combined: false,
        }
    }
}

impl Render for BottomSheetStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
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
                                    .child("BottomSheet Styled Trait Customization Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Demonstrating full GPUI styling control via Styled trait")
                            )
                    )
                    // 1. Default BottomSheet
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("1. Default BottomSheet")
                            )
                            .child(
                                Button::new("show-default", "Show Default BottomSheet")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_default = true;
                                        cx.notify();
                                    }))
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
                                    .child("2. Custom Background Color")
                            )
                            .child(
                                Button::new("show-custom-bg", "Show Purple Background")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_bg = true;
                                        cx.notify();
                                    }))
                            )
                    )
                    // 3. Custom Border
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("3. Custom Border")
                            )
                            .child(
                                Button::new("show-custom-border", "Show Thick Blue Border")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_border = true;
                                        cx.notify();
                                    }))
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
                                Button::new("show-custom-radius", "Show Square Corners")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_radius = true;
                                        cx.notify();
                                    }))
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
                                Button::new("show-custom-shadow", "Show Extra Large Shadow")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_shadow = true;
                                        cx.notify();
                                    }))
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
                                Button::new("show-combined", "Show Ultra Custom BottomSheet")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_combined = true;
                                        cx.notify();
                                    }))
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
                                    .child("Methods used: .bg(), .border_4(), .border_color(), .rounded(), .shadow_xl(), .p()")
                            )
                    )
            )
            // Default BottomSheet
            .when(self.show_default, |this| {
                this.child(
                    BottomSheet::new()
                        .size(BottomSheetSize::Md)
                        .title("Default BottomSheet")
                        .description("This is the default BottomSheet with no custom styling")
                        .content(
                            div()
                                .p(px(24.0))
                                .child("This BottomSheet uses the default theme styling.")
                        )
                        .on_close(cx.listener(|view, _, cx| {
                            view.show_default = false;
                            cx.notify();
                        }))
                )
            })
            // Custom Background BottomSheet
            .when(self.show_custom_bg, |this| {
                this.child(
                    BottomSheet::new()
                        .size(BottomSheetSize::Md)
                        .title("Custom Background")
                        .description("Using .bg() from Styled trait")
                        .bg(rgb(0x8b5cf6))  // ← Styled trait
                        .content(
                            div()
                                .p(px(24.0))
                                .text_color(gpui::white())
                                .child("This BottomSheet has a purple background applied via the Styled trait!")
                        )
                        .on_close(cx.listener(|view, _, cx| {
                            view.show_custom_bg = false;
                            cx.notify();
                        }))
                )
            })
            // Custom Border BottomSheet
            .when(self.show_custom_border, |this| {
                this.child(
                    BottomSheet::new()
                        .size(BottomSheetSize::Md)
                        .title("Custom Border")
                        .description("Using .border_4() and .border_color() from Styled trait")
                        .border_4()  // ← Styled trait
                        .border_color(rgb(0x3b82f6))  // ← Styled trait
                        .content(
                            div()
                                .p(px(24.0))
                                .child("This BottomSheet has a thick blue border applied via the Styled trait!")
                        )
                        .on_close(cx.listener(|view, _, cx| {
                            view.show_custom_border = false;
                            cx.notify();
                        }))
                )
            })
            // Custom Radius BottomSheet
            .when(self.show_custom_radius, |this| {
                this.child(
                    BottomSheet::new()
                        .size(BottomSheetSize::Md)
                        .title("Custom Border Radius")
                        .description("Using .rounded() from Styled trait")
                        .rounded(px(0.0))  // ← Styled trait (no radius)
                        .content(
                            div()
                                .p(px(24.0))
                                .child("This BottomSheet has square corners (no border radius) applied via the Styled trait!")
                        )
                        .on_close(cx.listener(|view, _, cx| {
                            view.show_custom_radius = false;
                            cx.notify();
                        }))
                )
            })
            // Custom Shadow BottomSheet
            .when(self.show_custom_shadow, |this| {
                this.child(
                    BottomSheet::new()
                        .size(BottomSheetSize::Md)
                        .title("Custom Shadow")
                        .description("Using .shadow_xl() from Styled trait")
                        .shadow_xl()  // ← Styled trait
                        .content(
                            div()
                                .p(px(24.0))
                                .child("This BottomSheet has an extra large shadow applied via the Styled trait!")
                        )
                        .on_close(cx.listener(|view, _, cx| {
                            view.show_custom_shadow = false;
                            cx.notify();
                        }))
                )
            })
            // Combined Styling BottomSheet
            .when(self.show_combined, |this| {
                this.child(
                    BottomSheet::new()
                        .size(BottomSheetSize::Lg)
                        .title("Ultra Custom BottomSheet")
                        .description("Multiple Styled trait methods combined")
                        .bg(rgb(0xf59e0b))  // ← Styled trait (orange background)
                        .border_4()  // ← Styled trait
                        .border_color(rgb(0xef4444))  // ← Styled trait (red border)
                        .rounded(px(32.0))  // ← Styled trait (large radius)
                        .shadow_xl()  // ← Styled trait
                        .p(px(8.0))  // ← Styled trait (extra padding)
                        .content(
                            div()
                                .p(px(24.0))
                                .text_color(gpui::white())
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(16.0))
                                        .child("This BottomSheet combines multiple Styled trait methods:")
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child("• Orange background (.bg())")
                                                .child("• Thick red border (.border_4() + .border_color())")
                                                .child("• Large border radius (.rounded(32px))")
                                                .child("• Extra large shadow (.shadow_xl())")
                                                .child("• Extra padding (.p(8px))")
                                        )
                                )
                        )
                        .on_close(cx.listener(|view, _, cx| {
                            view.show_combined = false;
                            cx.notify();
                        }))
                )
            })
    }
}
